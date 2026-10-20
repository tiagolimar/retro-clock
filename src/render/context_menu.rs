use crate::ui::{
    menu_option_rect, theme_menu_rect, theme_option_rect, MenuOption, ThemeOption, UiView,
    MENU_HEIGHT, MENU_WIDTH,
};

use super::canvas::fill_rect;
use super::colors::Palette;
use super::font;

pub fn draw(buffer: &mut [u32], view: UiView, palette: Palette) {
    draw_panel(
        buffer,
        view.menu.x,
        view.menu.y,
        MENU_WIDTH,
        MENU_HEIGHT,
        palette,
    );

    let options = [
        (
            MenuOption::Fullscreen,
            "Tela inteira",
            view.fullscreen_hover,
        ),
        (MenuOption::Themes, "TEMAS", view.themes_hover),
        (MenuOption::Close, "FECHAR", view.close_hover),
    ];

    for (option, label, hover) in options {
        let (x, y, width, height) = menu_option_rect(view.menu, option);
        draw_option(buffer, x, y, width, height, label, hover, palette);
    }

    if view.menu.themes_visible {
        let (x, y, width, height) = theme_menu_rect(view.menu);
        draw_panel(buffer, x, y, width, height, palette);

        for option in ThemeOption::ALL {
            let (x, y, width, height) = theme_option_rect(view.menu, option);
            draw_option(
                buffer,
                x,
                y,
                width,
                height,
                option.label(),
                view.theme_hovers[option.index()],
                palette,
            );
        }
    }
}

fn draw_panel(buffer: &mut [u32], x: i32, y: i32, width: i32, height: i32, palette: Palette) {
    fill_rect(buffer, x + 3, y + 3, width, height, palette.shadow);
    fill_rect(buffer, x, y, width, height, palette.panel);
    fill_rect(buffer, x, y, width, 2, palette.muted);
    fill_rect(buffer, x, y + height - 2, width, 2, palette.muted);
}

fn draw_option(
    buffer: &mut [u32],
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    label: &str,
    hover: bool,
    palette: Palette,
) {
    if hover {
        fill_rect(buffer, x, y, width, height, palette.accent);
    }

    let color = if hover {
        palette.background
    } else {
        palette.highlight
    };
    font::draw_text(buffer, label, x + 10, y + 6, 2, color);
}
