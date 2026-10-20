use super::super::canvas::fill_rect;

use super::super::colors::Palette;

pub fn draw(
    buffer: &mut [u32],
    x: i32,
    y: i32,
    height: i32,
    on: bool,
    palette: Palette,
    inactive_visible: bool,
) {
    if !on && !inactive_visible {
        return;
    }

    let color = if on { palette.accent } else { palette.shadow };
    let size = (height / 10).max(6);
    fill_rect(buffer, x, y + height / 3, size, size, color);
    fill_rect(buffer, x, y + (height * 2) / 3, size, size, color);
}
