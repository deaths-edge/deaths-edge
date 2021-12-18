use bevy::math::Vec2;

#[derive(Debug)]
pub struct SelectClick {
    pub mouse_position: Vec2,
}

#[derive(PartialEq)]
pub enum MouseRightElementState {
    Pressed,
    Released,
}

impl Default for MouseRightElementState {
    fn default() -> Self {
        Self::Released
    }
}
