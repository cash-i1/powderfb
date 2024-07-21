use crate::graphics::Graphics;
use crate::misc::Rectangle;
use crate::particle::Particle;
use crate::particle::PARTICLES;
use crate::World;

pub struct Ui {
    pub focused: bool,
}
impl Ui {
    pub fn new() -> Self {
        Self { focused: false }
    }
    pub fn draw(&mut self, gfx: &mut Graphics, world: &mut World) {
        self.focused = false;

        let mut buttons: Vec<Rectangle> = vec![];

        for (i, particle) in PARTICLES.iter().enumerate() {
            let rect = Rectangle {
                color: particle.color,
                height: 35,
                width: 35,
                x: 10 + 50 * i,
                y: 10,
            };

            buttons.push(rect);
        }

        for (i, rect) in buttons.iter_mut().enumerate() {
            if let Some(mouse_pos) = gfx.window.get_mouse_pos(minifb::MouseMode::Discard) {
                if mouse_pos.0 > rect.x as f32
                    && mouse_pos.0 < rect.x as f32 + rect.width as f32
                    && mouse_pos.1 > rect.y as f32
                    && mouse_pos.1 < rect.y as f32 + rect.height as f32
                {
                    self.focused = true;
                    rect.color = 999999 - rect.color;
                    if gfx.window.get_mouse_down(minifb::MouseButton::Left) {
                        world.selected_particle = Some(PARTICLES[i].clone()).clone();
                    }
                }
            }
        }

        for rect in buttons {
            gfx.rectangle(rect);
        }
    }
}
