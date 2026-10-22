use super::super::canvas::fill_rect;

use super::super::colors::Palette;

pub fn draw(
    buffer: &mut [u32],
    x: i32,
    y: i32,
    height: i32,
    on: bool,
    palette: Palette,
    inactive_color: Option<u32>,
) {
    let Some(color) = (if on {
        Some(palette.accent)
    } else {
        inactive_color
    }) else {
        return;
    };

    let size = (height / 10).max(6);
    fill_rect(buffer, x, y + height / 3, size, size, color);
    fill_rect(buffer, x, y + (height * 2) / 3, size, size, color);
}
