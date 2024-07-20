pub mod graphics;
pub mod misc;
pub mod world;

use graphics::*;
use minifb::*;
use misc::Rectangle;
use world::*;

fn main() {
    let screen_width = 1280;
    let screen_height = 720;
    let cell_size = 10;

    let mut gfx = Graphics::builder()
        .width(screen_width)
        .height(screen_height)
        .fps(60)
        .build();

    let mut world = World::new(
        screen_width / cell_size,
        screen_height / cell_size,
        cell_size,
        cell_size,
    );

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

        for (x, i) in world.particles.iter().enumerate() {
            for (y, particle) in i.iter().enumerate() {
                let rect = Rectangle {
                    x: world.cell_width * x,
                    y: world.cell_height * y,
                    width: world.cell_width,
                    height: world.cell_height,
                    color:  rand::random::<u32>(),
                };
                gfx.rectangle(rect);
            }
        }
        gfx.update();
    }
}
