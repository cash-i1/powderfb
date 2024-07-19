pub mod graphics;
pub mod misc;

use graphics::*;
use minifb::*;

fn main() {
    let mut gfx = Graphics::builder().width(1280).height(720).fps(60).build();

    while gfx.window.is_open() {
        if let Some((x, y)) = gfx.window.get_mouse_pos(MouseMode::Clamp) {
            // gfx.rectangle((x as usize, y as usize), (20, 20));
            if (x as usize) < gfx.dimensions.width && (y as usize) < gfx.dimensions.height {

                gfx.rectangle((x as usize, y as usize), (20, 20));
            }
        }
        gfx.update();
    }
}
