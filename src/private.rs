use std::str::FromStr;

use chrono::NaiveDateTime;
use thiserror::Error;

// days_from_ce_from_year(1969) + 1)
// 719_163_i64
const DAYS_FROM_CE_TO_UNIX_EPOCH: i64 = 719_163_i64;

// 365_i64
const DAYS_PER_COMMON_YEAR: i64 = 365;

// 1_461_i64
const DAYS_PER_4_YEARS: i64 = DAYS_PER_COMMON_YEAR * 4 + 1;

// 36_524_i64
const DAYS_PER_100_YEARS: i64 = DAYS_PER_4_YEARS * 25 - 1;

// 146_097_i64
const DAYS_PER_400_YEARS: i64 = DAYS_PER_100_YEARS * 4 + 1;

// 86_400_i64
const SECONDS_PER_DAY: i64 = 24 * 60 * 60;

// 62_135_683_200_i64;
const SECONDS_FROM_CE_TO_UNIX_EPOCH: i64 = DAYS_FROM_CE_TO_UNIX_EPOCH * SECONDS_PER_DAY;

#[derive(Debug, Error)]
#[error("timestamp error")]
pub struct TimestampError;

pub(crate) fn date_from_ordinal_date(ordinal_date: (i64, i64)) -> (i64, i64, i64) {
    let (year, day_of_year) = ordinal_date;
    let days_of_month_table = if is_leap_year(year) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    let mut days = 0;
    for (index, days_of_month) in days_of_month_table.iter().enumerate() {
        let month = index as i64 + 1;
        if day_of_year <= days + days_of_month {
            let day_of_month = day_of_year - days;
            return (year, month, day_of_month);
        }
        days += days_of_month;
    }
    unreachable!()
}

pub(crate) fn date_time_string_from_seconds_from_unix_epoch(
    seconds_from_unix_epoch: i64,
) -> Result<String, TimestampError> {
    let seconds_from_ce = seconds_from_unix_epoch + SECONDS_FROM_CE_TO_UNIX_EPOCH;
    let days_from_ce = seconds_from_ce / SECONDS_PER_DAY;
    let seconds_from_midnight = seconds_from_ce % SECONDS_PER_DAY;
    let ordinal_date = ordinal_date_from_days_from_ce(days_from_ce);
    let (hour, minute, second) = time_from_seconds_from_midnight(seconds_from_midnight);
    let (year, month, day_of_month) = date_from_ordinal_date(ordinal_date);
    Ok(format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
        year, month, day_of_month, hour, minute, second
    ))
}

pub(crate) fn days_from_ce_from_ordinal_date((year, day_of_year): (i64, i64)) -> i64 {
    if year == 0 {
        panic!()
    }
    days_from_ce_from_year(year - 1) + day_of_year
}

pub(crate) fn days_from_ce_from_year(y: i64) -> i64 {
    y * 365 + y / 4 - y / 100 + y / 400
}

pub(crate) fn is_leap_year(year: i64) -> bool {
    if year < 0 {
        panic!()
    }

    (year % 400 == 0) || ((year % 100 != 0) && (year % 4 == 0))
}

pub(crate) fn ordinal_date_from_date((year, month, day_of_month): (i64, i64, i64)) -> (i64, i64) {
    let days_of_month_table = if is_leap_year(year) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    let day_of_year = days_of_month_table[0..(month - 1) as usize]
        .iter()
        .sum::<i64>()
        + day_of_month;
    (year, day_of_year)
}

