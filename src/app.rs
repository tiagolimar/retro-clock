use crate::input::{KeyboardInput, MouseInput};
use crate::model::{ClockSnapshot, LocalTimeProvider, TimeProvider};
use crate::ui::{UiController, UiEvent, UiView};

pub struct App<T: TimeProvider> {
    time_provider: T,
    ui: UiController,
    running: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct AppView {
    pub clock: ClockSnapshot,
    pub ui: UiView,
}

impl App<LocalTimeProvider> {
    pub fn new(time_provider: LocalTimeProvider) -> Self {
        App {
            time_provider,
            ui: UiController::new(),
            running: true,
        }
    }
}

impl<T: TimeProvider> App<T> {
    pub fn tick(&mut self, mouse: MouseInput, keyboard: KeyboardInput) -> (AppView, UiEvent) {
        let (event, ui) = self.ui.update(mouse, keyboard);
        if event == UiEvent::ExitRequested {
            self.running = false;
        }

        (
            AppView {
                clock: self.time_provider.snapshot(),
                ui,
            },
            event,
        )
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}
