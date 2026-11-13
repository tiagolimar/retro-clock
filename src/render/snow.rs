use crate::config::{SnowLevel, CANVAS_HEIGHT, CANVAS_WIDTH};

use super::canvas::fill_rect;

const SMALL_FLAKE: u32 = 0x00c8d8e6;
const LARGE_FLAKE: u32 = 0x00f3faff;

pub fn draw(buffer: &mut [u32], level: SnowLevel, elapsed_seconds: f64) {
    for index in 0..level.flake_count() {
        let seed = (index as u32)
            .wrapping_mul(1_664_525)
            .wrapping_add(1_013_904_223);
        let start_x = (seed as usize % CANVAS_WIDTH) as f64;
        let start_y = (seed.rotate_left(13) as usize % CANVAS_HEIGHT) as f64;
        let speed = 18.0 + (seed % 25) as f64;
        let phase = ((seed >> 8) % 628) as f64 / 100.0;
        let drift = (4.0 + ((seed >> 16) % 9) as f64) * (elapsed_seconds * 0.8 + phase).sin();
        let x = (start_x + drift).rem_euclid(CANVAS_WIDTH as f64) as i32;
        let y = (start_y + elapsed_seconds * speed).rem_euclid(CANVAS_HEIGHT as f64) as i32;
        let size = if seed % 8 == 0 { 2 } else { 1 };
        let color = if size == 2 { LARGE_FLAKE } else { SMALL_FLAKE };

        fill_rect(buffer, x, y, size, size, color);
    }
}