pub(crate) fn ordinal_date_from_days_from_ce(d: i64) -> (i64, i64) {
    if d <= 0 {
        panic!()
    }

    let d = d - 1;
    let c400 = d / DAYS_PER_400_YEARS;
    let r400 = d % DAYS_PER_400_YEARS;
    let c100 = r400 / DAYS_PER_100_YEARS;
    let r100 = r400 % DAYS_PER_100_YEARS;
    let c4 = r100 / DAYS_PER_4_YEARS;
    let r4 = r100 % DAYS_PER_4_YEARS;
    let c1 = r4 / DAYS_PER_COMMON_YEAR;
    let r1 = r4 % DAYS_PER_COMMON_YEAR;
    let is_leap = c100 == 4 || c1 == 4;
    let year = c400 * 400 + c100 * 100 + c4 * 4 + c1 + if is_leap { 0 } else { 1 };
    let day_of_year = if is_leap { DAYS_PER_COMMON_YEAR } else { r1 } + 1;
    (year, day_of_year)
}

pub(crate) fn seconds_from_midnight_from_time((h, min, s): (i64, i64, i64)) -> i64 {
    if !(0..24).contains(&h) {
        panic!()
    }
    if !(0..60).contains(&min) {
        panic!()
    }
    if !(0..60).contains(&s) {
        panic!()
    }
    (h * 60 + min) * 60 + s
}

pub(crate) fn seconds_from_unix_epoch_from_date_time_string(
    date_time_string: &str,
) -> Result<i64, TimestampError> {
    Ok(NaiveDateTime::from_str(date_time_string)
        .map_err(|_| TimestampError)?
        .timestamp())
}

