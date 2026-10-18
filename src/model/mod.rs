mod clock;
mod countdown;

pub use clock::{ClockSnapshot, LocalTimeProvider, TimeProvider};
pub use countdown::{countdown_for, Countdown, CountdownStatus, END_OF_DAY_SECONDS, START_OF_DAY_SECONDS};

