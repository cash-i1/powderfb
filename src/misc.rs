use rand::Rng;
use std::ops::{self, Add};

pub struct WindowDimensions {
    pub width: usize,
    pub height: usize,
}

#[derive(Clone)]
pub struct Rectangle {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

pub fn error(text: &str) {
    eprint!("error: {text}");
    std::process::exit(1);
}

pub fn warning(text: &str) {
    eprint!("warning: {text}");
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Color {
    #[default]
    None,
    Custom(u32),
}
impl Color {
    pub fn red() -> Self {
        Self::Custom(0xff0000)
    }
    pub fn green() -> Self {
        Self::Custom(0x00ff00)
    }
    pub fn blue() -> Self {
        Self::Custom(0x0000ff)
    }
    pub fn random() -> Self {
        Self::Custom(rand::thread_rng().gen_range(0x000000..0xffffff))
    }
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        let hex_string = format!("{:X}{:X}{:X}", r, g, b);
        Color::Custom(u32::from_str_radix(&hex_string, 16).unwrap())
    }
    pub fn raw(&self) -> u32 {
        match self {
            Self::None => 0,
            Self::Custom(col) => *col,
        }
    }
    pub fn raw_mut(&mut self) -> u32 {
        match self {
            Self::None => 0,
            Self::Custom(col) => *col,
        }
    }
}

impl ops::Sub<Color> for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Self::Output {
        match self {
            Color::None => Color::None,
            Color::Custom(col) => Color::Custom(
                col - match rhs {
                    Color::Custom(col) => col,
                    Color::None => 0,
                },
            ),
        }
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        match self {
            Color::None => Color::None,
            Color::Custom(col) => Color::Custom(
                col + match rhs {
                    Color::Custom(col) => col,
                    Color::None => 0,
                },
            ),
        }
    }
}

impl ops::AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        *self = self.clone() + rhs
    }
}
impl ops::SubAssign<Color> for Color {
    fn sub_assign(&mut self, rhs: Color) {
        *self = self.clone() + rhs
    }
}
