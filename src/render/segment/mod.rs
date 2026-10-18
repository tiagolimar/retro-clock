mod digit;
mod separator;

use super::canvas::WIDTH;

const GREEN: u32 = 0x0000ff66;
const GREEN_BRIGHT: u32 = 0x0066ff99;

pub fn draw_large_time(
    buffer: &mut [u32],
    hour: u32,
    minute: u32,
    second: u32,
    y: i32,
    height: i32,
) {
    let digits = [
        hour / 10,
        hour % 10,
        minute / 10,
        minute % 10,
        second / 10,
        second % 10,
    ];
    let digit_width = 92;
    let digit_step = 112;
    let colon_step = 40;
    let total_width = digit_step * 6 + colon_step * 2;
    let mut x = (WIDTH as i32 - total_width) / 2;
    let colon_on = second % 2 == 0;

    for (index, digit) in digits.into_iter().enumerate() {
        digit::draw(buffer, x, y, digit_width, height, digit as usize, GREEN);
        x += digit_step;

        if index == 1 || index == 3 {
            separator::draw(buffer, x - 5, y, height, colon_on);
            x += colon_step;
        }
    }
}

pub fn draw_countdown(buffer: &mut [u32], total_seconds: u32, y: i32) {
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    let digits = [hours / 10, hours % 10, minutes / 10, minutes % 10, seconds / 10, seconds % 10];

    let digit_width = 40;
    let digit_height = 78;
    let digit_step = 52;
    let colon_step = 24;
    let total_width = digit_step * 6 + colon_step * 2;
    let mut x = (WIDTH as i32 - total_width) / 2;

    for (index, digit) in digits.into_iter().enumerate() {
        digit::draw(buffer, x, y, digit_width, digit_height, digit as usize, GREEN_BRIGHT);
        x += digit_step;

        if index == 1 || index == 3 {
            separator::draw(buffer, x - 5, y, digit_height, true);
            x += colon_step;
        }
    }
}

