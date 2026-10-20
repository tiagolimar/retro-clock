use super::canvas::{fill_rect, HEIGHT, WIDTH};
use super::colors::Palette;

pub fn draw(buffer: &mut [u32], palette: Palette) {
    fill_rect(
        buffer,
        18,
        18,
        WIDTH as i32 - 36,
        HEIGHT as i32 - 36,
        palette.shadow,
    );
    fill_rect(
        buffer,
        22,
        22,
        WIDTH as i32 - 44,
        HEIGHT as i32 - 44,
        palette.background,
    );
    fill_rect(buffer, 42, 368, WIDTH as i32 - 84, 2, palette.shadow);
    fill_rect(buffer, 42, 494, WIDTH as i32 - 84, 2, palette.shadow);
}
