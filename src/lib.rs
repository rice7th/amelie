use miniquad::*;
use rgb::*;

pub mod draw;
use draw::*;

pub mod widget;
use widget::*;

pub mod shapes;

struct Texture {
    pub w: u16,
    pub h: u16,
    pub bitmap: Vec<u8>
}


pub struct Scene {
    last_index: u16,
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
    textures: Vec<Texture>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene { last_index: 0, vertices: vec![], indices: vec![], textures: vec![] }
    }

    pub fn get_len_index(&self) -> i32 {
        return self.indices.len() as i32;
    }

    pub fn compose(&mut self, item: Box<dyn Draw>) {
        let draw_data = item.draw();
        draw_data.0.iter().for_each(|vertex| self.vertices.push(*vertex));
        draw_data.1.iter().for_each(|index|  self.indices.push(*index + self.last_index));
        self.last_index += draw_data.0.len() as u16;
    }

    pub fn texture(&mut self, texture: Vec<u8>, w: u16, h: u16) {
        self.textures.push(Texture { bitmap: texture, w, h })
    }
}

struct Stage {
    pipeline: Pipeline,
    bindings: Bindings,
    ctx: Box<dyn RenderingBackend>,
    index: i32,
}

impl Stage {
    pub fn new(scene: Scene) -> Stage {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        
        
        let mut bitmaps = vec![];
        scene.textures.iter().for_each(|tex| {
            bitmaps.push(ctx.new_texture_from_rgba8(tex.w, tex.h, &tex.bitmap))
        });


        let index = scene.get_len_index();

        let vertices = scene.vertices;
        let indices = scene.indices;

        //vertices.push(value);
        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vertices),
        );

        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices),
        );

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: bitmaps,
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
                VertexAttribute::new("rounding", VertexFormat::Float4),
            ],
            shader,
            PipelineParams {
                color_blend: Some(BlendState::new(
                        Equation::Add,
                        BlendFactor::Value(BlendValue::SourceAlpha),
                        BlendFactor::OneMinusValue(BlendValue::SourceAlpha)
                    )
                ),
                ..Default::default()
            },
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

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        dbg!([x, y]);
    }

    fn draw(&mut self) {
        self.ctx.begin_default_pass(PassAction::clear_color(0.0, 0.0, 0.0, 1.0));

        self.ctx.apply_pipeline(&self.pipeline);
        self.ctx.apply_bindings(&self.bindings);
        self.ctx.apply_uniforms(UniformsSource::table(&shader::Uniforms {
            res: window::screen_size().into(),
        }));
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
            uniforms: UniformBlockLayout { uniforms: vec![UniformDesc::new("res", UniformType::Float2)] },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub res: [f32; 2],
    }
}

pub struct Config {
    hidpi: bool,
    fullscreen: bool,
    sample_count: i32,
}

pub fn init(title: &str, height: i32, width: i32, resizable: bool, other_config: Option<Config>, scene: Scene) {
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

    miniquad::start(conf, move || Box::new(Stage::new(scene)));
}
