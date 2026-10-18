use crate::input::MouseInput;
use crate::config::{CANVAS_HEIGHT, CANVAS_WIDTH};

pub const MENU_WIDTH: i32 = 210;
pub const MENU_HEIGHT: i32 = 74;

#[derive(Clone, Copy, Debug, Default)]
pub struct MenuView {
    pub visible: bool,
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct UiView {
    pub menu: MenuView,
    pub close_hover: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiEvent {
    None,
    ExitRequested,
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

        let close_hover = self.menu.visible
            && point_in_rect(
                input.x,
                input.y,
                (self.menu.x + 12, self.menu.y + 31, MENU_WIDTH - 24, 32),
            );

        let event = if self.menu.visible && input.left_pressed {
            if close_hover {
                UiEvent::ExitRequested
            } else if !point_in_rect(
                input.x,
                input.y,
                (self.menu.x, self.menu.y, MENU_WIDTH, MENU_HEIGHT),
            ) {
                self.menu.visible = false;
                UiEvent::None
            } else {
                UiEvent::None
            }
        } else {
            UiEvent::None
        };

        (
            event,
            UiView {
                menu: self.menu,
                close_hover,
            },
        )
    }
}

fn point_in_rect(x: i32, y: i32, rect: (i32, i32, i32, i32)) -> bool {
    let (rx, ry, width, height) = rect;
    x >= rx && x < rx + width && y >= ry && y < ry + height
}

