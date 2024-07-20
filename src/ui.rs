use crate::graphics::Graphics;
use crate::misc::Rectangle;

pub struct Ui {}
impl Ui {
    pub fn new() -> Self {
        Self {}
    }
    pub fn draw(&mut self, gfx: &mut Graphics) {
        for i in 0..10 {
            let mut rect = Rectangle {
                color: 110011,
                height: 35,
                width: 35,
                x: 10 + 50 * i,
                y: 10,
            };
            if let Some(mouse_pos) = gfx.window.get_mouse_pos(minifb::MouseMode::Discard) {
                if mouse_pos.0 > rect.x as f32 
                && mouse_pos.0 < rect.x as f32 + rect.width as f32 
                && mouse_pos.1 > rect.y as f32 
                && mouse_pos.1 < rect.y as f32 + rect.height as f32 {
                    rect.color = 777777;
                }
            }
            gfx.rectangle(rect);
        }
    }
}
