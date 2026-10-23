use chrono::{DateTime, Local};
use std::time::{Duration, Instant};

use crate::input::{KeyboardInput, MouseInput};
use crate::model::{
    friday_target_crossed, ClockSnapshot, CountdownStatus, LocalTimeProvider, TimeProvider,
};
use crate::ui::{UiController, UiEvent, UiView};

const END_OF_CYCLE_ANIMATION_PASSES: u64 = 6;
const END_OF_CYCLE_ANIMATION_PASS_DURATION: Duration = Duration::from_secs(5);
const END_OF_CYCLE_ANIMATION_TEXT: &str = "ACABOU";
const END_OF_CYCLE_ANIMATION_FINAL_TEXT: &str = "ACABOOUUUUUUUUUUU";
const CELEBRATION_SEQUENCE_DURATION: Duration = Duration::from_secs(30);

pub struct App<T: TimeProvider> {
    time_provider: T,
    ui: UiController,
    running: bool,
    last_moment: Option<DateTime<Local>>,
    celebration_started_at: Option<Instant>,
    previous_countdown_status: Option<CountdownStatus>,
    end_of_cycle_animation_started_at: Option<Instant>,
}

#[derive(Clone, Copy, Debug)]
pub struct AppView {
    pub clock: ClockSnapshot,
    pub ui: UiView,
    pub celebration: Option<CelebrationView>,
    pub end_of_cycle_animation: Option<EndOfCycleAnimation>,
}

#[derive(Clone, Copy, Debug)]
pub struct CelebrationView {
    pub progress: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct EndOfCycleAnimation {
    pub text: &'static str,
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
            previous_countdown_status: None,
            end_of_cycle_animation_started_at: None,
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

        let end_of_cycle_animation = self.end_of_cycle_animation(clock.countdown.status);

        (
            AppView {
                clock,
                ui,
                celebration: self.celebration_view(),
                end_of_cycle_animation,
            },
            event,
        )
    }

    fn celebration_view(&mut self) -> Option<CelebrationView> {
        let started_at = self.celebration_started_at?;
        let elapsed = started_at.elapsed();

        if elapsed >= CELEBRATION_SEQUENCE_DURATION {
            self.celebration_started_at = None;
            return None;
        }

        Some(CelebrationView {
            progress: elapsed.as_secs_f32() / CELEBRATION_SEQUENCE_DURATION.as_secs_f32(),
        })
    }

    fn end_of_cycle_animation(
        &mut self,
        countdown_status: CountdownStatus,
    ) -> Option<EndOfCycleAnimation> {
        if entered_finished(self.previous_countdown_status, countdown_status) {
            self.end_of_cycle_animation_started_at = Some(Instant::now());
        }

        self.previous_countdown_status = Some(countdown_status);

        let Some(started_at) = self.end_of_cycle_animation_started_at else {
            return None;
        };
        let elapsed = started_at.elapsed();
        let animation = animation_at(elapsed);
        if animation.is_none() {
            self.end_of_cycle_animation_started_at = None;
        }
        animation
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}

fn entered_finished(previous_status: Option<CountdownStatus>, status: CountdownStatus) -> bool {
    previous_status == Some(CountdownStatus::Active) && status == CountdownStatus::Finished
}

fn animation_at(elapsed: Duration) -> Option<EndOfCycleAnimation> {
    let pass_duration_seconds = END_OF_CYCLE_ANIMATION_PASS_DURATION.as_secs();
    let pass = elapsed.as_secs() / pass_duration_seconds;
    if pass >= END_OF_CYCLE_ANIMATION_PASSES {
        return None;
    }

    let progress = (elapsed - END_OF_CYCLE_ANIMATION_PASS_DURATION * pass as u32).as_secs_f32()
        / END_OF_CYCLE_ANIMATION_PASS_DURATION.as_secs_f32();
    let text = if pass >= END_OF_CYCLE_ANIMATION_PASSES - 2 {
        END_OF_CYCLE_ANIMATION_FINAL_TEXT
    } else {
        END_OF_CYCLE_ANIMATION_TEXT
    };

    Some(EndOfCycleAnimation { text, progress })
}

#[cfg(test)]
mod tests {
    use super::{
        animation_at, entered_finished, App, END_OF_CYCLE_ANIMATION_FINAL_TEXT,
        END_OF_CYCLE_ANIMATION_TEXT,
    };
    use crate::input::{KeyboardInput, MouseInput};
    use crate::model::{CountdownStatus, LocalTimeProvider};
    use crate::ui::UiEvent;
    use std::time::Duration;

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

    #[test]
    fn end_of_cycle_animation_only_starts_after_an_active_cycle() {
        assert!(entered_finished(
            Some(CountdownStatus::Active),
            CountdownStatus::Finished
        ));
        assert!(!entered_finished(None, CountdownStatus::Finished));
        assert!(!entered_finished(
            Some(CountdownStatus::WaitingForStart),
            CountdownStatus::Finished
        ));
    }

    #[test]
    fn end_of_cycle_animation_repeats_four_times_and_final_twice() {
        assert_eq!(
            animation_at(Duration::from_millis(0)).unwrap().text,
            END_OF_CYCLE_ANIMATION_TEXT
        );
        assert_eq!(
            animation_at(Duration::from_millis(19_999)).unwrap().text,
            END_OF_CYCLE_ANIMATION_TEXT
        );
        assert_eq!(
            animation_at(Duration::from_secs(20)).unwrap().text,
            END_OF_CYCLE_ANIMATION_FINAL_TEXT
        );
        assert_eq!(
            animation_at(Duration::from_millis(24_999)).unwrap().text,
            END_OF_CYCLE_ANIMATION_FINAL_TEXT
        );
        assert_eq!(
            animation_at(Duration::from_secs(25)).unwrap().text,
            END_OF_CYCLE_ANIMATION_FINAL_TEXT
        );
        assert!(animation_at(Duration::from_secs(30)).is_none());
    }
}
