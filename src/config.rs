pub const CANVAS_WIDTH: usize = 960;
pub const CANVAS_HEIGHT: usize = 560;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Theme {
    #[default]
    Green,
    Amber,
    Blue,
    Red,
    Violet,
    Cyan,
    White,
}

impl Theme {
    pub fn next(self) -> Self {
        match self {
            Self::Green => Self::Amber,
            Self::Amber => Self::Blue,
            Self::Blue => Self::Red,
            Self::Red => Self::Violet,
            Self::Violet => Self::Cyan,
            Self::Cyan => Self::White,
            Self::White => Self::Green,
        }
    }
}
