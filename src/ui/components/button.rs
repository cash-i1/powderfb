use crate::graphics::Graphics;
use crate::misc::Color;
use crate::misc::Rectangle;
use crate::particle::particles;
use crate::particle::Particle;
use crate::ui::UiObject;
use crate::Ui;
use crate::World;

pub struct ChooseParticleButton {
    pub rect: Rectangle,
    pub focused: bool,
    pub focused_color: Color,
    pub default_color: Color,
    pub toggled: bool,
    pub particle_index: usize,
}

impl ChooseParticleButton {
    pub fn new(particle: &Particle, i: usize) -> Box<dyn UiObject> {
        let btn = ChooseParticleButton {
            rect: Rectangle {
                height: 35,
                width: 35,
                x: 10 + 50 * i,
                y: 10,
            },
            focused: false,
            focused_color: Color::Custom((particle.color.raw() & 0xfefefe) >> 1),
            default_color: particle.color,
            toggled: false,
            particle_index: i,
        };
        Box::new(btn)
    }
}

impl UiObject for ChooseParticleButton {
    fn rect(&self) -> Rectangle {
        self.rect.clone()
    }

    fn color(&self) -> Color {
        if self.focused || self.toggled {
            self.focused_color.clone()
        } else {
            self.default_color.clone()
        }
    }

    fn init(&mut self, ui: &mut Ui) {}

    fn step(&mut self, gfx: &mut Graphics, world: &mut World) {
        self.focused = false;
        if let Some(mouse_pos) = gfx.window.get_mouse_pos(minifb::MouseMode::Discard) {
            if mouse_pos.0 > self.rect.x as f32
                && mouse_pos.0 < self.rect.x as f32 + self.rect.width as f32
                && mouse_pos.1 > self.rect.y as f32
                && mouse_pos.1 < self.rect.y as f32 + self.rect.height as f32
            {
                self.focused = true;
                if gfx.window.get_mouse_down(minifb::MouseButton::Left) {
                    world.selected_particle =
                        Some(particles()[self.particle_index].clone()).clone();
                }
            } else {
                self.focused = false;
            }
        }
    }

    fn focused(&self) -> bool {
        self.focused.clone()
    }
}
