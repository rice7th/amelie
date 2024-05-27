use crate::draw::*;
use rgb::*;

pub struct Quad {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub fill: Fill,
}

impl Draw for Quad {
    fn draw(&self) -> DrawData {
        let mut vertices = vec![];

        let mut base_col: RGBA<f32> = [0.0, 0.0, 0.0, 1.0].into();
        let text_id = match self.fill {
            Fill::Solid(col)  => { base_col = col; -1 },
            Fill::Linear {col, ..} => { base_col = col; -2 },
            Fill::Radial {col, ..} => { base_col = col; -3 },
            Fill::Texture(id) => id as i32,
        };

        vertices.push(Vertex { pos: [self.x,          self.y],     color: base_col.into(), texcoord: [0.0, 1.0], tex_index: text_id }); // Always pos is the top left vertex
        vertices.push(Vertex { pos: [self.x,          self.y - self.h], color: base_col.into(), texcoord: [0.0, 0.0], tex_index: text_id });
        vertices.push(Vertex { pos: [self.x + self.w, self.y - self.h], color: base_col.into(), texcoord: [1.0, 0.0], tex_index: text_id });
        vertices.push(Vertex { pos: [self.x + self.w, self.y],     color: base_col.into(), texcoord: [1.0, 1.0], tex_index: text_id });

        return (vertices, vec![0, 1, 2, 2, 3, 0])
    }
}