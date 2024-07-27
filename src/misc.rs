use rand::Rng;
use std::{env::Args, ops::{self, Add}, process::Output};

pub struct WindowDimensions {
    pub width: usize,
    pub height: usize,
}

#[derive(Copy, Clone)]
pub struct Position {
    i: usize,
    j: usize,
}
impl Position {
    pub fn i(&self) -> usize {
        self.i
    }
    pub fn j(&self) -> usize {
        self.j
    }

    pub fn i_mut(&mut self) -> &mut usize {
        &mut self.i
    }
    pub fn j_mut(&mut self) -> &mut usize {
        &mut self.j
    }

    pub fn modify<F>(&self, f: F) -> Self 
    where
        F: FnOnce(&mut Self),
    {
        let mut new_self = self.clone();
        f(&mut new_self);
        new_self
    }
}

pub fn pos(x: usize, y: usize) -> Position {
    Position { i: x, j: y }
}
pub fn offset(x: i32, y: i32) -> Direction {
    Direction { x: x, y: y }
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
        Color::Custom(
            u32::from_str_radix(&hex_string, 16)
                .unwrap()
                .clamp(0x000000, 0xffffff),
        )
    }
    pub fn raw(&self) -> u32 {
        match self {
            Self::None => 0,
            Self::Custom(col) => col.clamp(&0x000000, &0xffffff).clone(),
        }
    }
    pub fn raw_mut(&mut self) -> &mut u32 {
        match self {
            Self::None => panic!("cant get mutable reference to an empty color"),
            Self::Custom(col) => col,
        }
    }
    pub fn variate(&mut self, variation: u32) {
        let mut new = self.raw();
        if rand::random::<bool>() {
            new -= variation;
        } else {
            new += variation;
        }

        *self.raw_mut() = new.clamp(0x000000, 0xffffff);
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

// pub enum Direction {
//     Up,
//     Down,
//     Left,
//     Right,
//     Custom(Offset),
// }
//
// impl Direction {
//     pub fn as_custom(&self) -> Self {
//         match self {
//             Self::Up => Self::Custom(offset(0, 1)),
//             Self::Down => Self::Custom(offset(0, -1)),
//             Self::Left => Self::Custom(offset(-1, 0)),
//             Self::Right => Self::Custom(offset(1, 0)),
//             Self::Custom(offset) => Self::Custom(offset.clone()),
//         }
//     }
// }

#[derive(Clone)]
pub struct Direction {
    x: i32,
    y: i32,
}
impl Direction {
    pub fn new(x: i32, y: i32) -> Self {
        Direction { x, y }
    }

    pub fn up() -> Self {
        Direction { x: 0, y: -1 }
    }
    pub fn down() -> Self {
        Direction { x: 0, y: 1 }
    }
    pub fn left() -> Self {
        Direction { x: -1, y: 0 }
    }
    pub fn right() -> Self {
        Direction { x: 1, y: 1 }
    }

    pub fn i(&self) -> i32 {
        self.x
    }
    pub fn j(&self) -> i32 {
        self.y
    }
}
