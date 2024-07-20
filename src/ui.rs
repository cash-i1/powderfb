use crate::graphics::Graphics;
use crate::misc::Rectangle;

pub struct Ui {
}
impl Ui {
    pub fn new() -> Self {
        Self { }
    }
    pub fn draw(&mut self, gfx: &mut Graphics) {
        let rect = Rectangle {
            color: 999999,
            height: 100,
            width: 100,
            x: 100,
            y: 100,
        };
        gfx.rectangle(rect);
    }
}

