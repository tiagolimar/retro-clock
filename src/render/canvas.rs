use super::colors::BLACK;
use crate::config::{CANVAS_HEIGHT, CANVAS_WIDTH};

pub const WIDTH: usize = CANVAS_WIDTH;
pub const HEIGHT: usize = CANVAS_HEIGHT;

pub fn clear(buffer: &mut [u32]) {
    buffer.fill(BLACK);
}

pub fn fill_rect(buffer: &mut [u32], x: i32, y: i32, width: i32, height: i32, color: u32) {
    let x0 = x.max(0).min(WIDTH as i32) as usize;
    let y0 = y.max(0).min(HEIGHT as i32) as usize;
    let x1 = (x + width).max(0).min(WIDTH as i32) as usize;
    let y1 = (y + height).max(0).min(HEIGHT as i32) as usize;

    if x0 >= x1 || y0 >= y1 {
        return;
    }

    for row in y0..y1 {
        let start = row * WIDTH + x0;
        buffer[start..start + (x1 - x0)].fill(color);
    }
}

