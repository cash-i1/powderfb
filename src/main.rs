pub mod graphics;
pub mod misc;
pub mod particle;
pub mod simulate;
pub mod ui;
pub mod world;

use graphics::*;
use minifb::*;
use misc::Rectangle;
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
        .resizeable(false)
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
                if gfx.window.get_mouse_down(MouseButton::Left) {
                    gfx.window.set_cursor_style(CursorStyle::Crosshair);
                    let pos = (x as usize / cell_size, y as usize / cell_size);

                    if let Some(mut particle) = world.selected_particle.clone() {
                        if particle.properties.randomness != 0 {
                            particle.color.variate(particle.properties.randomness);
                        }
                        world.particles[pos.0][pos.1] = Some(particle);
                    } else {
                        world.particles[pos.0][pos.1] = Some(Particle {
                            color: Color::random(),
                            properties: ParticleProperties::default(),
                        });
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
