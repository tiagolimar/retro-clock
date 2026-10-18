mod glyphs;

use super::canvas::fill_rect;

pub fn draw_text_centered(buffer: &mut [u32], text: &str, y: i32, scale: i32, color: u32) {
    let width = text_width(text, scale);
    draw_text(buffer, text, (super::canvas::WIDTH as i32 - width) / 2, y, scale, color);
}

pub fn draw_text(buffer: &mut [u32], text: &str, x: i32, y: i32, scale: i32, color: u32) {
    let mut cursor_x = x;
    for ch in text.chars() {
        draw_glyph(buffer, ch, cursor_x, y, scale, color);
        cursor_x += 6 * scale;
    }
}

fn text_width(text: &str, scale: i32) -> i32 {
    text.chars().count() as i32 * 6 * scale - scale
}

fn draw_glyph(buffer: &mut [u32], ch: char, x: i32, y: i32, scale: i32, color: u32) {
    for (row, bits) in glyphs::glyph(ch).into_iter().enumerate() {
        for col in 0..5 {
            if bits & (1 << (4 - col)) != 0 {
                fill_rect(buffer, x + col * scale, y + row as i32 * scale, scale, scale, color);
            }
        }
    }
}

