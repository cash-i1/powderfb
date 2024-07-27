pub mod graphics;
pub mod misc;
pub mod particle;
pub mod simulate;
pub mod ui;
pub mod world;

use graphics::*;
use minifb::*;
use misc::{pos, Position, Rectangle};
use particle::{Particle, ParticleProperties};
use rand::Rng;
use ui::*;
use world::*;

use crate::misc::Color;

fn main() {
    let screen_width = 1280;
    let screen_height = 720;
    let cell_size = 20;

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

    let mut ui = Ui::new();
    ui.init(&gfx);

    while gfx.window.is_open() {
        if !ui.focused() {
            if let Some((x, y)) = gfx.window.get_mouse_pos(MouseMode::Discard) {
                let pos = pos(x as usize / cell_size, y as usize / cell_size);
                if gfx.window.get_mouse_down(MouseButton::Left) {
                    gfx.window.set_cursor_style(CursorStyle::Crosshair);

                    if let Some(mut particle) = world.selected_particle.clone() {
                        if particle.properties.randomness != 0 {
                            particle.color.variate(particle.properties.randomness);
                        }
                        world.set(pos, particle);
                    }
                }
                if gfx.window.get_mouse_down(MouseButton::Right) {
                    gfx.window.set_cursor_style(CursorStyle::Crosshair);

                    if world.take(pos).is_some() {
                        world.remove(pos)
                    }
                } else {
                    gfx.window.set_cursor_style(CursorStyle::Arrow);
                }
            }
        }

        {
            world.step();
            world.draw(&mut gfx);
            ui.step(&mut gfx, &mut world);
            ui.draw(&mut gfx);
            gfx.update();
        }
    }
}
