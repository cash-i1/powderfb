use crate::graphics::Graphics;
use crate::misc::Color;
use crate::misc::Rectangle;
use crate::World;

pub mod components;

pub trait UiObject {
    fn init(&mut self);
    fn rect(&self) -> Rectangle;
    fn color(&self) -> Color;
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
    }
    pub fn step(&mut self, gfx: &mut Graphics, world: &mut World) {}
    pub fn draw(&mut self, gfx: &mut Graphics) {
        for obj in &mut components::button::ChooseParticleButton::objects() {
            gfx.rectangle(obj.rect(), obj.color());
        }
    }
}
