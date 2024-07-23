use crate::misc::Color;
use crate::misc::Rectangle;

pub struct Button {
    rect: Rectangle,
    focused: bool,
    focused_color: Color,
    default_color: Color,
    current_color: Color,
    id: String,
    toggled: bool,
}


