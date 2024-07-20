pub mod graphics;
pub mod misc;

use graphics::*;
use minifb::*;
use misc::Rectangle;

fn main() {
    let mut gfx = Graphics::builder().width(1280).height(720).fps(60).build();

    while gfx.window.is_open() {
        if let Some((x, y)) = gfx.window.get_mouse_pos(MouseMode::Discard) {
            if (x as usize) < gfx.dimensions.width && (y as usize) < gfx.dimensions.height {
                let rect = Rectangle {
                    x: x as usize,
                    y: y as usize,
                    height: 10,
                    width: 10,
                    color: x as u32,
                };
                gfx.rectangle(rect);
            }
        }
        gfx.update();
    }
}
