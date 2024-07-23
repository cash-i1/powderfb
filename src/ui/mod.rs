use crate::graphics::Graphics;
use crate::misc::Color;
use crate::misc::Rectangle;
use crate::particle::particles;
use crate::World;

pub mod components;

use components::button::ChooseParticleButton;

pub trait UiObject {
    fn init(&mut self, ui: &mut Ui);
    fn step(&mut self, gfx: &mut Graphics, world: &mut World);
    fn rect(&self) -> Rectangle;
    fn color(&self) -> Color;
    fn focused(&self) -> bool;
}

pub struct Ui {
    pub focused: bool,
    pub objects: Vec<Box<dyn UiObject>>,
}
impl Ui {
    pub fn new() -> Self {
        Self {
            focused: false,
            objects: vec![],
        }
    }
    pub fn init(&mut self, gfx: &Graphics) {
        for (i, particle) in particles().iter().enumerate() {
            let mut btn = ChooseParticleButton::new(particle, i);
            btn.init(self);
            self.objects.push(btn);
        }
    }
    pub fn step(&mut self, gfx: &mut Graphics, world: &mut World) {
        for obj in &mut self.objects {
            obj.step(gfx, world);
            self.focused = obj.focused();
        }
    }
    pub fn draw(&mut self, gfx: &mut Graphics) {
        for obj in &mut self.objects {
            gfx.rectangle(obj.rect(), obj.color());
        }
    }
}
