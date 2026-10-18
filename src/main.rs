#![cfg_attr(windows, windows_subsystem = "windows")]

mod app;
mod config;
mod input;
mod model;
mod render;
mod ui;

use app::{App, LocalTimeProvider};
use config::{CANVAS_HEIGHT, CANVAS_WIDTH, WINDOW_TITLE};
use input::MouseInput;
use minifb::{MouseButton, MouseMode, ScaleMode, Window, WindowOptions};

fn main() {
    let mut window = Window::new(
        WINDOW_TITLE,
        CANVAS_WIDTH,
        CANVAS_HEIGHT,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::Stretch,
            ..WindowOptions::default()
        },
    )
    .expect("Não foi possível criar a janela");

    window.set_target_fps(30);

    let mut app = App::new(LocalTimeProvider);
    let mut buffer = vec![0; CANVAS_WIDTH * CANVAS_HEIGHT];
    let mut previous_right_down = false;
    let mut previous_left_down = false;

    while window.is_open() && app.is_running() {
        let (mouse_x, mouse_y) = window
            .get_unscaled_mouse_pos(MouseMode::Clamp)
            .map(|(x, y)| (x as i32, y as i32))
            .unwrap_or((0, 0));

        let right_down = window.get_mouse_down(MouseButton::Right);
        let left_down = window.get_mouse_down(MouseButton::Left);
        let input = MouseInput {
            x: mouse_x,
            y: mouse_y,
            right_pressed: right_down && !previous_right_down,
            left_pressed: left_down && !previous_left_down,
        };

        let view = app.tick(input);
        render::draw(&mut buffer, &view);

        if window
            .update_with_buffer(&buffer, CANVAS_WIDTH, CANVAS_HEIGHT)
            .is_err()
        {
            break;
        }

        previous_right_down = right_down;
        previous_left_down = left_down;
    }
}

