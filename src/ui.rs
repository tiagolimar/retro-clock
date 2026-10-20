use crate::config::{Theme, CANVAS_HEIGHT, CANVAS_WIDTH};
use crate::input::{KeyboardInput, MouseInput};

pub const MENU_WIDTH: i32 = 210;
pub const MENU_OPTION_HEIGHT: i32 = 28;
pub const MENU_OPTION_GAP: i32 = 4;
pub const MENU_HEIGHT: i32 = 2 * 10 + 3 * MENU_OPTION_HEIGHT + 2 * MENU_OPTION_GAP;
pub const THEME_MENU_WIDTH: i32 = 150;
pub const THEME_OPTION_COUNT: i32 = 7;
pub const THEME_MENU_HEIGHT: i32 =
    2 * 10 + THEME_OPTION_COUNT * MENU_OPTION_HEIGHT + (THEME_OPTION_COUNT - 1) * MENU_OPTION_GAP;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuOption {
    Fullscreen,
    Themes,
    Close,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThemeOption {
    Green,
    Amber,
    Blue,
    Red,
    Violet,
    Cyan,
    White,
}

impl ThemeOption {
    pub const ALL: [Self; THEME_OPTION_COUNT as usize] = [
        Self::Green,
        Self::Amber,
        Self::Blue,
        Self::Red,
        Self::Violet,
        Self::Cyan,
        Self::White,
    ];

    fn theme(self) -> Theme {
        match self {
            Self::Green => Theme::Green,
            Self::Amber => Theme::Amber,
            Self::Blue => Theme::Blue,
            Self::Red => Theme::Red,
            Self::Violet => Theme::Violet,
            Self::Cyan => Theme::Cyan,
            Self::White => Theme::White,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Green => "VERDE",
            Self::Amber => "AMBAR",
            Self::Blue => "AZUL",
            Self::Red => "VERMELHO",
            Self::Violet => "ROXO",
            Self::Cyan => "CIANO",
            Self::White => "BRANCO",
        }
    }

    pub fn index(self) -> usize {
        match self {
            Self::Green => 0,
            Self::Amber => 1,
            Self::Blue => 2,
            Self::Red => 3,
            Self::Violet => 4,
            Self::Cyan => 5,
            Self::White => 6,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MenuView {
    pub visible: bool,
    pub themes_visible: bool,
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct UiView {
    pub menu: MenuView,
    pub theme: Theme,
    pub fullscreen_hover: bool,
    pub themes_hover: bool,
    pub close_hover: bool,
    pub theme_hovers: [bool; THEME_OPTION_COUNT as usize],
    pub transparent_sticks_visible: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiEvent {
    None,
    ExitRequested,
    ToggleFullscreen,
    EnterFullscreen,
}

pub struct UiController {
    menu: MenuView,
    theme: Theme,
    transparent_sticks_visible: bool,
}

impl UiController {
    pub fn new() -> Self {
        UiController {
            menu: MenuView::default(),
            theme: Theme::default(),
            transparent_sticks_visible: true,
        }
    }

    pub fn update(&mut self, input: MouseInput, keyboard: KeyboardInput) -> (UiEvent, UiView) {
        let mut event = UiEvent::None;

        if keyboard.next_theme_pressed {
            self.theme = self.theme.next();
        }

        if keyboard.toggle_sticks_pressed {
            self.transparent_sticks_visible = !self.transparent_sticks_visible;
        }

        if keyboard.fullscreen_pressed {
            self.menu.visible = false;
            self.menu.themes_visible = false;
            event = UiEvent::ToggleFullscreen;
        }

        if input.right_pressed {
            self.menu.visible = true;
            self.menu.themes_visible = false;
            self.menu.x =
                (input.x - MENU_WIDTH / 2).clamp(12, CANVAS_WIDTH as i32 - MENU_WIDTH - 12);
            self.menu.y =
                (input.y - MENU_HEIGHT / 2).clamp(12, CANVAS_HEIGHT as i32 - MENU_HEIGHT - 12);
        }

        let fullscreen_hover = self.menu.visible
            && point_in_rect(
                input.x,
                input.y,
                menu_option_rect(self.menu, MenuOption::Fullscreen),
            );
        let themes_hover = self.menu.visible
            && point_in_rect(
                input.x,
                input.y,
                menu_option_rect(self.menu, MenuOption::Themes),
            );
        let close_hover = self.menu.visible
            && point_in_rect(
                input.x,
                input.y,
                menu_option_rect(self.menu, MenuOption::Close),
            );
        let theme_hovers = ThemeOption::ALL.map(|option| {
            self.menu.themes_visible
                && point_in_rect(input.x, input.y, theme_option_rect(self.menu, option))
        });

        if self.menu.visible && input.left_pressed {
            if let Some(selected_theme) = ThemeOption::ALL
                .into_iter()
                .find(|option| theme_hovers[option.index()])
            {
                self.theme = selected_theme.theme();
            } else if fullscreen_hover {
                self.menu.visible = false;
                self.menu.themes_visible = false;
                event = UiEvent::EnterFullscreen;
            } else if themes_hover {
                self.menu.themes_visible = true;
            } else if close_hover {
                event = UiEvent::ExitRequested;
            } else if !point_in_rect(
                input.x,
                input.y,
                (self.menu.x, self.menu.y, MENU_WIDTH, MENU_HEIGHT),
            ) && !(self.menu.themes_visible
                && point_in_rect(input.x, input.y, theme_menu_rect(self.menu)))
            {
                self.menu.visible = false;
                self.menu.themes_visible = false;
            }
        }

        (
            event,
            UiView {
                menu: self.menu,
                theme: self.theme,
                fullscreen_hover: self.menu.visible && fullscreen_hover,
                themes_hover: self.menu.visible && themes_hover,
                close_hover: self.menu.visible && close_hover,
                theme_hovers,
                transparent_sticks_visible: self.transparent_sticks_visible,
            },
        )
    }
}

pub fn menu_option_rect(menu: MenuView, option: MenuOption) -> (i32, i32, i32, i32) {
    let row = match option {
        MenuOption::Fullscreen => 0,
        MenuOption::Themes => 1,
        MenuOption::Close => 2,
    };
    let y = menu.y + 10 + row * (MENU_OPTION_HEIGHT + MENU_OPTION_GAP);

    (menu.x + 12, y, MENU_WIDTH - 24, MENU_OPTION_HEIGHT)
}

pub fn theme_menu_rect(menu: MenuView) -> (i32, i32, i32, i32) {
    let right_x = menu.x + MENU_WIDTH + 6;
    let left_x = menu.x - THEME_MENU_WIDTH - 6;
    let x = if right_x + THEME_MENU_WIDTH <= CANVAS_WIDTH as i32 - 12 {
        right_x
    } else {
        left_x.max(12)
    };
    let y = (menu.y + 2 * (MENU_OPTION_HEIGHT + MENU_OPTION_GAP))
        .clamp(12, CANVAS_HEIGHT as i32 - THEME_MENU_HEIGHT - 12);

    (x, y, THEME_MENU_WIDTH, THEME_MENU_HEIGHT)
}

pub fn theme_option_rect(menu: MenuView, option: ThemeOption) -> (i32, i32, i32, i32) {
    let row = match option {
        ThemeOption::Green => 0,
        ThemeOption::Amber => 1,
        ThemeOption::Blue => 2,
        ThemeOption::Red => 3,
        ThemeOption::Violet => 4,
        ThemeOption::Cyan => 5,
        ThemeOption::White => 6,
    };
    let (x, y, _, _) = theme_menu_rect(menu);
    let y = y + 10 + row * (MENU_OPTION_HEIGHT + MENU_OPTION_GAP);

    (x + 12, y, THEME_MENU_WIDTH - 24, MENU_OPTION_HEIGHT)
}

fn point_in_rect(x: i32, y: i32, rect: (i32, i32, i32, i32)) -> bool {
    let (rx, ry, width, height) = rect;
    x >= rx && x < rx + width && y >= ry && y < ry + height
}

#[cfg(test)]
mod tests {
    use super::{
        menu_option_rect, theme_option_rect, MenuOption, ThemeOption, UiController, UiEvent,
    };
    use crate::config::Theme;
    use crate::input::{KeyboardInput, MouseInput};

    fn open_menu(controller: &mut UiController) -> super::MenuView {
        let (_, view) = controller.update(
            MouseInput {
                x: 100,
                y: 100,
                right_pressed: true,
                ..MouseInput::default()
            },
            KeyboardInput::default(),
        );
        view.menu
    }

    fn click_option(
        controller: &mut UiController,
        menu: super::MenuView,
        option: MenuOption,
    ) -> UiEvent {
        let (x, y, width, height) = menu_option_rect(menu, option);
        let (event, view) = controller.update(
            MouseInput {
                x: x + width / 2,
                y: y + height / 2,
                left_pressed: true,
                ..MouseInput::default()
            },
            KeyboardInput::default(),
        );
        assert!(!view.menu.visible || option == MenuOption::Themes);
        event
    }

    #[test]
    fn fullscreen_option_emits_enter_event() {
        let mut controller = UiController::new();
        let menu = open_menu(&mut controller);
        assert_eq!(
            click_option(&mut controller, menu, MenuOption::Fullscreen),
            UiEvent::EnterFullscreen
        );
    }

    #[test]
    fn theme_menu_selects_a_palette() {
        let mut controller = UiController::new();
        let menu = open_menu(&mut controller);
        assert_eq!(
            click_option(&mut controller, menu, MenuOption::Themes),
            UiEvent::None
        );

        let (theme_x, theme_y, width, height) = theme_option_rect(menu, ThemeOption::Amber);
        let (_, view) = controller.update(
            MouseInput {
                x: theme_x + width / 2,
                y: theme_y + height / 2,
                left_pressed: true,
                ..MouseInput::default()
            },
            KeyboardInput::default(),
        );

        assert_eq!(view.theme, Theme::Amber);
        assert!(view.menu.visible);
        assert!(view.menu.themes_visible);
    }

    #[test]
    fn keyboard_shortcuts_toggle_fullscreen_and_cycle_themes() {
        let mut controller = UiController::new();
        let (event, view) = controller.update(
            MouseInput::default(),
            KeyboardInput {
                fullscreen_pressed: true,
                next_theme_pressed: true,
                toggle_sticks_pressed: true,
            },
        );

        assert_eq!(event, UiEvent::ToggleFullscreen);
        assert_eq!(view.theme, Theme::Amber);
        assert!(!view.transparent_sticks_visible);
    }
}
