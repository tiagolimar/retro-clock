pub const CANVAS_WIDTH: usize = 960;
pub const CANVAS_HEIGHT: usize = 560;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ShadowLevel {
    #[default]
    Full,
    Half,
    None,
}

impl ShadowLevel {
    pub fn next(self) -> Self {
        match self {
            Self::Full => Self::Half,
            Self::Half => Self::None,
            Self::None => Self::Full,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SnowLevel {
    Off,
    Light,
    #[default]
    Medium,
    Heavy,
}

impl SnowLevel {
    pub fn next(self) -> Self {
        match self {
            Self::Off => Self::Light,
            Self::Light => Self::Medium,
            Self::Medium => Self::Heavy,
            Self::Heavy => Self::Off,
        }
    }

    pub fn flake_count(self) -> usize {
        match self {
            Self::Off => 0,
            Self::Light => 45,
            Self::Medium => 210,
            Self::Heavy => 800,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Off => "Neve: desligada",
            Self::Light => "Neve: pouca",
            Self::Medium => "Neve: média",
            Self::Heavy => "Neve: muita",
        }
    }
}

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
