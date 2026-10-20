#[derive(Clone, Copy, Debug, Default)]
pub struct MouseInput {
    pub x: i32,
    pub y: i32,
    pub right_pressed: bool,
    pub left_pressed: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct KeyboardInput {
    pub fullscreen_pressed: bool,
    pub next_theme_pressed: bool,
    pub toggle_sticks_pressed: bool,
}
