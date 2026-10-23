use crate::app::EndOfCycleAnimation;
use crate::config::CANVAS_WIDTH;

use super::colors::Palette;
use super::font;

const COUNTDOWN_BAND_Y: i32 = 410;
const COUNTDOWN_BAND_HEIGHT: i32 = 78;
const TEXT_SCALE: i32 = 10;
const TEXT_HEIGHT: i32 = 7 * TEXT_SCALE;

pub fn draw(buffer: &mut [u32], animation: EndOfCycleAnimation, palette: Palette) {
    let text_width = font::text_width(animation.text, TEXT_SCALE) + 1;
    let start_x = CANVAS_WIDTH as i32;
    let end_x = -text_width;
    let progress = animation.progress.clamp(0.0, 1.0);
    let x = start_x + ((end_x - start_x) as f32 * progress) as i32;
    let y = COUNTDOWN_BAND_Y + (COUNTDOWN_BAND_HEIGHT - TEXT_HEIGHT) / 2;

    font::draw_text_bold(buffer, animation.text, x, y, TEXT_SCALE, palette.accent);
}
