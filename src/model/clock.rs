use chrono::{DateTime, Datelike, Local, Timelike};

use super::countdown::{countdown_for, global_countdown_for, Countdown, GlobalCountdown};

pub trait TimeProvider {
    fn snapshot(&self, weekday_offset: i32) -> ClockSnapshot;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LocalTimeProvider;

impl TimeProvider for LocalTimeProvider {
    fn snapshot(&self, weekday_offset: i32) -> ClockSnapshot {
        let now = Local::now();
        let now = now
            .with_nanosecond(0)
            .expect("o horário local deve aceitar precisão de segundos");
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
            global_countdown: global_countdown_for(now, weekday_offset),
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
    pub global_countdown: GlobalCountdown,
}
