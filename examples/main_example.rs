use amelie::draw::*;
use amelie::shapes::*;
use rgb::*;
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

    let mut my_scene = amelie::Scene::new();

    let my_texture = lodepng::decode32(include_bytes!("texture.png")).unwrap();
    let my_texture2 = lodepng::decode32(include_bytes!("texture_2.png")).unwrap();

    // Best scenario would be to just like express a Fill::texture(my_texture) and boom it works.
    // TBD!

    my_scene.texture(my_texture.buffer.as_bytes().to_vec(), my_texture.width   as u16, my_texture.height  as u16);
    my_scene.texture(my_texture2.buffer.as_bytes().to_vec(), my_texture2.width as u16, my_texture2.height as u16);

    let myrect1 = Box::new(quad::Quad {x: -1.0, y: 1.0, w: 1.4, h: 1.4, fill: Fill::Solid(col(1., 0., 0., 0.4)), rounding: [0.40, 0.40, 0.40, 0.40] });
    let myrect2 = Box::new(quad::Quad {x: -0.7, y: -0.7, w: 0.5, h: 0.2, fill: Fill::Texture(0), rounding: [0.2, 0.6, 0.7, 0.2] });
    //let myrect3 = Box::new(quad::Quad {x: 0.,   y: 0.,   w: 0.2, h: 0.2, fill: Fill::Solid(col(1., 1., 0., 1.)), rounding: [0.6, 0.0, 0.0, 0.6] });
    let myrect3 = Box::new(frame::Frame {
        x: 0.,
        y: 0.,
        w: 0.4,
        h: 0.4,
        size: 0.1,
        fill: Fill::Solid(col(1., 0., 1., 0.5)),
        rounding: [0.5, 0.0, 0.0, 0.0]
    });

    
    my_scene.compose(myrect1);
    //my_scene.compose(myrect2);
    my_scene.compose(myrect3);

    amelie::init("Example", 400, 600, true, None, my_scene);
}
