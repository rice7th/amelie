use miniquad::*;
use rgb::*;

#[repr(C)]
#[derive(Debug)]
struct Vertex {
    pos: [f32; 2],
    color: [f32; 4],
    texcoord: [f32; 2],
    tex_index: i32,
}

fn col(r: f32, g: f32, b: f32, a: f32) -> RGBA<f32> {
    return RGBA {
        r, g, b, a
    };
}

#[repr(C)]
pub enum Fill {
    Solid(RGBA<f32>),
    Linear {
        col: RGBA<f32>,
        endcol: RGBA<f32>,
        start: [f32; 2],
        end: [f32; 2],
    },
    Radial {
        col: RGBA<f32>,
        endcol: RGBA<f32>,
        start: [f32; 2],
        end: [f32; 2],
    },
    Texture(u32),
}

pub struct Scene {
    last_index: u16,
    vertices: Vec<Vertex>,
    indices: Vec<u16>
}

impl Scene {
    pub fn new() -> Scene {
        Scene { last_index: 0, vertices: vec![], indices: vec![] }
    }

    pub fn get_len_index(&self) -> i32 {
        return self.indices.len() as i32;
    }

    pub fn rect(&mut self, x: f32, y: f32, w: f32, h: f32, fill: Fill) {
        let mut base_col: RGBA<f32> = [0.0, 0.0, 0.0, 1.0].into();
        let text_id = match fill {
            Fill::Solid(col)  => { base_col = col; -1 },
            Fill::Linear {col, ..} => { base_col = col; -2 },
            Fill::Radial {col, ..} => { base_col = col; -3 },
            Fill::Texture(id) => id as i32,
        };

        self.vertices.push(Vertex { pos: [x,     y],     color: base_col.into(), texcoord: [0.0, 1.0], tex_index: text_id }); // Always pos is the top left vertex
        self.vertices.push(Vertex { pos: [x,     y - h], color: base_col.into(), texcoord: [0.0, 0.0], tex_index: text_id });
        self.vertices.push(Vertex { pos: [x + w, y - h], color: base_col.into(), texcoord: [1.0, 0.0], tex_index: text_id });
        self.vertices.push(Vertex { pos: [x + w, y],     color: base_col.into(), texcoord: [1.0, 1.0], tex_index: text_id });

        let indices = [
            0, 1, 2, 2, 3, 0,
        ].into_iter()
            .map(|i| i + self.last_index)
            .for_each(|i| {
                self.indices.push(i);
            });
        self.last_index += 4;
    }
}

struct Stage {
    pipeline: Pipeline,
    bindings: Bindings,
    ctx: Box<dyn RenderingBackend>,
    index: i32,
}

impl Stage {
    pub fn new() -> Stage {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        let my_texture = lodepng::decode32(include_bytes!("texture.png")).unwrap();
        let my_texture2 = lodepng::decode32(include_bytes!("texture_2.png")).unwrap();
        let tex  = ctx.new_texture_from_rgba8(my_texture.width as u16, my_texture.height as u16, my_texture.buffer.as_bytes());
        let tex2 = ctx.new_texture_from_rgba8(my_texture2.width as u16, my_texture2.height as u16, my_texture2.buffer.as_bytes());

        #[rustfmt::skip]
        //let mut vertices = Vec::from([
        //    // First element
        //    Vertex { pos : [ -0.2, -0.2 ], color: [1., 0., 0., 1.], texcoord: [0.0, 0.0], tex_index: 0 },
        //    Vertex { pos : [  0.2, -0.2 ], color: [1., 0., 0., 1.], texcoord: [1.0, 0.0], tex_index: 0 },
        //    Vertex { pos : [  0.2,  0.2 ], color: [1., 0., 0., 1.], texcoord: [1.0, 1.0], tex_index: 0 },
        //    Vertex { pos : [  -0.2, 0.2 ], color: [1., 0., 0., 1.], texcoord: [0.0, 1.0], tex_index: 0 },

        //    // Second element
        //    Vertex { pos : [ -0.7, -0.7 ], color: [0., 1., 0., 1.], texcoord: [0.0, 0.0], tex_index: 1 },
        //    Vertex { pos : [ -0.3, -0.7 ], color: [0., 1., 0., 1.], texcoord: [1.0, 0.0], tex_index: 1 },
        //    Vertex { pos : [ -0.3, -0.3 ], color: [0., 1., 0., 1.], texcoord: [1.0, 1.0], tex_index: 1 },
        //    Vertex { pos : [ -0.7, -0.3 ], color: [0., 1., 0., 1.], texcoord: [0.0, 1.0], tex_index: 1 },
        //]);

        let mut my_scene = Scene::new();
        my_scene.rect(-0.2, -0.2, 0.4, 0.4, Fill::Solid(col(1., 0., 0., 1.)));
        my_scene.rect(-0.7, -0.7, 0.5, 0.2, Fill::Texture(0));
        my_scene.rect(0., 0., 0.2, 0.2, Fill::Solid(col(1., 1., 0., 1.)));

        let index = my_scene.get_len_index();

        let vertices = my_scene.vertices;
        let indices = my_scene.indices;

        //vertices.push(value);
        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vertices),
        );

        //let indices: [u16; 12] = [
        //    0, 1, 2, 2, 3, 0,
        //    4, 5, 6, 6, 7, 4 
        //];
        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices),
        );

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![tex, tex2],
        };

        let shader = ctx
            .new_shader(
                ShaderSource::Glsl {
                    vertex: shader::VERTEX,
                    fragment: shader::FRAGMENT,
                },
                shader::meta(),
            )
            .unwrap();

        let pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("col", VertexFormat::Float4),
                VertexAttribute::new("uv", VertexFormat::Float2),
                VertexAttribute::new("tex_index", VertexFormat::Float1),
            ],
            shader,
            PipelineParams::default(),
        );

        Stage {
            pipeline,
            bindings,
            ctx,
            index,
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {}

    fn draw(&mut self) {
        self.ctx.begin_default_pass(Default::default());

        self.ctx.apply_pipeline(&self.pipeline);
        self.ctx.apply_bindings(&self.bindings);
        self.ctx.draw(0, self.index as i32, 1);
        self.ctx.end_render_pass();

        self.ctx.commit_frame();
    }
}

mod shader {
    use miniquad::*;

    pub const VERTEX: &str = include_str!("vert.glsl");

    pub const FRAGMENT: &str = include_str!("frag.glsl");

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec![String::from("tex"), String::from("tex")],
            uniforms: UniformBlockLayout { uniforms: vec![] },
        }
    }
}

pub struct Config {
    hidpi: bool,
    fullscreen: bool,
    sample_count: i32,
}

pub fn init(title: &str, height: i32, width: i32, resizable: bool, other_config: Option<Config>) {
    let mut conf = conf::Conf::default();
    conf.window_title = title.to_string();
    conf.window_height = height;
    conf.window_width = width;
    conf.window_resizable = resizable;
    match other_config {
        Some(config) => {
            conf.high_dpi = config.hidpi;
            conf.fullscreen = config.fullscreen;
            conf.sample_count = config.sample_count;
        },
        None => {
            conf.high_dpi = bool::default();
            conf.fullscreen = bool::default();
            conf.sample_count = 1;
        }
    }
    conf.platform.apple_gfx_api = conf::AppleGfxApi::OpenGl;

    miniquad::start(conf, move || Box::new(Stage::new()));
}
