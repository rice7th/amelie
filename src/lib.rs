use miniquad::*;
use rgb::*;

#[repr(C)]
struct Vertex {
    pos: [f32; 2],
    color: [f32; 4],
    texcoord: [f32; 2],
    tex_index: i32,
    rot2d: f32
}

struct Stage {
    pipeline: Pipeline,
    bindings: Bindings,
    ctx: Box<dyn RenderingBackend>,
}

impl Stage {
    pub fn new() -> Stage {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        let my_texture = lodepng::decode32(include_bytes!("texture.png")).unwrap();
        let my_texture2 = lodepng::decode32(include_bytes!("texture_2.png")).unwrap();
        let tex  = ctx.new_texture_from_rgba8(my_texture.width as u16, my_texture.height as u16, my_texture.buffer.as_bytes());
        let tex2 = ctx.new_texture_from_rgba8(my_texture2.width as u16, my_texture2.height as u16, my_texture2.buffer.as_bytes());

        #[rustfmt::skip]
        let vertices = [
            // First element
            Vertex { pos : [ -0.2, -0.2 ], color: [1., 0., 0., 1.], texcoord: [0.0, 0.0], tex_index: 0, rot2d: 45.0},
            Vertex { pos : [  0.2, -0.2 ], color: [1., 0., 0., 1.], texcoord: [1.0, 0.0], tex_index: 0, rot2d: 45.0},
            Vertex { pos : [  0.2,  0.2 ], color: [1., 0., 0., 1.], texcoord: [1.0, 1.0], tex_index: 0, rot2d: 45.0},
            Vertex { pos : [  -0.2, 0.2 ], color: [1., 0., 0., 1.], texcoord: [0.0, 1.0], tex_index: 0, rot2d: 45.0},

            // Second element
            Vertex { pos : [ -0.7, -0.7 ], color: [0., 1., 0., 1.], texcoord: [0.0, 0.0], tex_index: 1, rot2d: 10.0 },
            Vertex { pos : [ -0.3, -0.7 ], color: [0., 1., 0., 1.], texcoord: [1.0, 0.0], tex_index: 1, rot2d: 10.0 },
            Vertex { pos : [ -0.3, -0.3 ], color: [0., 1., 0., 1.], texcoord: [1.0, 1.0], tex_index: 1, rot2d: 10.0 },
            Vertex { pos : [ -0.7, -0.3 ], color: [0., 1., 0., 1.], texcoord: [0.0, 1.0], tex_index: 1, rot2d: 10.0 },
        ];
        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vertices),
        );

        let indices: [u16; 12] = [
            0, 1, 2, 2, 3, 0,
            4, 5, 6, 6, 7, 4 
        ];
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
                VertexAttribute::new("rot", VertexFormat::Float1),
            ],
            shader,
            PipelineParams::default(),
        );

        Stage {
            pipeline,
            bindings,
            ctx,
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {}

    fn draw(&mut self) {
        self.ctx.begin_default_pass(Default::default());

        self.ctx.apply_pipeline(&self.pipeline);
        self.ctx.apply_bindings(&self.bindings);
        self.ctx.draw(0, 12, 1);
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
