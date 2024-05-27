use rgb::*;

pub type DrawData = (Vec<Vertex>, Vec<u16>);
pub trait Draw {
    fn draw(&self) -> DrawData;
}


#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub pos: [f32; 2],
    pub color: [f32; 4],
    pub texcoord: [f32; 2],
    pub tex_index: i32,
}

pub fn col(r: f32, g: f32, b: f32, a: f32) -> RGBA<f32> {
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