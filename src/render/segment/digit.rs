use super::super::canvas::fill_rect;

const SEGMENTS: [[bool; 7]; 10] = [
    [true, true, true, true, true, true, false],
    [false, true, true, false, false, false, false],
    [true, true, false, true, true, false, true],
    [true, true, true, true, false, false, true],
    [false, true, true, false, false, true, true],
    [true, false, true, true, false, true, true],
    [true, false, true, true, true, true, true],
    [true, true, true, false, false, false, false],
    [true, true, true, true, true, true, true],
    [true, true, true, true, false, true, true],
];

pub fn draw(
    buffer: &mut [u32],
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    digit: usize,
    color: u32,
    inactive_color: u32,
    inactive_visible: bool,
) {
    let thickness = (width / 6).max(5);
    let segments = SEGMENTS[digit.min(9)];

    for (index, active) in segments.into_iter().enumerate() {
        if active || inactive_visible {
            draw_segment(
                buffer,
                x,
                y,
                width,
                height,
                thickness,
                index,
                if active { color } else { inactive_color },
            );
        }
    }
}

fn draw_segment(
    buffer: &mut [u32],
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    thickness: i32,
    segment: usize,
    color: u32,
) {
    let half = height / 2;
    match segment {
        0 => fill_rect(
            buffer,
            x + thickness / 2,
            y,
            width - thickness,
            thickness,
            color,
        ),
        1 => fill_rect(
            buffer,
            x + width - thickness,
            y + thickness / 2,
            thickness,
            half - thickness / 2,
            color,
        ),
        2 => fill_rect(
            buffer,
            x + width - thickness,
            y + half,
            thickness,
            half - thickness / 2,
            color,
        ),
        3 => fill_rect(
            buffer,
            x + thickness / 2,
            y + height - thickness,
            width - thickness,
            thickness,
            color,
        ),
        4 => fill_rect(buffer, x, y + half, thickness, half - thickness / 2, color),
        5 => fill_rect(
            buffer,
            x,
            y + thickness / 2,
            thickness,
            half - thickness / 2,
            color,
        ),
        6 => fill_rect(
            buffer,
            x + thickness / 2,
            y + half - thickness / 2,
            width - thickness,
            thickness,
            color,
        ),
        _ => {}
    }
}
