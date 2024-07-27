use crate::graphics::Graphics;
use crate::misc::Color;
use crate::misc::Rectangle;
use crate::particle::particles;
use crate::World;

pub mod components;

use components::particle_button::ParticleButton;

pub trait UiObject {
    fn init(&mut self, ui: &mut Ui);
    fn step(&mut self, gfx: &mut Graphics, world: &mut World);
    fn rect(&self) -> Rectangle;
    fn color(&self) -> Color;
    fn focused(&self) -> bool;
}

pub struct Ui {
    pub objects: Vec<Box<dyn UiObject>>,
}
impl Ui {
    pub fn new() -> Self {
        Self {
            objects: vec![],
        }
    }
    pub fn init(&mut self, gfx: &Graphics) {
        for (i, particle) in particles().iter().enumerate() {
            let mut btn = ParticleButton::new(particle, i);
            btn.init(self);
            self.objects.push(btn);
        }
    }
    pub fn step(&mut self, gfx: &mut Graphics, world: &mut World) {
        for obj in &mut self.objects {
            obj.step(gfx, world);
        }
    }
    pub fn draw(&mut self, gfx: &mut Graphics) {
        for obj in &mut self.objects {
            gfx.rectangle(obj.rect(), obj.color());
        }
    }
    pub fn focused(&self) -> bool {
        for obj in &self.objects {
            if obj.focused() {
                return true
            }
        }
        false
    }
}
