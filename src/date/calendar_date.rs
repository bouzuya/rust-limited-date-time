use crate::private::days_from_unix_epoch_from_date;
use crate::Days;

use std::convert::TryFrom;

use super::day_of_month::{DayOfMonth, ParseDayOfMonthError};
use super::day_of_year::DayOfYear;
use super::month::{Month, ParseMonthError};
use super::ordinal_date::OrdinalDate;
use super::year::{ParseYearError, Year};
use super::year_month::{ParseYearMonthError, YearMonth};
use thiserror::Error;

pub type Date = CalendarDate;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CalendarDate {
    year: Year,
    month: Month,
    day_of_month: DayOfMonth,
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum ParseDateError {
    #[error("invalid day of month")]
    InvalidDayOfMonth,
    #[error("invalid format")]
    InvalidFormat,
    #[error("invalid length")]
    InvalidLength,
    #[error("parse day of month")]
    ParseDayOfMonth(ParseDayOfMonthError),
    #[error("parse month")]
    ParseMonth(ParseMonthError),
    #[error("parse year")]
    ParseYear(ParseYearError),
}

#[derive(Debug, Eq, Error, PartialEq)]
#[error("invalid date error")]
pub struct InvalidDateError;

impl CalendarDate {
    pub fn first_date_of_month(year_month: YearMonth) -> Self {
        Self {
            year: year_month.year(),
            month: year_month.month(),
            day_of_month: year_month.first_day_of_month(),
        }
    }

    pub fn first_date_of_year(year: Year) -> Self {
        let year_month = YearMonth::first_year_month_of_year(year);
        Self::first_date_of_month(year_month)
    }

    pub fn last_date_of_month(year_month: YearMonth) -> Self {
        Self {
            year: year_month.year(),
            month: year_month.month(),
            day_of_month: year_month.last_day_of_month(),
        }
    }

    pub fn last_date_of_year(year: Year) -> Self {
        let year_month = YearMonth::last_year_month_of_year(year);
        Self::last_date_of_month(year_month)
    }

    pub fn from_ymd(
        year: Year,
        month: Month,
        day_of_month: DayOfMonth,
    ) -> Result<Self, InvalidDateError> {
        let year_month = YearMonth::new(year, month);
        if day_of_month > year_month.last_day_of_month() {
            return Err(InvalidDateError);
        }
        Ok(Self {
            year,
            month,
            day_of_month,
        })
    }

    pub fn day_of_month(&self) -> DayOfMonth {
        self.day_of_month
    }

    pub fn month(&self) -> Month {
        self.month
    }

    pub fn year(&self) -> Year {
        self.year
    }

    pub fn year_month(&self) -> YearMonth {
        YearMonth::new(self.year, self.month)
    }

    pub fn pred(&self) -> Option<Self> {
        if self.day_of_month() == self.year_month().first_day_of_month() {
            self.year_month().pred().map(Self::last_date_of_month)
        } else {
            self.day_of_month().pred().and_then(|next_day_of_month| {
                CalendarDate::from_ymd(self.year(), self.month(), next_day_of_month).ok()
            })
        }
    }

    pub fn succ(&self) -> Option<Self> {
        if self.day_of_month() == self.year_month().last_day_of_month() {
            self.year_month().succ().map(Self::first_date_of_month)
        } else {
            self.day_of_month().succ().and_then(|next_day_of_month| {
                CalendarDate::from_ymd(self.year(), self.month(), next_day_of_month).ok()
            })
        }
    }

    // UTC ???????????? Date ??????????????? 1970-01-01 ??????????????????????????????
    pub(crate) fn days_from_unix_epoch(&self) -> Days {
        let days_from_unix_epoch = days_from_unix_epoch_from_date((
            i64::from(self.year()),
            i64::from(self.month()),
            i64::from(self.day_of_month()),
        ));
        let days_from_unix_epoch_as_u32 =
            u32::try_from(days_from_unix_epoch).expect("days from unix epoch is [0, 2_932_896]");
        Days::try_from(days_from_unix_epoch_as_u32).expect("Days supports [0, 2_932_896]")
    }
}

impl std::fmt::Display for CalendarDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}-{}", self.year, self.month, self.day_of_month)
    }
}

