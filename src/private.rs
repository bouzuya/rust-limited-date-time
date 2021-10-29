use std::str::FromStr;

use chrono::NaiveDateTime;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("timestamp error")]
pub struct TimestampError;

pub(crate) fn date_time_string_from_seconds_from_unix_epoch(
    timestamp: i64,
) -> Result<String, TimestampError> {
    let naive_date_time = NaiveDateTime::from_timestamp(timestamp, 0);
    Ok(format!("{:?}", naive_date_time))
}

pub(crate) fn seconds_from_unix_epoch_from_date_time_string(
    date_time_string: &str,
) -> Result<i64, TimestampError> {
    Ok(NaiveDateTime::from_str(date_time_string)
        .map_err(|_| TimestampError)?
        .timestamp())
}

pub(crate) fn days_from_ce_from_year(y: i64) -> i64 {
    y * 365 + y / 4 - y / 100 + y / 400
}

pub(crate) fn days_from_ce_from_ordinal_date((year, day_of_year): (i64, i64)) -> i64 {
    if year == 0 {
        panic!()
    }
    days_from_ce_from_year(year - 1) + day_of_year
}

pub(crate) fn ordinal_date_from_days_from_ce(d: i64) -> (i64, i64) {
    if d <= 0 {
        panic!()
    }

    let d = d - 1;
    let days_of_year = 365;
    let days_of_4_years = days_of_year * 4 + 1;
    let days_of_100_years = days_of_4_years * 25 - 1;
    let days_of_400_years = days_of_100_years * 4 + 1;
    let c400 = d / days_of_400_years;
    let r400 = d % days_of_400_years;
    let c100 = r400 / days_of_100_years;
    let r100 = r400 % days_of_100_years;
    let c4 = r100 / days_of_4_years;
    let r4 = r100 % days_of_4_years;
    let c1 = r4 / days_of_year;
    let r1 = r4 % days_of_year;
    let is_leap = c100 == 4 || c1 == 4;
    let year = c400 * 400 + c100 * 100 + c4 * 4 + c1 + if is_leap { 0 } else { 1 };
    let day_of_year = if is_leap { days_of_year } else { r1 } + 1;
    (year, day_of_year)
}

#[cfg(test)]
mod tests {
    use crate::Instant;

    use super::*;

    #[test]
    fn ordinal_date_from_days_from_ce_test() {
        for d in 1..=(253_402_300_799 / 86_400) {
            let (y1, d1) = ordinal_date_from_days_from_ce(d);
            let (y2, d2) = {
                let naive_date = chrono::NaiveDate::from_num_days_from_ce(d as i32);
                (
                    chrono::Datelike::year(&naive_date) as i64,
                    chrono::Datelike::ordinal(&naive_date) as i64,
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
    fn date_time_string_from_seconds_from_unix_epoch_test() -> anyhow::Result<()> {
        let f = date_time_string_from_seconds_from_unix_epoch;
        let min_timestamp = u64::from(Instant::min()) as i64;
        let max_timestamp = u64::from(Instant::max()) as i64;
        assert_eq!(f(min_timestamp - 1)?, "1969-12-31T23:59:59");
        assert_eq!(f(min_timestamp)?, "1970-01-01T00:00:00");
        assert_eq!(f(max_timestamp)?, "9999-12-31T23:59:59");
        assert_eq!(f(max_timestamp + 1)?, "+10000-01-01T00:00:00");
        Ok(())
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
    fn days_from_ce_from_year_test() -> anyhow::Result<()> {
        let f = days_from_ce_from_year;
        let g =
            |y| chrono::Datelike::num_days_from_ce(&chrono::NaiveDate::from_ymd(y as i32, 1, 1));
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
}
