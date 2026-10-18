use crate::input::MouseInput;
use crate::config::{CANVAS_HEIGHT, CANVAS_WIDTH};

pub const MENU_WIDTH: i32 = 210;
pub const MENU_OPTION_HEIGHT: i32 = 28;
pub const MENU_OPTION_GAP: i32 = 4;
pub const MENU_HEIGHT: i32 = 2 * 10 + 3 * MENU_OPTION_HEIGHT + 2 * MENU_OPTION_GAP;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuOption {
    Fullscreen,
    Restore,
    Close,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MenuView {
    pub visible: bool,
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct UiView {
    pub menu: MenuView,
    pub fullscreen_hover: bool,
    pub restore_hover: bool,
    pub close_hover: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiEvent {
    None,
    ExitRequested,
    ToggleFullscreen,
    RestoreWindow,
}

pub struct UiController {
    menu: MenuView,
}

impl UiController {
    pub fn new() -> Self {
        UiController {
            menu: MenuView::default(),
        }
    }

    pub fn update(&mut self, input: MouseInput) -> (UiEvent, UiView) {
        if input.right_pressed {
            self.menu.visible = true;
            self.menu.x = (input.x - MENU_WIDTH / 2)
                .clamp(12, CANVAS_WIDTH as i32 - MENU_WIDTH - 12);
            self.menu.y = (input.y - MENU_HEIGHT / 2)
                .clamp(12, CANVAS_HEIGHT as i32 - MENU_HEIGHT - 12);
        }

        let fullscreen_hover = self.menu.visible
            && point_in_rect(input.x, input.y, menu_option_rect(self.menu, MenuOption::Fullscreen));
        let restore_hover = self.menu.visible
            && point_in_rect(input.x, input.y, menu_option_rect(self.menu, MenuOption::Restore));
        let close_hover = self.menu.visible
            && point_in_rect(input.x, input.y, menu_option_rect(self.menu, MenuOption::Close));

        let mut event = UiEvent::None;
        if self.menu.visible && input.left_pressed {
            if fullscreen_hover {
                self.menu.visible = false;
                event = UiEvent::ToggleFullscreen;
            } else if restore_hover {
                self.menu.visible = false;
                event = UiEvent::RestoreWindow;
            } else if close_hover {
                event = UiEvent::ExitRequested;
            } else if !point_in_rect(
                input.x,
                input.y,
                (self.menu.x, self.menu.y, MENU_WIDTH, MENU_HEIGHT),
            ) {
                self.menu.visible = false;
            }
        }

        (
            event,
            UiView {
                menu: self.menu,
                fullscreen_hover: self.menu.visible && fullscreen_hover,
                restore_hover: self.menu.visible && restore_hover,
                close_hover: self.menu.visible && close_hover,
            },
        )
    }
}

pub fn menu_option_rect(menu: MenuView, option: MenuOption) -> (i32, i32, i32, i32) {
    let row = match option {
        MenuOption::Fullscreen => 0,
        MenuOption::Restore => 1,
        MenuOption::Close => 2,
    };
    let y = menu.y + 10 + row * (MENU_OPTION_HEIGHT + MENU_OPTION_GAP);

    (menu.x + 12, y, MENU_WIDTH - 24, MENU_OPTION_HEIGHT)
}

fn point_in_rect(x: i32, y: i32, rect: (i32, i32, i32, i32)) -> bool {
    let (rx, ry, width, height) = rect;
    x >= rx && x < rx + width && y >= ry && y < ry + height
}

#[cfg(test)]
mod tests {
    use super::{menu_option_rect, MenuOption, UiController, UiEvent};
    use crate::input::MouseInput;

    fn open_menu(controller: &mut UiController) -> super::MenuView {
        let (_, view) = controller.update(MouseInput {
            x: 100,
            y: 100,
            right_pressed: true,
            ..MouseInput::default()
        });
        view.menu
    }

    fn click_option(
        controller: &mut UiController,
        menu: super::MenuView,
        option: MenuOption,
    ) -> UiEvent {
        let (x, y, width, height) = menu_option_rect(menu, option);
        let (event, view) = controller.update(MouseInput {
            x: x + width / 2,
            y: y + height / 2,
            left_pressed: true,
            ..MouseInput::default()
        });
        assert!(!view.menu.visible);
        event
    }

    #[test]
    fn fullscreen_and_restore_options_emit_their_events() {
        let mut controller = UiController::new();
        let menu = open_menu(&mut controller);
        assert_eq!(
            click_option(&mut controller, menu, MenuOption::Fullscreen),
            UiEvent::ToggleFullscreen
        );

        let menu = open_menu(&mut controller);
        assert_eq!(
            click_option(&mut controller, menu, MenuOption::Restore),
            UiEvent::RestoreWindow
        );
    }
}

