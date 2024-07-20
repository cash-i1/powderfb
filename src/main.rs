pub mod graphics;
pub mod misc;
pub mod world;

use graphics::*;
use minifb::*;
use misc::{Particle, Rectangle};
use world::*;

fn main() {
    let screen_width = 1280;
    let screen_height = 720;
    let cell_size = 10;

    let mut gfx = Graphics::builder()
        .width(screen_width)
        .height(screen_height)
        .fps(60)
        .resizeable(true)
        .build();

    let mut world = World::new(
        screen_width / cell_size,
        screen_height / cell_size,
        cell_size,
        cell_size,
    );

    while gfx.window.is_open() {
        if let Some((x, y)) = gfx.window.get_mouse_pos(MouseMode::Discard) {
            if gfx.window.get_mouse_down(MouseButton::Left) {
                if (x as usize) < gfx.dimensions.width && (y as usize) < gfx.dimensions.height {

                    let pos = (x as usize / cell_size, y as usize / cell_size);
                    world.particles[pos.0][pos.1] = Some(Particle { color: rand::random::<u32>()});
                }
            }
        }

        for (x, i) in world.particles.iter().enumerate() {
            for (y, particle) in i.iter().enumerate() {
                if let Some(p) = particle {
                    let rect = Rectangle {
                        x: world.cell_width * x,
                        y: world.cell_height * y,
                        width: world.cell_width,
                        height: world.cell_height,
                        color: p.color,
                    };
                    gfx.rectangle(rect);
                }
            }
        }
        gfx.update();
    }
}
