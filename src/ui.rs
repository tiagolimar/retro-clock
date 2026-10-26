use crate::config::{ShadowLevel, Theme, CANVAS_HEIGHT, CANVAS_WIDTH};
use crate::input::{KeyboardInput, MouseInput};

pub const MENU_WIDTH: i32 = 210;
pub const MENU_OPTION_HEIGHT: i32 = 28;
pub const MENU_OPTION_GAP: i32 = 4;
pub const MENU_OPTION_COUNT: i32 = 6;
pub const MENU_HEIGHT: i32 =
    2 * 10 + MENU_OPTION_COUNT * MENU_OPTION_HEIGHT + (MENU_OPTION_COUNT - 1) * MENU_OPTION_GAP;
pub const THEME_MENU_WIDTH: i32 = 150;
pub const THEME_OPTION_COUNT: i32 = 7;
pub const THEME_MENU_HEIGHT: i32 =
    2 * 10 + THEME_OPTION_COUNT * MENU_OPTION_HEIGHT + (THEME_OPTION_COUNT - 1) * MENU_OPTION_GAP;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuOption {
    Fullscreen,
    Shadows,
    Themes,
    AdvanceGlobalDay,
    RewindGlobalDay,
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
            Self::Amber => "ÂMBAR",
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
    pub shadows_hover: bool,
    pub themes_hover: bool,
    pub advance_global_day_hover: bool,
    pub rewind_global_day_hover: bool,
    pub close_hover: bool,
    pub theme_hovers: [bool; THEME_OPTION_COUNT as usize],
    pub shadow_level: ShadowLevel,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiEvent {
    None,
    ExitRequested,
    ToggleFullscreen,
    AdvanceGlobalDay,
    RewindGlobalDay,
}

pub struct UiController {
    menu: MenuView,
    theme: Theme,
    shadow_level: ShadowLevel,
}

impl UiController {
    pub fn new() -> Self {
        UiController {
            menu: MenuView::default(),
            theme: Theme::default(),
            shadow_level: ShadowLevel::default(),
        }
    }

    pub fn update(&mut self, input: MouseInput, keyboard: KeyboardInput) -> (UiEvent, UiView) {
        let mut event = UiEvent::None;

        if keyboard.next_theme_pressed {
            self.theme = self.theme.next();
        }

        if keyboard.adjust_shadows_pressed {
            self.shadow_level = self.shadow_level.next();
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
        let shadows_hover = self.menu.visible
            && point_in_rect(
                input.x,
                input.y,
                menu_option_rect(self.menu, MenuOption::Shadows),
            );
        let themes_hover = self.menu.visible
            && point_in_rect(
                input.x,
                input.y,
                menu_option_rect(self.menu, MenuOption::Themes),
            );
        let advance_global_day_hover = self.menu.visible
            && point_in_rect(
                input.x,
                input.y,
                menu_option_rect(self.menu, MenuOption::AdvanceGlobalDay),
            );
        let rewind_global_day_hover = self.menu.visible
            && point_in_rect(
                input.x,
                input.y,
                menu_option_rect(self.menu, MenuOption::RewindGlobalDay),
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
                event = UiEvent::ToggleFullscreen;
            } else if shadows_hover {
                self.shadow_level = self.shadow_level.next();
            } else if themes_hover {
                self.menu.themes_visible = true;
            } else if advance_global_day_hover {
                event = UiEvent::AdvanceGlobalDay;
            } else if rewind_global_day_hover {
                event = UiEvent::RewindGlobalDay;
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
                shadows_hover: self.menu.visible && shadows_hover,
                themes_hover: self.menu.visible && themes_hover,
                advance_global_day_hover: self.menu.visible && advance_global_day_hover,
                rewind_global_day_hover: self.menu.visible && rewind_global_day_hover,
                close_hover: self.menu.visible && close_hover,
                theme_hovers,
                shadow_level: self.shadow_level,
            },
        )
    }
}

pub fn menu_option_rect(menu: MenuView, option: MenuOption) -> (i32, i32, i32, i32) {
    let row = match option {
        MenuOption::Fullscreen => 0,
        MenuOption::Shadows => 1,
        MenuOption::Themes => 2,
        MenuOption::AdvanceGlobalDay => 3,
        MenuOption::RewindGlobalDay => 4,
        MenuOption::Close => 5,
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
    use crate::config::{ShadowLevel, Theme};
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
        assert!(
            !view.menu.visible
                || matches!(
                    option,
                    MenuOption::Shadows
                        | MenuOption::Themes
                        | MenuOption::AdvanceGlobalDay
                        | MenuOption::RewindGlobalDay
                )
        );
        event
    }

    #[test]
    fn fullscreen_option_emits_toggle_event() {
        let mut controller = UiController::new();
        let menu = open_menu(&mut controller);
        assert_eq!(
            click_option(&mut controller, menu, MenuOption::Fullscreen),
            UiEvent::ToggleFullscreen
        );
    }

    #[test]
    fn shadows_option_cycles_levels_and_keeps_menu_open() {
        let mut controller = UiController::new();
        let menu = open_menu(&mut controller);
        let (x, y, width, height) = menu_option_rect(menu, MenuOption::Shadows);

        let input = MouseInput {
            x: x + width / 2,
            y: y + height / 2,
            left_pressed: true,
            ..MouseInput::default()
        };
        let (event, view) = controller.update(input, KeyboardInput::default());

        assert_eq!(event, UiEvent::None);
        assert_eq!(view.shadow_level, ShadowLevel::Half);
        assert!(view.menu.visible);

        let (_, view) = controller.update(input, KeyboardInput::default());
        assert_eq!(view.shadow_level, ShadowLevel::None);
        assert!(view.menu.visible);

        let (_, view) = controller.update(input, KeyboardInput::default());
        assert_eq!(view.shadow_level, ShadowLevel::Full);
        assert!(view.menu.visible);
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
            MouseInput {
                ..MouseInput::default()
            },
            KeyboardInput {
                celebration_pressed: false,
                fullscreen_pressed: true,
                next_theme_pressed: true,
                adjust_shadows_pressed: true,
                alert_pressed: false,
            },
        );

        assert_eq!(event, UiEvent::ToggleFullscreen);
        assert_eq!(view.theme, Theme::Amber);
        assert_eq!(view.shadow_level, ShadowLevel::Half);
    }

    #[test]
    fn global_day_options_emit_navigation_events_and_keep_menu_open() {
        let mut controller = UiController::new();
        let menu = open_menu(&mut controller);

        assert_eq!(
            click_option(&mut controller, menu, MenuOption::AdvanceGlobalDay),
            UiEvent::AdvanceGlobalDay
        );
        assert_eq!(
            click_option(&mut controller, menu, MenuOption::RewindGlobalDay),
            UiEvent::RewindGlobalDay
        );
    }
}
