pub mod graphics;
pub mod misc;

use minifb::*;
use graphics::*;


fn main() {
    let mut gfx = Graphics::builder()
        .width(1280)
        .height(720)
        .fps(60)
        .build();

    while gfx.window.is_open() {
        gfx.rectangle((100, 100), (20, 20));
        gfx.pixel((10, 10));
        gfx.update();
    }
}
