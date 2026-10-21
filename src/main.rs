#![cfg_attr(windows, windows_subsystem = "windows")]

mod app;
mod config;
mod input;
mod model;
mod render;
mod ui;
mod window_mode;

use app::App;
use config::{CANVAS_HEIGHT, CANVAS_WIDTH};
use input::{KeyboardInput, MouseInput};
use minifb::{Key, KeyRepeat, MouseButton, MouseMode, ScaleMode, Window, WindowOptions};
use model::LocalTimeProvider;
use ui::UiEvent;

fn main() {
    let mut window = Window::new(
        "Relogio Retro",
        CANVAS_WIDTH,
        CANVAS_HEIGHT,
        WindowOptions {
            title: true,
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
    let mut fullscreen = false;

    while window.is_open() && app.is_running() {
        let (raw_mouse_x, raw_mouse_y) = window
            .get_unscaled_mouse_pos(MouseMode::Clamp)
            .unwrap_or((0.0, 0.0));
        let (mouse_x, mouse_y) =
            canvas_mouse_position(window.get_size(), (raw_mouse_x, raw_mouse_y));

        let right_down = window.get_mouse_down(MouseButton::Right);
        let left_down = window.get_mouse_down(MouseButton::Left);
        let input = MouseInput {
            x: mouse_x,
            y: mouse_y,
            right_pressed: right_down && !previous_right_down,
            left_pressed: left_down && !previous_left_down,
        };
        let keyboard = KeyboardInput {
            fullscreen_pressed: window.is_key_pressed(Key::F, KeyRepeat::No),
            next_theme_pressed: window.is_key_pressed(Key::J, KeyRepeat::No),
            toggle_sticks_pressed: window.is_key_pressed(Key::K, KeyRepeat::No),
            animation_pressed: window.is_key_pressed(Key::A, KeyRepeat::No),
        };

        let (view, event) = app.tick(input, keyboard);

        match event {
            UiEvent::ToggleFullscreen => {
                fullscreen = !fullscreen;
                window_mode::set_fullscreen(&window, fullscreen);
            }
            UiEvent::None | UiEvent::ExitRequested => {}
        };

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

fn canvas_mouse_position(window_size: (usize, usize), mouse_position: (f32, f32)) -> (i32, i32) {
    let (window_width, window_height) = window_size;
    let width = window_width.max(1) as f32;
    let height = window_height.max(1) as f32;
    let x = (mouse_position.0 * CANVAS_WIDTH as f32 / width).floor() as i32;
    let y = (mouse_position.1 * CANVAS_HEIGHT as f32 / height).floor() as i32;

    (
        x.clamp(0, CANVAS_WIDTH as i32 - 1),
        y.clamp(0, CANVAS_HEIGHT as i32 - 1),
    )
}
