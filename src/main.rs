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
use input::MouseInput;
use model::LocalTimeProvider;
use minifb::{MouseButton, MouseMode, ScaleMode, Window, WindowOptions};
use ui::UiEvent;

#[derive(Clone, Copy, Debug)]
struct WindowDrag {
    window_position: (isize, isize),
    cursor_position: (isize, isize),
}

fn main() {
    let mut window = Window::new(
        "",
        CANVAS_WIDTH,
        CANVAS_HEIGHT,
        WindowOptions {
            title: false,
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
    let mut menu_visible = false;
    let mut window_drag = None;

    while window.is_open() && app.is_running() {
        let (raw_mouse_x, raw_mouse_y) = window
            .get_unscaled_mouse_pos(MouseMode::Clamp)
            .unwrap_or((0.0, 0.0));
        let (mouse_x, mouse_y) = canvas_mouse_position(
            window.get_size(),
            (raw_mouse_x, raw_mouse_y),
        );

        let right_down = window.get_mouse_down(MouseButton::Right);
        let left_down = window.get_mouse_down(MouseButton::Left);
        let input = MouseInput {
            x: mouse_x,
            y: mouse_y,
            right_pressed: right_down && !previous_right_down,
            left_pressed: left_down && !previous_left_down,
        };

        let menu_was_visible = menu_visible;
        let (view, event) = app.tick(input);
        menu_visible = view.ui.menu.visible;

        match event {
            UiEvent::ToggleFullscreen => window_mode::set_fullscreen(&window, true),
            UiEvent::RestoreWindow => window_mode::set_fullscreen(&window, false),
            UiEvent::None | UiEvent::ExitRequested => false,
        };

        update_window_drag(
            &mut window,
            left_down,
            input.left_pressed,
            menu_was_visible,
            (raw_mouse_x, raw_mouse_y),
            &mut window_drag,
        );

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

fn update_window_drag(
    window: &mut Window,
    left_down: bool,
    left_pressed: bool,
    menu_was_visible: bool,
    mouse_position: (f32, f32),
    drag: &mut Option<WindowDrag>,
) {
    if !left_down {
        *drag = None;
        return;
    }

    let window_position = window.get_position();
    let cursor_position = (
        window_position.0 + mouse_position.0 as isize,
        window_position.1 + mouse_position.1 as isize,
    );

    if left_pressed && !menu_was_visible {
        *drag = Some(WindowDrag {
            window_position,
            cursor_position,
        });
    }

    if let Some(origin) = *drag {
        window.set_position(
            origin.window_position.0 + cursor_position.0 - origin.cursor_position.0,
            origin.window_position.1 + cursor_position.1 - origin.cursor_position.1,
        );
    }
}

