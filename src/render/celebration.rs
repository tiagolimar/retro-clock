use crate::config::{CANVAS_HEIGHT, CANVAS_WIDTH};

use super::canvas::{clear, fill_rect};
use super::colors::Palette;
use super::font;

const BURSTS: [(i32, i32, f32); 5] = [
    (150, 145, 0.00),
    (810, 145, 0.22),
    (110, 420, 0.46),
    (850, 420, 0.68),
    (480, 90, 0.84),
];

const COUNTDOWN_DURATION_SECONDS: f32 = 10.0;
const MAIN_ANIMATION_DURATION_SECONDS: f32 = 20.0;
const TOTAL_ANIMATION_DURATION_SECONDS: f32 =
    COUNTDOWN_DURATION_SECONDS + MAIN_ANIMATION_DURATION_SECONDS;

pub fn draw(buffer: &mut [u32], progress: f32, palette: Palette) {
    let progress = progress.clamp(0.0, 1.0);
    let elapsed = progress * TOTAL_ANIMATION_DURATION_SECONDS;

    if elapsed < COUNTDOWN_DURATION_SECONDS {
        draw_countdown(buffer, elapsed, palette);
        return;
    }

    let main_progress =
        ((elapsed - COUNTDOWN_DURATION_SECONDS) / MAIN_ANIMATION_DURATION_SECONDS).clamp(0.0, 1.0);
    draw_main(buffer, main_progress, palette);
}

fn draw_countdown(buffer: &mut [u32], elapsed: f32, palette: Palette) {
    let second = countdown_seconds(elapsed);
    let beat = (elapsed * std::f32::consts::TAU * 1.2).sin().abs();
    let tick_phase = elapsed.fract();
    let tick_flash = 1.0 - smoothstep(tick_phase / 0.18);
    let intensity = 0.45 + beat * 0.55;
    let base = mix_color(palette.background, palette.panel, 0.24 + beat * 0.22);
    let glow = mix_color(palette.accent, palette.highlight, 0.4 + beat * 0.6);

    clear(buffer, base);

    if tick_flash > 0.0 {
        fill_rect(
            buffer,
            0,
            0,
            CANVAS_WIDTH as i32,
            CANVAS_HEIGHT as i32,
            mix_color(base, palette.highlight, tick_flash * 0.16),
        );
    }

    for y in (0..CANVAS_HEIGHT as i32).step_by(10) {
        fill_rect(
            buffer,
            0,
            y,
            CANVAS_WIDTH as i32,
            1,
            mix_color(base, palette.muted, 0.2 + intensity * 0.18),
        );
    }

    let border = mix_color(palette.accent, palette.highlight, intensity);
    fill_rect(buffer, 12, 12, CANVAS_WIDTH as i32 - 24, 2, border);
    fill_rect(
        buffer,
        12,
        CANVAS_HEIGHT as i32 - 14,
        CANVAS_WIDTH as i32 - 24,
        2,
        border,
    );
    fill_rect(buffer, 12, 12, 2, CANVAS_HEIGHT as i32 - 24, border);
    fill_rect(
        buffer,
        CANVAS_WIDTH as i32 - 14,
        12,
        2,
        CANVAS_HEIGHT as i32 - 24,
        border,
    );

    let center_x = CANVAS_WIDTH as i32 / 2;
    let center_y = CANVAS_HEIGHT as i32 / 2;
    let radius = 148.0 + beat * 34.0;
    draw_ring(buffer, center_x, center_y, radius, 2, glow);
    draw_ring(
        buffer,
        center_x,
        center_y,
        radius + 22.0 + beat * 16.0,
        1,
        mix_color(palette.muted, glow, 0.6),
    );

    for ray in 0..16 {
        let angle = ray as f32 * std::f32::consts::TAU / 16.0 + elapsed * 0.7;
        let ray_radius = radius + 54.0 + beat * 28.0;
        let end_x = center_x as f32 + angle.cos() * ray_radius;
        let end_y = center_y as f32 + angle.sin() * ray_radius;
        draw_line(
            buffer,
            center_x,
            center_y,
            end_x as i32,
            end_y as i32,
            if ray % 4 == 0 { 2 } else { 1 },
            mix_color(palette.shadow, glow, 0.48 + intensity * 0.35),
        );
    }

    font::draw_text_centered(buffer, "SEXTA EM", 104, 3, palette.muted);
    font::draw_text_centered(
        buffer,
        &format!("{second:02}"),
        174,
        36,
        mix_color(palette.accent, palette.highlight, 0.58 + beat * 0.42),
    );
    font::draw_text_centered(buffer, "PREPARE-SE", 452, 2, palette.highlight);
}

