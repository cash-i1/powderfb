use crate::graphics::Graphics;
use crate::misc::Color;
use crate::misc::Rectangle;
use crate::World;

pub mod components;

pub trait UiObject {
    fn init(&mut self) -> Rectangle;
    fn rect(&self) -> Rectangle;
    fn color(&self) -> Color;
    fn render(&self, gfx: &Graphics);
}

pub struct Ui {
    pub focused: bool,
    pub buttons: Option<Vec<Box<dyn UiObject>>>,
}
impl Ui {
    pub fn new() -> Self {
        Self {
            focused: false,
            buttons: None,
        }
    }
    pub fn init(&mut self, gfx: &Graphics) {}
    pub fn step(&mut self, gfx: &mut Graphics, world: &mut World) {}
    pub fn draw(&mut self, gfx: &mut Graphics) {}
}
