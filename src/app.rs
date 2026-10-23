use chrono::{DateTime, Local};
use std::time::{Duration, Instant};

use crate::input::{KeyboardInput, MouseInput};
use crate::model::{friday_target_crossed, ClockSnapshot, LocalTimeProvider, TimeProvider};
use crate::ui::{UiController, UiEvent, UiView};

const CELEBRATION_DURATION: Duration = Duration::from_secs(20);

pub struct App<T: TimeProvider> {
    time_provider: T,
    ui: UiController,
    running: bool,
    last_moment: Option<DateTime<Local>>,
    celebration_started_at: Option<Instant>,
}

#[derive(Clone, Copy, Debug)]
pub struct AppView {
    pub clock: ClockSnapshot,
    pub ui: UiView,
    pub celebration: Option<CelebrationView>,
}

#[derive(Clone, Copy, Debug)]
pub struct CelebrationView {
    pub progress: f32,
}

impl App<LocalTimeProvider> {
    pub fn new(time_provider: LocalTimeProvider) -> Self {
        App {
            time_provider,
            ui: UiController::new(),
            running: true,
            last_moment: None,
            celebration_started_at: None,
        }
    }
}

impl<T: TimeProvider> App<T> {
    pub fn tick(&mut self, mouse: MouseInput, keyboard: KeyboardInput) -> (AppView, UiEvent) {
        let (event, ui) = self.ui.update(mouse, keyboard);
        let clock = self.time_provider.snapshot();

        let friday_target_reached = self
            .last_moment
            .map_or(clock.friday_countdown.remaining_seconds == 0, |previous| {
                friday_target_crossed(previous, clock.moment)
            });

        if keyboard.celebration_pressed || friday_target_reached {
            self.celebration_started_at = Some(Instant::now());
        }

        self.last_moment = Some(clock.moment);

        if event == UiEvent::ExitRequested {
            self.running = false;
        }

        (
            AppView {
                clock,
                ui,
                celebration: self.celebration_view(),
            },
            event,
        )
    }

    fn celebration_view(&mut self) -> Option<CelebrationView> {
        let started_at = self.celebration_started_at?;
        let elapsed = started_at.elapsed();

        if elapsed >= CELEBRATION_DURATION {
            self.celebration_started_at = None;
            return None;
        }

        Some(CelebrationView {
            progress: elapsed.as_secs_f32() / CELEBRATION_DURATION.as_secs_f32(),
        })
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}

#[cfg(test)]
mod tests {
    use super::App;
    use crate::input::{KeyboardInput, MouseInput};
    use crate::model::LocalTimeProvider;
    use crate::ui::UiEvent;

    #[test]
    fn a_starts_the_global_celebration() {
        let mut app = App::new(LocalTimeProvider);
        let (view, event) = app.tick(
            MouseInput::default(),
            KeyboardInput {
                celebration_pressed: true,
                ..KeyboardInput::default()
            },
        );

        assert_eq!(event, UiEvent::None);
        assert!(view.celebration.is_some());
    }
}
