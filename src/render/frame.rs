use super::canvas::{fill_rect, HEIGHT, WIDTH};
use super::colors::{BLACK, GREEN_DARK};

pub fn draw(buffer: &mut [u32]) {
    fill_rect(buffer, 18, 18, WIDTH as i32 - 36, HEIGHT as i32 - 36, GREEN_DARK);
    fill_rect(buffer, 22, 22, WIDTH as i32 - 44, HEIGHT as i32 - 44, BLACK);
    fill_rect(buffer, 42, 72, WIDTH as i32 - 84, 2, GREEN_DARK);
    fill_rect(buffer, 42, 368, WIDTH as i32 - 84, 2, GREEN_DARK);
    fill_rect(buffer, 42, 494, WIDTH as i32 - 84, 2, GREEN_DARK);
}

