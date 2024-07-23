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
