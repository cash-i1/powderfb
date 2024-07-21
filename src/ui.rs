use crate::graphics::Graphics;
use crate::misc::Rectangle;
use crate::particle::particles;
use crate::particle::Particle;
use crate::World;

pub struct Button {
    rect: Rectangle,
    focused: bool,
    focused_color: u32,
    default_color: u32,
}

pub struct Ui {
    pub focused: bool,
    pub buttons: Option<Vec<Button>>,
}
impl Ui {
    pub fn new() -> Self {
        Self {
            focused: false,
            buttons: None,
        }
    }
    pub fn init(&mut self) {
        if self.buttons.is_none() {
            let mut buttons = vec![];
            for (i, particle) in particles().iter().enumerate() {
                let btn = Button {
                    rect: Rectangle {
                        color: particle.color,
                        height: 35,
                        width: 35,
                        x: 10 + 50 * i,
                        y: 10,
                    },
                    focused: false,
                    focused_color: particle.color >> 1,
                    default_color: particle.color,
                };
                buttons.push(btn);
            }
            self.buttons = Some(buttons);
        }
    }
    pub fn step(&mut self, gfx: &mut Graphics, world: &mut World) {
        self.focused = false;

        // update buttons
        if let Some(buttons) = &mut self.buttons {
            for (i, btn) in buttons.iter_mut().enumerate() {
                if let Some(mouse_pos) = gfx.window.get_mouse_pos(minifb::MouseMode::Discard) {
                    if mouse_pos.0 > btn.rect.x as f32
                        && mouse_pos.0 < btn.rect.x as f32 + btn.rect.width as f32
                        && mouse_pos.1 > btn.rect.y as f32
                        && mouse_pos.1 < btn.rect.y as f32 + btn.rect.height as f32
                    {
                        self.focused = true;
                        btn.focused = true;
                        if gfx.window.get_mouse_down(minifb::MouseButton::Left) {
                            world.selected_particle = Some(particles()[i].clone()).clone();
                        }
                    } else {
                        btn.focused = false;
                    }
                }
            }
        }
    }
    pub fn draw(&mut self, gfx: &mut Graphics) {
        if let Some(buttons) = &mut self.buttons {
            for btn in buttons {
                if btn.focused {
                    btn.rect.color = btn.focused_color;
                } else {
                    btn.rect.color = btn.default_color;
                }
                gfx.rectangle(btn.rect.clone());
            }
        }
    }
}