impl std::str::FromStr for CalendarDate {
    type Err = ParseDateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 10 {
            return Err(Self::Err::InvalidLength);
        }
        let year_month = match YearMonth::from_str(&s[0..7]) {
            Ok(ym) => ym,
            Err(e) => match e {
                ParseYearMonthError::InvalidLength => unreachable!(),
                ParseYearMonthError::InvalidFormat => return Err(Self::Err::InvalidFormat),
                ParseYearMonthError::ParseYear(e) => return Err(Self::Err::ParseYear(e)),
                ParseYearMonthError::ParseMonth(e) => return Err(Self::Err::ParseMonth(e)),
            },
        };
        if s.as_bytes().get(7) != Some(&b'-') {
            return Err(Self::Err::InvalidFormat);
        }
        let day_of_month = match DayOfMonth::from_str(&s[8..10]) {
            Ok(d) => d,
            Err(e) => return Err(Self::Err::ParseDayOfMonth(e)),
        };
        if day_of_month > year_month.last_day_of_month() {
            return Err(Self::Err::InvalidDayOfMonth);
        }
        Ok(CalendarDate {
            year: year_month.year(),
            month: year_month.month(),
            day_of_month,
        })
    }
}

impl From<CalendarDate> for OrdinalDate {
    fn from(date: CalendarDate) -> Self {
        let year = date.year();
        let mut days = 0_u16;
        // TODO: impl Iterator for Range<Month>
        for m in u8::from(Month::january())..u8::from(date.month()) {
            let m = Month::try_from(m).unwrap();
            let year_month = YearMonth::new(year, m);
            days += u16::try_from(u32::from(year_month.days()))
                .expect("sum of year_month.days() in year <= 366");
        }
        days += u16::from(date.day_of_month());
        let day_of_year =
            DayOfYear::try_from(days).expect("sum of year_month.days() in year <= 366");
        OrdinalDate::new(year, day_of_year).expect("CalendarDate is broken")
    }
}

