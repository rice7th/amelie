use crate::draw::*;
use rgb::*;

pub struct Frame {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub size: f32,
    pub fill: Fill,
    pub rounding: [f32; 4],
}

impl Draw for Frame {
    fn draw(&self) -> DrawData {
        let mut vertices = vec![];

        let mut base_col: RGBA<f32> = [0.0, 0.0, 0.0, 1.0].into();
        let text_id = match self.fill {
            Fill::Solid(col)  => { base_col = col; -1 },
            Fill::Linear {col, ..} => { base_col = col; -2 },
            Fill::Radial {col, ..} => { base_col = col; -3 },
            Fill::Texture(id) => id as i32,
        };

        // External quad
        vertices.push(Vertex { pos: [self.x,          self.y],          color: base_col.into(), texcoord: [0.0, 1.0], tex_index: text_id, rounding: self.rounding }); // Always pos is the top left vertex
        vertices.push(Vertex { pos: [self.x,          self.y - self.h], color: base_col.into(), texcoord: [0.0, 0.0], tex_index: text_id, rounding: self.rounding });
        vertices.push(Vertex { pos: [self.x + self.w, self.y - self.h], color: base_col.into(), texcoord: [1.0, 0.0], tex_index: text_id, rounding: self.rounding });
        vertices.push(Vertex { pos: [self.x + self.w, self.y],          color: base_col.into(), texcoord: [1.0, 1.0], tex_index: text_id, rounding: self.rounding });

        // Internal negative quad
        vertices.push(Vertex { pos: [self.x + self.size,          self.y - self.size],          color: base_col.into(), texcoord: [self.size / self.h,            (self.w - self.size) / self.w], tex_index: text_id, rounding: self.rounding });
        vertices.push(Vertex { pos: [self.x + self.size,          self.y - self.h + self.size], color: base_col.into(), texcoord: [self.size / self.h,            self.size / self.w           ], tex_index: text_id, rounding: self.rounding });
        vertices.push(Vertex { pos: [self.x + self.w - self.size, self.y - self.h + self.size], color: base_col.into(), texcoord: [(self.h - self.size) / self.h, self.size / self.w           ], tex_index: text_id, rounding: self.rounding });
        vertices.push(Vertex { pos: [self.x + self.w - self.size, self.y - self.size],          color: base_col.into(), texcoord: [(self.h - self.size) / self.h, (self.w - self.size) / self.w], tex_index: text_id, rounding: self.rounding });

        return (vertices, vec![0, 5, 4,
                               1, 5, 0,
                               1, 6, 5,
                               1, 2, 6,
                               2, 7, 6,
                               2, 3, 7,
                               3, 0, 4,
                               3, 4, 7])
    }
}