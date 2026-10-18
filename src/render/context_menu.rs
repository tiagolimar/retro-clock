use crate::ui::{MenuView, MENU_HEIGHT, MENU_WIDTH};

use super::canvas::fill_rect;
use super::colors::{BLACK, GREEN, GREEN_BRIGHT, GREEN_DARK, GREEN_DIM, PANEL};
use super::font;

pub fn draw(buffer: &mut [u32], menu: MenuView, close_hover: bool) {
    fill_rect(buffer, menu.x + 3, menu.y + 3, MENU_WIDTH, MENU_HEIGHT, GREEN_DARK);
    fill_rect(buffer, menu.x, menu.y, MENU_WIDTH, MENU_HEIGHT, PANEL);
    fill_rect(buffer, menu.x, menu.y, MENU_WIDTH, 2, GREEN_DIM);
    fill_rect(buffer, menu.x, menu.y + MENU_HEIGHT - 2, MENU_WIDTH, 2, GREEN_DIM);

    font::draw_text(buffer, "MENU", menu.x + 14, menu.y + 10, 2, GREEN_BRIGHT);
    if close_hover {
        fill_rect(buffer, menu.x + 12, menu.y + 31, MENU_WIDTH - 24, 32, GREEN);
    }

    let option_color = if close_hover { BLACK } else { GREEN_BRIGHT };
    font::draw_text(buffer, "FECHAR", menu.x + 28, menu.y + 40, 3, option_color);
}