impl From<OrdinalDate> for CalendarDate {
    fn from(ordinal_date: OrdinalDate) -> Self {
        let year = ordinal_date.year();
        let day_of_year = u16::from(ordinal_date.day_of_year());
        let mut days = 0_u16;
        // TODO: impl Iterator for Range<Month>
        for m in u8::from(Month::january())..=u8::from(Month::december()) {
            let m = Month::try_from(m).unwrap();
            let year_month = YearMonth::new(year, m);
            let days_of_month = u16::try_from(u32::from(year_month.days()))
                .expect("sum of year_month.days() in year <= 366");
            if day_of_year <= days + days_of_month {
                let month = m;
                let day_of_month = u8::try_from(day_of_year - days).expect("day_of_year - days");
                let day_of_month =
                    DayOfMonth::try_from(day_of_month).expect("DayOfMonth::try_from");
                return CalendarDate::from_ymd(year, month, day_of_month)
                    .expect("From<OrdinalDate> for Date");
            }
            days += days_of_month;
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn first_date_of_month_test() -> anyhow::Result<()> {
        let year_month = YearMonth::from_str("2021-01")?;
        assert_eq!(
            CalendarDate::first_date_of_month(year_month).to_string(),
            "2021-01-01"
        );
        Ok(())
    }

    #[test]
    fn first_date_of_year_test() -> anyhow::Result<()> {
        let year = Year::from_str("2021")?;
        assert_eq!(
            CalendarDate::first_date_of_year(year).to_string(),
            "2021-01-01"
        );
        Ok(())
    }

    #[test]
    fn last_date_of_month_test() -> anyhow::Result<()> {
        let year_month = YearMonth::from_str("2021-01")?;
        assert_eq!(
            CalendarDate::last_date_of_month(year_month).to_string(),
            "2021-01-31"
        );
        let year_month = YearMonth::from_str("2021-02")?;
        assert_eq!(
            CalendarDate::last_date_of_month(year_month).to_string(),
            "2021-02-28"
        );
        Ok(())
    }

    #[test]
    fn last_date_of_year_test() -> anyhow::Result<()> {
        let year = Year::from_str("2021")?;
        assert_eq!(
            CalendarDate::last_date_of_year(year).to_string(),
            "2021-12-31"
        );
        Ok(())
    }

    #[test]
    fn from_ymd_test() -> anyhow::Result<()> {
        assert_eq!(
            CalendarDate::from_ymd(
                Year::from_str("2021")?,
                Month::from_str("02")?,
                DayOfMonth::from_str("03")?
            )?,
            CalendarDate::from_str("2021-02-03")?
        );
        assert!(matches!(
            CalendarDate::from_ymd(
                Year::from_str("2021")?,
                Month::from_str("02")?,
                DayOfMonth::from_str("31")?
            ),
            Err(InvalidDateError)
        ));
        Ok(())
    }

    #[test]
    fn str_conversion_test() {
        type E = ParseDateError;
        let f = |s: &str| CalendarDate::from_str(s);

        assert!(matches!(f("2021-01-02"), Ok(_)));
        assert!(matches!(f("20021-01-02"), Err(E::InvalidLength)));
        assert!(matches!(f("2021+01-02"), Err(E::InvalidFormat)));
        assert!(matches!(f("2021-01+02"), Err(E::InvalidFormat)));
        assert!(matches!(f("+001-01-02"), Err(E::ParseYear(_))));
        assert!(matches!(f("2021-13-02"), Err(E::ParseMonth(_))));
        assert!(matches!(f("2021-01-32"), Err(E::ParseDayOfMonth(_))));
        assert!(matches!(f("2021-02-29"), Err(E::InvalidDayOfMonth)));

        assert_eq!(
            f("2021-01-02").map(|d| d.to_string()),
            Ok("2021-01-02".to_string())
        );
    }

    #[test]
    fn day_of_month_test() -> anyhow::Result<()> {
        let d = CalendarDate::from_str("2021-01-02")?;
        assert_eq!(d.day_of_month(), DayOfMonth::from_str("02")?);
        Ok(())
    }

    #[test]
    fn month_test() -> anyhow::Result<()> {
        let d = CalendarDate::from_str("2021-01-02")?;
        assert_eq!(d.month(), Month::from_str("01")?);
        Ok(())
    }

    #[test]
    fn year_test() -> anyhow::Result<()> {
        let d = CalendarDate::from_str("2021-01-02")?;
        assert_eq!(d.year(), Year::from_str("2021")?);
        Ok(())
    }

    #[test]
    fn year_month_test() -> anyhow::Result<()> {
        let d = CalendarDate::from_str("2021-01-02")?;
        assert_eq!(d.year_month(), YearMonth::from_str("2021-01")?);
        Ok(())
    }

    #[test]
    fn pred_test() -> anyhow::Result<()> {
        assert_eq!(
            CalendarDate::from_str("9999-12-31")?.pred(),
            Some(CalendarDate::from_str("9999-12-30")?)
        );
        assert_eq!(
            CalendarDate::from_str("1971-01-01")?.pred(),
            Some(CalendarDate::from_str("1970-12-31")?)
        );
        assert_eq!(
            CalendarDate::from_str("1970-12-01")?.pred(),
            Some(CalendarDate::from_str("1970-11-30")?)
        );
        assert_eq!(
            CalendarDate::from_str("1970-01-02")?.pred(),
            Some(CalendarDate::from_str("1970-01-01")?)
        );
        assert_eq!(CalendarDate::from_str("1970-01-01")?.pred(), None);
        Ok(())
    }

    #[test]
    fn succ_test() -> anyhow::Result<()> {
        assert_eq!(
            CalendarDate::from_str("1970-01-01")?.succ(),
            Some(CalendarDate::from_str("1970-01-02")?)
        );
        assert_eq!(
            CalendarDate::from_str("9998-12-31")?.succ(),
            Some(CalendarDate::from_str("9999-01-01")?)
        );
        assert_eq!(
            CalendarDate::from_str("9999-01-31")?.succ(),
            Some(CalendarDate::from_str("9999-02-01")?)
        );
        assert_eq!(CalendarDate::from_str("9999-12-31")?.succ(), None);
        Ok(())
    }

    #[test]
    fn days_from_unix_epoch_test() -> anyhow::Result<()> {
        assert_eq!(
            CalendarDate::from_str("1970-01-01")?.days_from_unix_epoch(),
            Days::from(0_u8)
        );
        assert_eq!(
            CalendarDate::from_str("1970-01-02")?.days_from_unix_epoch(),
            Days::from(1_u8)
        );
        assert_eq!(
            CalendarDate::from_str("9999-12-31")?.days_from_unix_epoch(),
            Days::try_from(2_932_896_u32)?
        );
        Ok(())
    }

    #[test]
    fn date_conversion_test() -> anyhow::Result<()> {
        assert_eq!(
            OrdinalDate::from(CalendarDate::from_str("2021-01-01")?),
            OrdinalDate::from_str("2021-001")?
        );
        assert_eq!(
            CalendarDate::from(OrdinalDate::from(CalendarDate::from_str("2021-01-01")?)),
            CalendarDate::from_str("2021-01-01")?,
        );
        assert_eq!(
            OrdinalDate::from(CalendarDate::from_str("2021-12-31")?),
            OrdinalDate::from_str("2021-365")?
        );
        assert_eq!(
            CalendarDate::from(OrdinalDate::from(CalendarDate::from_str("2021-12-31")?)),
            CalendarDate::from_str("2021-12-31")?,
        );
        Ok(())
    }
}
