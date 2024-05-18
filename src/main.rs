mod lib;

/* General API overview of Amelie

Amelie needs to be:
    - Easy
    - Intuitive
    - No-boilerplate
    - VERY customizable
    - NOT a pain in the ass
    - Animation centric

Idea: widgets don't exist really

Layouts and idk Buttons for example all just implement
the trait widget.

trait Widget {
    fn tesselize(&self) -> Vec<Vertex> {
        // Code to tesselize
    }
}

Then we calculate each position based on the parent.

the Base "widget" (its more of a base struct) provides
a basic interface:

struct Base {
    pos: [f32; 2];
}

As an example:

Window -> Parent

struct AlignH<'w, W: Widget> {
    children: Vec<W>,
    base: Base,
    parent: &'w mut W
}

impl<'w, W: Widget> Widget for AlignH<'w, W> {
    let mut vertices = vec![];
    fn draw(&self) -> Vec<Vertex> {
        for child in children {
            child.pos(self)
            vertices.push(child.draw());
        }
    }
}

*/


fn main() {
    lib::init("Example", 400, 600, true, None);
}
