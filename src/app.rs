use crate::input::{KeyboardInput, MouseInput};
use crate::model::{ClockSnapshot, CountdownStatus, LocalTimeProvider, TimeProvider};
use crate::ui::{UiController, UiEvent, UiView};
use std::time::{Duration, Instant};

const END_OF_CYCLE_ANIMATION_PASSES: u64 = 6;
const END_OF_CYCLE_ANIMATION_PASS_DURATION: Duration = Duration::from_secs(5);
const END_OF_CYCLE_ANIMATION_TEXT: &str = "ACABOU";
const END_OF_CYCLE_ANIMATION_FINAL_TEXT: &str = "ACABOOUUUUUUUUUUU";

pub struct App<T: TimeProvider> {
    time_provider: T,
    ui: UiController,
    running: bool,
    previous_countdown_status: Option<CountdownStatus>,
    end_of_cycle_animation_started_at: Option<Instant>,
}

#[derive(Clone, Copy, Debug)]
pub struct AppView {
    pub clock: ClockSnapshot,
    pub ui: UiView,
    pub end_of_cycle_animation: Option<EndOfCycleAnimation>,
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
            previous_countdown_status: None,
            end_of_cycle_animation_started_at: None,
        }
    }
}

impl<T: TimeProvider> App<T> {
    pub fn tick(&mut self, mouse: MouseInput, keyboard: KeyboardInput) -> (AppView, UiEvent) {
        let (event, ui) = self.ui.update(mouse, keyboard);
        if event == UiEvent::ExitRequested {
            self.running = false;
        }

        let clock = self.time_provider.snapshot();
        let end_of_cycle_animation =
            self.end_of_cycle_animation(clock.countdown.status, keyboard.animation_pressed);

        (
            AppView {
                clock,
                ui,
                end_of_cycle_animation,
            },
            event,
        )
    }

    fn end_of_cycle_animation(
        &mut self,
        countdown_status: CountdownStatus,
        animation_pressed: bool,
    ) -> Option<EndOfCycleAnimation> {
        let entered_finished = countdown_status == CountdownStatus::Finished
            && self.previous_countdown_status != Some(CountdownStatus::Finished);
        if animation_pressed || entered_finished {
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

fn animation_at(elapsed: Duration) -> Option<EndOfCycleAnimation> {
    let pass_duration_seconds = END_OF_CYCLE_ANIMATION_PASS_DURATION.as_secs();
    let pass = elapsed.as_secs() / pass_duration_seconds;
    if pass >= END_OF_CYCLE_ANIMATION_PASSES {
        return None;
    }

    let progress = (elapsed - END_OF_CYCLE_ANIMATION_PASS_DURATION * pass as u32)
        .as_secs_f32()
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
    use super::{animation_at, END_OF_CYCLE_ANIMATION_FINAL_TEXT, END_OF_CYCLE_ANIMATION_TEXT};
    use std::time::Duration;

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
