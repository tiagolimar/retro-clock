use super::super::canvas::fill_rect;

const INACTIVE_SEGMENT: u32 = 0x00152b15;
const ACTIVE_SEPARATOR: u32 = 0x0000ff66;

pub fn draw(buffer: &mut [u32], x: i32, y: i32, height: i32, on: bool) {
    let color = if on { ACTIVE_SEPARATOR } else { INACTIVE_SEGMENT };
    let size = (height / 10).max(6);
    fill_rect(buffer, x, y + height / 3, size, size, color);
    fill_rect(buffer, x, y + (height * 2) / 3, size, size, color);
}

