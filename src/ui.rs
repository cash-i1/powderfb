use crate::graphics::Graphics;
use crate::misc::Color;
use crate::misc::Rectangle;
use crate::particle::particles;
use crate::particle::Particle;
use crate::World;

pub struct Button {
    rect: Rectangle,
    focused: bool,
    focused_color: Color,
    default_color: Color,
    current_color: Color,
    id: String,
    toggled: bool,
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
    pub fn init(&mut self, gfx: &Graphics) {
        if self.buttons.is_none() {
            let mut buttons = vec![];

            // particle buttons
            for (i, particle) in particles().iter().enumerate() {
                let btn = Button {
                    rect: Rectangle {
                        height: 35,
                        width: 35,
                        x: 10 + 50 * i,
                        y: 10,
                    },
                    focused: false,
                    focused_color: Color::Custom((particle.color.raw() & 0xfefefe) >> 1),
                    default_color: particle.color,
                    current_color: particle.color,
                    id: "select_particle".to_string(),
                    toggled: false,
                };
                buttons.push(btn);
            }

            // option buttons
            let diagonals = Button {
                rect: Rectangle {
                    height: 15,
                    width: 15,
                    x: gfx.window.get_size().0 - 20,
                    y: 20,
                },
                focused: false,
                focused_color: Color::Custom(0x2f3030),
                default_color: Color::Custom(0x3f3f3f),
                current_color: Color::Custom(0x3f3f3f),
                id: "toggle_diagonals".to_string(),
                toggled: false,
            };

            buttons.push(diagonals);

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
                        if btn.id == "select_particle" {
                            if gfx.window.get_mouse_down(minifb::MouseButton::Left) {
                                world.selected_particle = Some(particles()[i].clone()).clone();
                            }
                        } else if btn.id == "toggle_diagonals" {
                            if gfx.window.get_mouse_down(minifb::MouseButton::Left) {
                                btn.toggled = !btn.toggled;
                                world.paused = btn.toggled;
                            }
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
                    btn.current_color = btn.focused_color;
                } else {
                    btn.current_color = btn.default_color;
                }
                gfx.rectangle(btn.rect.clone(), btn.current_color);
            }
        }
    }
}
