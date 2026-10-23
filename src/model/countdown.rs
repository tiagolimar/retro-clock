use chrono::{DateTime, Datelike, Duration, Local, TimeZone, Weekday};

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FridayCountdown {
    pub remaining_seconds: u32,
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

pub fn friday_countdown_for(now: DateTime<Local>) -> FridayCountdown {
    let target = next_friday_at_five(now);
    let remaining_seconds = target.signed_duration_since(now).num_seconds().max(0) as u32;

    FridayCountdown { remaining_seconds }
}

pub fn next_friday_at_five(now: DateTime<Local>) -> DateTime<Local> {
    let days_until_friday =
        (Weekday::Fri.num_days_from_monday() + 7 - now.weekday().num_days_from_monday()) % 7;
    let candidate_date = now.date_naive() + Duration::days(days_until_friday as i64);
    let candidate = Local
        .with_ymd_and_hms(
            candidate_date.year(),
            candidate_date.month(),
            candidate_date.day(),
            17,
            0,
            0,
        )
        .single()
        .expect("17:00 deve ser um horário local válido");

    if candidate < now {
        candidate + Duration::days(7)
    } else {
        candidate
    }
}

pub fn friday_target_crossed(previous: DateTime<Local>, current: DateTime<Local>) -> bool {
    let target = next_friday_at_five(previous);
    previous < target && current >= target
}

#[cfg(test)]
mod tests {
    use chrono::{Local, TimeZone};

    use super::{
        countdown_for, friday_countdown_for, friday_target_crossed, CountdownStatus,
        END_OF_DAY_SECONDS, START_OF_DAY_SECONDS,
    };

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

    #[test]
    fn counts_down_to_the_next_friday_at_five() {
        let thursday = Local
            .with_ymd_and_hms(2026, 10, 22, 16, 0, 0)
            .single()
            .unwrap();

        assert_eq!(friday_countdown_for(thursday).remaining_seconds, 25 * 3600);
    }

    #[test]
    fn friday_after_five_targets_the_following_week() {
        let friday_evening = Local
            .with_ymd_and_hms(2026, 10, 23, 18, 0, 0)
            .single()
            .unwrap();

        assert_eq!(
            friday_countdown_for(friday_evening).remaining_seconds,
            6 * 24 * 3600 + 23 * 3600
        );
    }

    #[test]
    fn detects_crossing_the_friday_target_even_if_a_frame_is_skipped() {
        let before_target = Local
            .with_ymd_and_hms(2026, 10, 23, 16, 59, 59)
            .single()
            .unwrap();
        let after_target = Local
            .with_ymd_and_hms(2026, 10, 23, 17, 0, 2)
            .single()
            .unwrap();

        assert!(friday_target_crossed(before_target, after_target));
    }
}