fn countdown_seconds(elapsed: f32) -> u32 {
    (COUNTDOWN_DURATION_SECONDS - elapsed)
        .ceil()
        .clamp(1.0, COUNTDOWN_DURATION_SECONDS) as u32
}

fn draw_main(buffer: &mut [u32], progress: f32, palette: Palette) {
    let entrance = smoothstep(progress / 0.08);
    let exit = 1.0 - smoothstep((progress - 0.9) / 0.1);
    let intensity = entrance * exit;
    let pulse = (progress * 18.0).sin().abs();
    let base = mix_color(palette.background, palette.panel, 0.28 + pulse * 0.24);

    clear(buffer, base);

    let flash = (1.0 - ((progress * 14.0).sin().abs() * 1.8)).max(0.0) * intensity;
    if flash > 0.0 {
        fill_rect(
            buffer,
            0,
            0,
            CANVAS_WIDTH as i32,
            CANVAS_HEIGHT as i32,
            mix_color(base, palette.highlight, flash * 0.45),
        );
    }

    for y in (0..CANVAS_HEIGHT as i32).step_by(10) {
        fill_rect(
            buffer,
            0,
            y,
            CANVAS_WIDTH as i32,
            1,
            mix_color(base, palette.muted, 0.25 * intensity),
        );
    }

    for &(x, y, phase) in &BURSTS {
        draw_burst(buffer, x, y, phase, progress, intensity, palette);
    }

    let center_x = CANVAS_WIDTH as i32 / 2;
    let center_y = CANVAS_HEIGHT as i32 / 2;
    let breathing_radius = 76.0 + pulse * 42.0;
    draw_ring(
        buffer,
        center_x,
        center_y,
        breathing_radius,
        2,
        mix_color(palette.accent, palette.highlight, 0.72 * intensity),
    );
    draw_ring(
        buffer,
        center_x,
        center_y,
        breathing_radius + 24.0 + pulse * 18.0,
        1,
        mix_color(palette.muted, palette.accent, 0.65 * intensity),
    );

    for ray in 0..18 {
        let angle = ray as f32 * std::f32::consts::TAU / 18.0 + progress * 2.2;
        let radius = 155.0 + pulse * 44.0;
        let end_x = center_x as f32 + angle.cos() * radius;
        let end_y = center_y as f32 + angle.sin() * radius;
        draw_line(
            buffer,
            center_x,
            center_y,
            end_x as i32,
            end_y as i32,
            1,
            mix_color(palette.shadow, palette.accent, 0.58 * intensity),
        );
    }

    if progress > 0.08 {
        let reveal = smoothstep((progress - 0.08) / 0.12);
        let title_color = mix_color(palette.accent, palette.highlight, reveal);
        font::draw_text_centered(buffer, "SEXTA CHEGOU", 210, 5, title_color);
        font::draw_text_centered(
            buffer,
            "17:00  MODO FESTA",
            276,
            2,
            mix_color(palette.muted, palette.highlight, 0.72 * intensity),
        );
    }

    draw_confetti(buffer, progress, intensity, palette);
}

