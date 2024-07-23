use crate::misc::Color;
use crate::misc::Rectangle;
use crate::ui::UiObject;
use crate::graphics::Graphics;
use crate::particle::particles;
use crate::particle::Particle;
use crate::World;

pub struct ChooseParticleButton {
    pub rect: Rectangle,
    pub focused: bool,
    pub focused_color: Color,
    pub default_color: Color,
    pub toggled: bool,
}

impl ChooseParticleButton {
    pub fn objects() -> Vec<Box<dyn UiObject>> {
        let mut buttons = vec![];
        for (i, particle) in particles().iter().enumerate() {
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
            };
            buttons.push(Box::new(btn) as Box<dyn UiObject>);
        }
        buttons
    }
}

impl UiObject for ChooseParticleButton {
    fn rect(&self) -> Rectangle {
        self.rect.clone()
    }

    fn color(&self) -> Color {
        if self.focused {
            self.focused_color.clone()
        } else {
            self.default_color.clone()
        }
    }

    fn init(&mut self) {
        todo!()
    }

}
