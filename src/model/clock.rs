use chrono::{DateTime, Datelike, Local, Timelike};

use super::countdown::{countdown_for, friday_countdown_for, Countdown, FridayCountdown};

pub trait TimeProvider {
    fn snapshot(&self) -> ClockSnapshot;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LocalTimeProvider;

impl TimeProvider for LocalTimeProvider {
    fn snapshot(&self) -> ClockSnapshot {
        let now = Local::now();
        let seconds_since_midnight = now.hour() * 3600 + now.minute() * 60 + now.second();

        ClockSnapshot {
            moment: now,
            hour: now.hour(),
            minute: now.minute(),
            second: now.second(),
            day: now.day(),
            month: now.month(),
            year: now.year(),
            countdown: countdown_for(seconds_since_midnight),
            friday_countdown: friday_countdown_for(now),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ClockSnapshot {
    pub moment: DateTime<Local>,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub day: u32,
    pub month: u32,
    pub year: i32,
    pub countdown: Countdown,
    pub friday_countdown: FridayCountdown,
}
