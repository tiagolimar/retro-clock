use crate::ui::{menu_option_rect, MenuOption, MenuView, MENU_HEIGHT, MENU_WIDTH};

use super::canvas::fill_rect;
use super::colors::{BLACK, GREEN, GREEN_BRIGHT, GREEN_DARK, GREEN_DIM, PANEL};
use super::font;

pub fn draw(
    buffer: &mut [u32],
    menu: MenuView,
    fullscreen_hover: bool,
    restore_hover: bool,
    close_hover: bool,
) {
    fill_rect(buffer, menu.x + 3, menu.y + 3, MENU_WIDTH, MENU_HEIGHT, GREEN_DARK);
    fill_rect(buffer, menu.x, menu.y, MENU_WIDTH, MENU_HEIGHT, PANEL);
    fill_rect(buffer, menu.x, menu.y, MENU_WIDTH, 2, GREEN_DIM);
    fill_rect(buffer, menu.x, menu.y + MENU_HEIGHT - 2, MENU_WIDTH, 2, GREEN_DIM);

    let options = [
        (MenuOption::Fullscreen, "Tela inteira", fullscreen_hover),
        (MenuOption::Restore, "Restaurar", restore_hover),
        (MenuOption::Close, "FECHAR", close_hover),
    ];

    for (option, label, hover) in options {
        let (x, y, width, height) = menu_option_rect(menu, option);
        if hover {
            fill_rect(buffer, x, y, width, height, GREEN);
        }

        let color = if hover { BLACK } else { GREEN_BRIGHT };
        font::draw_text(buffer, label, x + 10, y + 6, 2, color);
    }
}

