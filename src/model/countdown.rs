pub const START_OF_DAY_SECONDS: u32 = 8 * 60 * 60;
pub const END_OF_DAY_SECONDS: u32 = 17 * 60 * 60;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CountdownStatus {
    WaitingForStart,
    Active,
    Finished,
}

#[derive(Clone, Copy, Debug)]
pub struct Countdown {
    pub remaining_seconds: u32,
    pub status: CountdownStatus,
}

pub fn countdown_for(current_seconds: u32) -> Countdown {
    if current_seconds < START_OF_DAY_SECONDS {
        Countdown {
            remaining_seconds: END_OF_DAY_SECONDS - START_OF_DAY_SECONDS,
            status: CountdownStatus::WaitingForStart,
        }
    } else if current_seconds < END_OF_DAY_SECONDS {
        Countdown {
            remaining_seconds: END_OF_DAY_SECONDS - current_seconds,
            status: CountdownStatus::Active,
        }
    } else {
        Countdown {
            remaining_seconds: 0,
            status: CountdownStatus::Finished,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{countdown_for, CountdownStatus, END_OF_DAY_SECONDS, START_OF_DAY_SECONDS};

    #[test]
    fn waits_for_the_eight_am_start() {
        let countdown = countdown_for(7 * 3600 + 59 * 60 + 59);
        assert_eq!(countdown.remaining_seconds, 9 * 3600);
        assert_eq!(countdown.status, CountdownStatus::WaitingForStart);
    }

    #[test]
    fn counts_down_until_five_pm() {
        let countdown = countdown_for(12 * 3600);
        assert_eq!(countdown.remaining_seconds, 5 * 3600);
        assert_eq!(countdown.status, CountdownStatus::Active);
    }

    #[test]
    fn stops_at_five_pm() {
        let countdown = countdown_for(END_OF_DAY_SECONDS);
        assert_eq!(countdown.remaining_seconds, 0);
        assert_eq!(countdown.status, CountdownStatus::Finished);
    }

    #[test]
    fn boundary_at_eight_is_active() {
        let countdown = countdown_for(START_OF_DAY_SECONDS);
        assert_eq!(countdown.remaining_seconds, 9 * 3600);
        assert_eq!(countdown.status, CountdownStatus::Active);
    }
}

