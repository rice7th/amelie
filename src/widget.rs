use crate::draw::*;

pub trait Widget: Draw {
    fn get_data(&self) -> &Base;
    fn set_data(&mut self) -> &mut Base;

    fn on_click(&self, func: dyn FnMut(Box<dyn UserData>));
    fn on_release(&self, func: dyn FnMut(Box<dyn UserData>));
    fn mouse_enter(&self, func: dyn FnMut(Box<dyn UserData>));
    fn mouse_leave(&self, func: dyn FnMut(Box<dyn UserData>));
}

pub struct Base {
    pos:  [f32; 2],
    size: [f32; 2],
    // Might be a problem for future Amelie releases since this inhibits the ability to modify the parent of a widget... Ouch!
    parent: *const dyn Widget
}

impl UserData for Base {
    fn get_data(&self) -> &Base {
        return self;
    }

    fn set_data(&mut self) -> &mut Base {
        return self;
    }
}

pub struct Container {
    child: Vec<Box<dyn Widget>>,
    parent: *const dyn Widget
}

impl Draw for Container {
    fn draw(&self) -> DrawData {
        let mut draw_vert = vec![];
        let mut draw_indx = vec![];
        self.child.iter().for_each(|c| {
            let (vert, indx) = (**c).draw();
            draw_vert.push(vert);
            draw_indx.push(indx);
        });
        return (draw_vert.concat(), draw_indx.concat());
    }
}


pub trait UserData {
    fn get_data(&self) -> &Base;
    fn set_data(&mut self) -> &mut Base;
}