fn draw_burst(
    buffer: &mut [u32],
    center_x: i32,
    center_y: i32,
    phase: f32,
    progress: f32,
    intensity: f32,
    palette: Palette,
) {
    let cycle = (progress * 1.45 + phase).fract();
    if cycle < 0.06 {
        return;
    }

    let expansion = ((cycle - 0.06) / 0.94).clamp(0.0, 1.0);
    let fade = (1.0 - expansion) * intensity;
    let color = mix_color(palette.muted, palette.highlight, 0.35 + fade * 0.65);
    let base_angle = progress * 0.9 + phase * 4.0;

    for ray in 0..14 {
        let angle = base_angle + ray as f32 * std::f32::consts::TAU / 14.0;
        let start_radius = expansion * 16.0;
        let end_radius = expansion * (110.0 + (ray % 3) as f32 * 24.0);
        let start_x = center_x as f32 + angle.cos() * start_radius;
        let start_y = center_y as f32 + angle.sin() * start_radius;
        let end_x = center_x as f32 + angle.cos() * end_radius;
        let end_y = center_y as f32 + angle.sin() * end_radius;

        draw_line(
            buffer,
            start_x as i32,
            start_y as i32,
            end_x as i32,
            end_y as i32,
            if ray % 4 == 0 { 2 } else { 1 },
            color,
        );
        fill_rect(buffer, end_x as i32 - 2, end_y as i32 - 2, 4, 4, color);
    }
}

fn draw_confetti(buffer: &mut [u32], progress: f32, intensity: f32, palette: Palette) {
    for index in 0..42 {
        let index = index as f32;
        let x = ((index * 83.0 + progress * (120.0 + index * 7.0)) % CANVAS_WIDTH as f32) as i32;
        let fall = (progress * (90.0 + index * 3.0) + index * 29.0) % CANVAS_HEIGHT as f32;
        let y = fall as i32;
        let size = 2 + (index as i32 % 3);
        let color = if index as i32 % 2 == 0 {
            mix_color(palette.accent, palette.highlight, 0.65 * intensity)
        } else {
            mix_color(palette.muted, palette.accent, 0.72 * intensity)
        };
        fill_rect(buffer, x, y, size, size * 2, color);
    }
}

fn draw_ring(buffer: &mut [u32], center_x: i32, center_y: i32, radius: f32, size: i32, color: u32) {
    for step in 0..180 {
        let angle = step as f32 * std::f32::consts::TAU / 180.0;
        let x = center_x + (angle.cos() * radius) as i32;
        let y = center_y + (angle.sin() * radius) as i32;
        fill_rect(buffer, x, y, size, size, color);
    }
}

fn draw_line(
    buffer: &mut [u32],
    mut x0: i32,
    mut y0: i32,
    x1: i32,
    y1: i32,
    size: i32,
    color: u32,
) {
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut error = dx + dy;

    loop {
        fill_rect(buffer, x0, y0, size, size, color);
        if x0 == x1 && y0 == y1 {
            break;
        }

        let double_error = 2 * error;
        if double_error >= dy {
            error += dy;
            x0 += sx;
        }
        if double_error <= dx {
            error += dx;
            y0 += sy;
        }
    }
}

fn smoothstep(value: f32) -> f32 {
    let value = value.clamp(0.0, 1.0);
    value * value * (3.0 - 2.0 * value)
}

fn mix_color(from: u32, to: u32, amount: f32) -> u32 {
    let amount = amount.clamp(0.0, 1.0);
    let from_red = ((from >> 16) & 0xff) as f32;
    let from_green = ((from >> 8) & 0xff) as f32;
    let from_blue = (from & 0xff) as f32;
    let to_red = ((to >> 16) & 0xff) as f32;
    let to_green = ((to >> 8) & 0xff) as f32;
    let to_blue = (to & 0xff) as f32;

    let red = from_red + (to_red - from_red) * amount;
    let green = from_green + (to_green - from_green) * amount;
    let blue = from_blue + (to_blue - from_blue) * amount;

    ((red as u32) << 16) | ((green as u32) << 8) | blue as u32
}

#[cfg(test)]
mod tests {
    use super::countdown_seconds;

    #[test]
    fn countdown_starts_at_ten_seconds() {
        assert_eq!(countdown_seconds(0.0), 10);
        assert_eq!(countdown_seconds(0.25), 10);
    }

    #[test]
    fn countdown_shows_one_during_the_last_second() {
        assert_eq!(countdown_seconds(9.01), 1);
        assert_eq!(countdown_seconds(9.99), 1);
    }
}
