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
pub struct GlobalCountdown {
    pub remaining_seconds: u32,
    pub target_weekday: Weekday,
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

pub const CELEBRATION_LEAD_SECONDS: i64 = 10;

pub fn global_countdown_for(now: DateTime<Local>, weekday_offset: i32) -> GlobalCountdown {
    let target_weekday = weekday_for_offset(weekday_offset);
    let target = next_weekday_at_five(now, target_weekday);
    let remaining_seconds = target.signed_duration_since(now).num_seconds().max(0) as u32;

    GlobalCountdown {
        remaining_seconds,
        target_weekday,
    }
}

pub fn weekday_for_offset(offset: i32) -> Weekday {
    match (Weekday::Fri.num_days_from_monday() as i32 + offset).rem_euclid(7) {
        0 => Weekday::Mon,
        1 => Weekday::Tue,
        2 => Weekday::Wed,
        3 => Weekday::Thu,
        4 => Weekday::Fri,
        5 => Weekday::Sat,
        _ => Weekday::Sun,
    }
}

pub fn next_weekday_at_five(now: DateTime<Local>, weekday: Weekday) -> DateTime<Local> {
    let days_until_target =
        (weekday.num_days_from_monday() + 7 - now.weekday().num_days_from_monday()) % 7;
    let candidate_date = now.date_naive() + Duration::days(days_until_target as i64);
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

pub fn global_celebration_crossed(
    previous: DateTime<Local>,
    current: DateTime<Local>,
    weekday_offset: i32,
) -> bool {
    let target = next_weekday_at_five(previous, weekday_for_offset(weekday_offset));
    let celebration_start = target - Duration::seconds(CELEBRATION_LEAD_SECONDS);
    previous < celebration_start && current >= celebration_start
}

#[cfg(test)]
mod tests {
    use chrono::{Local, TimeZone};

    use super::{
        countdown_for, global_celebration_crossed, global_countdown_for, CountdownStatus,
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

        assert_eq!(
            global_countdown_for(thursday, 0).remaining_seconds,
            25 * 3600
        );
    }

    #[test]
    fn friday_after_five_targets_the_following_week() {
        let friday_evening = Local
            .with_ymd_and_hms(2026, 10, 23, 18, 0, 0)
            .single()
            .unwrap();

        assert_eq!(
            global_countdown_for(friday_evening, 0).remaining_seconds,
            6 * 24 * 3600 + 23 * 3600
        );
    }

    #[test]
    fn advances_the_global_target_to_saturday() {
        let thursday = Local
            .with_ymd_and_hms(2026, 10, 22, 16, 0, 0)
            .single()
            .unwrap();

        let countdown = global_countdown_for(thursday, 1);

        assert_eq!(countdown.target_weekday, chrono::Weekday::Sat);
        assert_eq!(countdown.remaining_seconds, 49 * 3600);
    }

    #[test]
    fn detects_the_celebration_start_ten_seconds_before_target() {
        let before_target = Local
            .with_ymd_and_hms(2026, 10, 23, 16, 59, 49)
            .single()
            .unwrap();
        let after_target = Local
            .with_ymd_and_hms(2026, 10, 23, 16, 59, 50)
            .single()
            .unwrap();

        assert!(global_celebration_crossed(before_target, after_target, 0));
    }
}
