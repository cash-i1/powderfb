pub struct WindowDimensions {
    pub width: usize,
    pub height: usize,
}

pub struct Rectangle {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub color: u32,
}

#[derive(Default, Clone, Debug)]
pub struct Particle {
    pub color: u32
}