pub(crate) fn time_from_seconds_from_midnight(seconds: i64) -> (i64, i64, i64) {
    if !(0..SECONDS_PER_DAY).contains(&seconds) {
        panic!()
    }
    let s = seconds % 60;
    let seconds = seconds / 60;
    let min = seconds % 60;
    let seconds = seconds / 60;
    let h = seconds % 24;
    let x = seconds / 24;
    if x != 0 {
        panic!()
    }
    (h, min, s)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use chrono::{Datelike, Timelike};

    use crate::Instant;

    use super::*;

    // #[test]
    // fn WIP_seconds_from_unix_epoch_from_date_time_string_test() -> anyhow::Result<()> {
    //     let f = seconds_from_unix_epoch_from_date_time_string;
    //     let min_timestamp = u64::from(Instant::min()) as i64;
    //     let max_timestamp = u64::from(Instant::max()) as i64;
    //     assert_eq!(f("1969-12-31T23:59:59")?, min_timestamp - 1);
    //     assert_eq!(f("1970-01-01T00:00:00")?, min_timestamp);
    //     assert_eq!(f("9999-12-31T23:59:59")?, max_timestamp);
    //     assert_eq!(f("+10000-01-01T00:00:00")?, max_timestamp + 1);
    //     Ok(())
    // }

    #[test]
    fn const_days_from_ce_to_unix_epoch_test() -> anyhow::Result<()> {
        // JavaScript
        // new Date('0000-12-31T00:00:00Z').getTime() - new Date('0000-12-31T00:00:00Z').getTime()
        // // => 0
        let days_from_ce =
            i64::from(chrono::NaiveDateTime::from_str("0000-12-31T00:00:00")?.num_days_from_ce());
        assert_eq!(days_from_ce, 0);

        // (new Date('1970-01-01T00:00:00Z').getTime() - new Date('0000-12-31T00:00:00Z').getTime()) / 86_400
        // // => 719163000
        let days_from_ce_to_unix_epoch =
            i64::from(chrono::NaiveDateTime::from_str("1970-01-01T00:00:00")?.num_days_from_ce());
        assert_eq!(days_from_ce_to_unix_epoch, 719_163_i64);

        assert_eq!(DAYS_FROM_CE_TO_UNIX_EPOCH, days_from_ce_to_unix_epoch);
        assert_eq!(DAYS_FROM_CE_TO_UNIX_EPOCH, days_from_ce_from_year(1969) + 1);
        assert_eq!(DAYS_FROM_CE_TO_UNIX_EPOCH, 719_163_i64);
        Ok(())
    }

    #[test]
    fn const_days_per_common_year_test() {
        assert_eq!(DAYS_PER_COMMON_YEAR, 365_i64);
    }

    #[test]
    fn const_days_per_4_years_test() {
        assert_eq!(DAYS_PER_4_YEARS, DAYS_PER_COMMON_YEAR * 4 + 1);
        assert_eq!(DAYS_PER_4_YEARS, 1_461_i64);
    }

    #[test]
    fn const_days_per_100_years_test() {
        assert_eq!(DAYS_PER_100_YEARS, DAYS_PER_4_YEARS * 25 - 1);
        assert_eq!(DAYS_PER_100_YEARS, 36_524_i64);
    }

    #[test]
    fn const_days_per_400_years_test() {
        assert_eq!(DAYS_PER_400_YEARS, DAYS_PER_100_YEARS * 4 + 1);
        assert_eq!(DAYS_PER_400_YEARS, 146_097_i64);
    }

    #[test]
    fn const_seconds_per_day_test() {
        assert_eq!(SECONDS_PER_DAY, 24 * 60 * 60);
        assert_eq!(SECONDS_PER_DAY, 86_400_i64);
    }

    #[test]
    fn const_seconds_from_ce_to_unix_epoch_test() -> anyhow::Result<()> {
        // JavaScript
        // new Date('0000-12-31T00:00:00Z').getTime() - new Date('0000-12-31T00:00:00Z').getTime()
        // // => 0
        let days_from_ce =
            i64::from(chrono::NaiveDateTime::from_str("0000-12-31T00:00:00")?.num_days_from_ce());
        assert_eq!(days_from_ce, 0);
        let seconds_from_ce = days_from_ce * SECONDS_PER_DAY;
        assert_eq!(seconds_from_ce, 0);

        // new Date('1970-01-01T00:00:00Z').getTime() - new Date('0000-12-31T00:00:00Z').getTime()
        // // => 62135683200000
        let days_from_ce_to_unix_epoch =
            i64::from(chrono::NaiveDateTime::from_str("1970-01-01T00:00:00")?.num_days_from_ce());
        assert_eq!(days_from_ce_to_unix_epoch, 719_163_i64);
        let seconds_from_ce_to_unix_epoch = days_from_ce_to_unix_epoch * SECONDS_PER_DAY;
        assert_eq!(seconds_from_ce_to_unix_epoch, 62_135_683_200_i64);

        assert_eq!(SECONDS_FROM_CE_TO_UNIX_EPOCH, seconds_from_ce_to_unix_epoch);
        assert_eq!(
            SECONDS_FROM_CE_TO_UNIX_EPOCH,
            (days_from_ce_from_year(1969) + 1) * SECONDS_PER_DAY
        );
        assert_eq!(SECONDS_FROM_CE_TO_UNIX_EPOCH, 62_135_683_200_i64);
        Ok(())
    }

    #[test]
    fn date_from_ordinal_date_test() {
        assert_eq!(date_from_ordinal_date((2021, 1)), (2021, 1, 1));
        assert_eq!(date_from_ordinal_date((2021, 365)), (2021, 12, 31));
        assert_eq!(date_from_ordinal_date((2000, 366)), (2000, 12, 31));
    }

    #[test]
    fn date_time_string_from_seconds_from_unix_epoch_test() -> anyhow::Result<()> {
        let f = date_time_string_from_seconds_from_unix_epoch;
        let min_timestamp = 0_i64; // 1970-01-01T00:00:00Z
        let max_timestamp = 253_402_300_799_i64; // 9999-12-31T23:59:59Z
        assert_eq!(f(min_timestamp - 1)?, "1969-12-31T23:59:59");
        assert_eq!(f(min_timestamp)?, "1970-01-01T00:00:00");
        assert_eq!(f(max_timestamp)?, "9999-12-31T23:59:59");
        assert_eq!(f(max_timestamp + 1)?, "10000-01-01T00:00:00");
        Ok(())
    }

    #[test]
    fn days_from_ce_from_ordinal_date_test() {
        // See: ordinal_date_from_days_from_ce_test
    }

    #[test]
    fn days_from_ce_from_year_test() -> anyhow::Result<()> {
        let f = days_from_ce_from_year;
        let g = |y| Datelike::num_days_from_ce(&chrono::NaiveDate::from_ymd(y as i32, 1, 1));
        assert_eq!(f(0), 0); // 0000-12-31 ... 0 d
        assert_eq!(g(1), 1); // 0001-01-01 ... 1 d
        assert_eq!(f(1), 365); // 0001-12-31 ... 365 d
        assert_eq!(g(2), 366); // 0002-01-01 ... 366 d
        assert_eq!(f(2), 730); // 0002-12-31 ... 730 d
        assert_eq!(g(3), 731); // 0003-01-01 ... 731 d
        assert_eq!(f(1969), 719162); // 1969-12-31 ... 719162 d
        assert_eq!(g(1970), 719163); // 1970-01-01 ... 719163 d
        for y in 1..=9999 + 1 {
            assert_eq!(f(y - 1) + 1, i64::from(g(y)));
        }
        Ok(())
    }

    #[test]
    fn is_leap_year_test() {
        let f = is_leap_year;
        assert!(f(2000));
        assert!(f(2004));
        assert!(!f(2100));
    }

    #[test]
    fn ordinal_date_from_date_test() {
        for d in 1..=(253_402_300_799 / 86_400) {
            let ordinal_date = ordinal_date_from_days_from_ce(d);
            let (y1, m1, d1) = date_from_ordinal_date(ordinal_date);
            let (y2, m2, d2) = {
                let naive_date = chrono::NaiveDate::from_num_days_from_ce(d as i32);
                (
                    Datelike::year(&naive_date) as i64,
                    Datelike::month(&naive_date) as i64,
                    Datelike::day(&naive_date) as i64,
                )
            };
            assert_eq!((y1, m1, d1), (y2, m2, d2));
        }
    }

    #[test]
    fn ordinal_date_from_days_from_ce_test() {
        for d in 1..=(253_402_300_799 / 86_400) {
            let (y1, d1) = ordinal_date_from_days_from_ce(d);
            let (y2, d2) = {
                let naive_date = chrono::NaiveDate::from_num_days_from_ce(d as i32);
                (
                    Datelike::year(&naive_date) as i64,
                    Datelike::ordinal(&naive_date) as i64,
                )
            };
            assert_eq!((y1, d1), (y2, d2));

            let days_from_ce = days_from_ce_from_ordinal_date((y1, d1));
            let (y3, d3) = ordinal_date_from_days_from_ce(days_from_ce);
            assert_eq!(days_from_ce, d);
            assert_eq!((y1, d1), (y3, d3));
        }
    }

    #[test]
    fn seconds_from_midnight_from_time_test() {
        for s in 0..86_400 {
            let time = chrono::NaiveTime::from_num_seconds_from_midnight(s as u32, 0);
            let t1 = (
                time.hour() as i64,
                time.minute() as i64,
                time.second() as i64,
            );
            let t2 = time_from_seconds_from_midnight(s);
            assert_eq!(t1, t2);
            assert_eq!(seconds_from_midnight_from_time(t2), s);
        }
    }

    #[test]
    fn seconds_from_unix_epoch_from_date_time_string_test() -> anyhow::Result<()> {
        let f = seconds_from_unix_epoch_from_date_time_string;
        let min_timestamp = u64::from(Instant::min()) as i64;
        let max_timestamp = u64::from(Instant::max()) as i64;
        assert_eq!(f("1969-12-31T23:59:59")?, min_timestamp - 1);
        assert_eq!(f("1970-01-01T00:00:00")?, min_timestamp);
        assert_eq!(f("9999-12-31T23:59:59")?, max_timestamp);
        assert_eq!(f("+10000-01-01T00:00:00")?, max_timestamp + 1);
        Ok(())
    }

    #[test]
    fn time_from_seconds_from_midnight_test() {
        // See: seconds_from_midnight_from_time_test
    }
}
