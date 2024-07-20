use crate::graphics::Graphics;
use crate::misc::{Particle, Rectangle};

pub struct Ui {
    pub ui_focused: bool,
}
impl Ui {
    pub fn new() -> Self {
        Self { ui_focused: false }
    }
    pub fn draw(&mut self, gfx: &mut Graphics) {
        let particles = vec![
            Particle { color: 11122 },
            Particle { color: 99992 },
            Particle { color: 556600 },
            Particle { color: 22222 },
        ];

        let mut buttons: Vec<Rectangle> = vec![];

        println!("{:?}", self.ui_focused);
        for (i, particle) in particles.iter().enumerate() {
            let rect = Rectangle {
                color: particle.color,
                height: 35,
                width: 35,
                x: 10 + 50 * i,
                y: 10,
            };

            buttons.push(rect);
        }

        for rect in buttons.iter_mut() {
            if let Some(mouse_pos) = gfx.window.get_mouse_pos(minifb::MouseMode::Discard) {
                if mouse_pos.0 > rect.x as f32
                    && mouse_pos.0 < rect.x as f32 + rect.width as f32
                    && mouse_pos.1 > rect.y as f32
                    && mouse_pos.1 < rect.y as f32 + rect.height as f32
                {
                    self.ui_focused = true;
                    rect.color = (999999 - (rect.color) ) as u32;
                } else {
                    self.ui_focused = false;
                }
            }
        }

        for rect in buttons {
            gfx.rectangle(rect);
        }
    }
}
