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


Other features:

* Rounded Rects
Just use a rounded rect sdf as a mask.
The mask in question shall be roundable
on each vertex independently.

* Text
Use Cosmic-text to render text into a 
texture and load it in a specific quad

* Gradients
Use texture indexes -1, -2 etc. and
just 2 point coordinates. Radial and
Linear shall be supported.

* Stokes
Strokes are exponentially harder to do
than anything else. The approach I chose
requires a new object called a "Frame".
A Frame is a simple rectangle with a smaller
rectangle cutout inside, just like this:

+----------+
| +------+ |
| |      | |
| +------+ |
+----------+

Where each + is a vertex. The most outside
vertices get normal uv textures, while the
innermost vertices are mixed, in order to
preserve the normal uv values at those points.
A Frame, on top of position, length and width,
also has a "size" parameter, rendering it
effectively just a "freestanding" stroke, whilst
maintaining all parameters and capabilities of
a normal retangle, such as gradients, textures,
masking and more.
This enables fast non rounded corners and kinda
fast rounded corners, textures and thus gradients.
On top of that, the frame approach completely
solves the stroke position problem.


UNITS

Measurements units should be expressed via
wrapped floats or ints.
I.E.

enum Measurements {
    Pixel(int),
    Percent(float),
    // e.g.
}

fn px(pxl: int) -> float {
    // Some magic happening here
}
*/


fn main() {
    amelie::init("Example", 400, 600, true, None);
}
