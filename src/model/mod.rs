mod clock;
mod countdown;

pub use clock::{ClockSnapshot, LocalTimeProvider, TimeProvider};
pub use countdown::{friday_target_crossed, CountdownStatus};
