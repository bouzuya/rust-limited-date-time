use std::convert::TryFrom;
use thiserror::Error;

use crate::Days;

// [1, 366]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DayOfYear(u16);

impl DayOfYear {
    pub fn max() -> DayOfYear {
        Self::max_in_leap_year()
    }

    pub fn max_in_common_year() -> DayOfYear {
        DayOfYear(365)
    }

    pub fn max_in_leap_year() -> DayOfYear {
        DayOfYear(366)
    }

    pub fn min() -> DayOfYear {
        DayOfYear(1)
    }

    pub fn days(&self) -> Days {
        Days::from(1_u8)
    }

    pub fn pred(&self) -> Option<Self> {
        if self.0 > 1 {
            Some(Self(self.0 - 1))
        } else {
            None
        }
    }

    pub fn succ(&self) -> Option<Self> {
        if self.0 < 366 {
            Some(Self(self.0 + 1))
        } else {
            None
        }
    }
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum ParseDayOfYearError {
    #[error("invalid digit")]
    InvalidDigit,
    #[error("invalid length")]
    InvalidLength,
    #[error("out of range")]
    OutOfRange,
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum TryFromDayOfYearError {
    #[error("out of range")]
    OutOfRange,
}

impl std::fmt::Display for DayOfYear {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:03}", self.0)
    }
}

impl std::str::FromStr for DayOfYear {
    type Err = ParseDayOfYearError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 3 {
            return Err(Self::Err::InvalidLength);
        }
        let mut doy = 0_u16;
        for c in s.chars() {
            let d = match c {
                '0'..='9' => c as u8 - b'0',
                _ => return Err(Self::Err::InvalidDigit),
            };
            doy = doy * 10 + d as u16;
        }
        Self::try_from(doy).map_err(|_| Self::Err::OutOfRange)
    }
}

impl From<DayOfYear> for i16 {
    fn from(day_of_year: DayOfYear) -> Self {
        i16::try_from(day_of_year.0).expect("day_of_year is [1,366]")
    }
}

impl From<DayOfYear> for i32 {
    fn from(day_of_year: DayOfYear) -> Self {
        i32::from(day_of_year.0)
    }
}

impl From<DayOfYear> for i64 {
    fn from(day_of_year: DayOfYear) -> Self {
        i64::from(day_of_year.0)
    }
}

impl From<DayOfYear> for u16 {
    fn from(day_of_year: DayOfYear) -> Self {
        day_of_year.0
    }
}

impl From<DayOfYear> for u32 {
    fn from(day_of_year: DayOfYear) -> Self {
        u32::from(day_of_year.0)
    }
}

impl From<DayOfYear> for u64 {
    fn from(day_of_year: DayOfYear) -> Self {
        u64::from(day_of_year.0)
    }
}

impl std::convert::TryFrom<DayOfYear> for i8 {
    type Error = TryFromDayOfYearError;

    fn try_from(value: DayOfYear) -> Result<Self, Self::Error> {
        i8::try_from(value.0).map_err(|_| Self::Error::OutOfRange)
    }
}

impl std::convert::TryFrom<DayOfYear> for u8 {
    type Error = TryFromDayOfYearError;

    fn try_from(value: DayOfYear) -> Result<Self, Self::Error> {
        u8::try_from(value.0).map_err(|_| Self::Error::OutOfRange)
    }
}

impl std::convert::TryFrom<i8> for DayOfYear {
    type Error = TryFromDayOfYearError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let value_as_u16 = u16::try_from(value).map_err(|_| Self::Error::OutOfRange)?;
        DayOfYear::try_from(value_as_u16)
    }
}

impl std::convert::TryFrom<i16> for DayOfYear {
    type Error = TryFromDayOfYearError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let value_as_u16 = u16::try_from(value).map_err(|_| Self::Error::OutOfRange)?;
        DayOfYear::try_from(value_as_u16)
    }
}

impl std::convert::TryFrom<i32> for DayOfYear {
    type Error = TryFromDayOfYearError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let value_as_u16 = u16::try_from(value).map_err(|_| Self::Error::OutOfRange)?;
        DayOfYear::try_from(value_as_u16)
    }
}

impl std::convert::TryFrom<i64> for DayOfYear {
    type Error = TryFromDayOfYearError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let value_as_u16 = u16::try_from(value).map_err(|_| Self::Error::OutOfRange)?;
        DayOfYear::try_from(value_as_u16)
    }
}

impl std::convert::TryFrom<u8> for DayOfYear {
    type Error = TryFromDayOfYearError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let value_as_u16 = u16::from(value);
        DayOfYear::try_from(value_as_u16)
    }
}

impl std::convert::TryFrom<u16> for DayOfYear {
    type Error = TryFromDayOfYearError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if !(1..=366).contains(&value) {
            return Err(Self::Error::OutOfRange);
        }
        Ok(Self(value))
    }
}

impl std::convert::TryFrom<u32> for DayOfYear {
    type Error = TryFromDayOfYearError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let value_as_u16 = u16::try_from(value).map_err(|_| Self::Error::OutOfRange)?;
        DayOfYear::try_from(value_as_u16)
    }
}

impl std::convert::TryFrom<u64> for DayOfYear {
    type Error = TryFromDayOfYearError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let value_as_u16 = u16::try_from(value).map_err(|_| Self::Error::OutOfRange)?;
        DayOfYear::try_from(value_as_u16)
    }
}

impl std::ops::Add<Days> for DayOfYear {
    type Output = DayOfYear;

    fn add(self, rhs: Days) -> Self::Output {
        u32::from(self.0)
            .checked_add(u32::from(rhs))
            .and_then(|d| u16::try_from(d).ok())
            .and_then(|d| DayOfYear::try_from(d).ok())
            .unwrap_or_else(|| panic!("overflow"))
    }
}

impl std::ops::Add<DayOfYear> for Days {
    type Output = DayOfYear;

    fn add(self, rhs: DayOfYear) -> Self::Output {
        rhs + self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_conversion_test() {
        type E = ParseDayOfYearError;
        let f = |s: &str| s.parse::<DayOfYear>();
        assert_eq!(f("001").map(|d| d.to_string()), Ok("001".to_string()));
        assert_eq!(f("366").map(|d| d.to_string()), Ok("366".to_string()));
        assert_eq!(f(""), Err(E::InvalidLength));
        assert_eq!(f("1"), Err(E::InvalidLength));
        assert_eq!(f("12"), Err(E::InvalidLength));
        assert_eq!(f("1234"), Err(E::InvalidLength));
        assert_eq!(f("00a"), Err(E::InvalidDigit));
        assert_eq!(f("+12"), Err(E::InvalidDigit));
        assert_eq!(f("000"), Err(E::OutOfRange));
        assert_eq!(f("367"), Err(E::OutOfRange));
    }

    #[test]
    fn i8_conversion_test() {
        type E = TryFromDayOfYearError;
        let f = |d: i8| DayOfYear::try_from(d);
        assert_eq!(f(0_i8), Err(E::OutOfRange));
        assert_eq!(f(1_i8).and_then(i8::try_from), Ok(1_i8));
        assert_eq!(f(i8::MAX).and_then(i8::try_from), Ok(127_i8));
    }

    #[test]
    fn i16_conversion_test() {
        type E = TryFromDayOfYearError;
        let f = |d: i16| DayOfYear::try_from(d);
        assert_eq!(f(0_i16), Err(E::OutOfRange));
        assert_eq!(f(1_i16).map(i16::from), Ok(1_i16));
        assert_eq!(f(366_i16).map(i16::from), Ok(366_i16));
        assert_eq!(f(367_i16), Err(E::OutOfRange));
    }

    #[test]
    fn i32_conversion_test() {
        type E = TryFromDayOfYearError;
        let f = |d: i32| DayOfYear::try_from(d);
        assert_eq!(f(0_i32), Err(E::OutOfRange));
        assert_eq!(f(1_i32).map(i32::from), Ok(1_i32));
        assert_eq!(f(366_i32).map(i32::from), Ok(366_i32));
        assert_eq!(f(367_i32), Err(E::OutOfRange));
    }

    #[test]
    fn i64_conversion_test() {
        type E = TryFromDayOfYearError;
        let f = |d: i64| DayOfYear::try_from(d);
        assert_eq!(f(0_i64), Err(E::OutOfRange));
        assert_eq!(f(1_i64).map(i64::from), Ok(1_i64));
        assert_eq!(f(366_i64).map(i64::from), Ok(366_i64));
        assert_eq!(f(367_i64), Err(E::OutOfRange));
    }

    #[test]
    fn u8_conversion_test() {
        type E = TryFromDayOfYearError;
        let f = |d: u8| DayOfYear::try_from(d);
        assert_eq!(f(0_u8), Err(E::OutOfRange));
        assert_eq!(f(1_u8).and_then(u8::try_from), Ok(1_u8));
        assert_eq!(f(u8::MAX).and_then(u8::try_from), Ok(255_u8));
    }

    #[test]
    fn u16_conversion_test() {
        type E = TryFromDayOfYearError;
        let f = |d: u16| DayOfYear::try_from(d);
        assert_eq!(f(0_u16), Err(E::OutOfRange));
        assert_eq!(f(1_u16).map(u16::from), Ok(1_u16));
        assert_eq!(f(366_u16).map(u16::from), Ok(366_u16));
        assert_eq!(f(367_u16), Err(E::OutOfRange));
    }

    #[test]
    fn u32_conversion_test() {
        type E = TryFromDayOfYearError;
        let f = |d: u32| DayOfYear::try_from(d);
        assert_eq!(f(0_u32), Err(E::OutOfRange));
        assert_eq!(f(1_u32).map(u32::from), Ok(1_u32));
        assert_eq!(f(366_u32).map(u32::from), Ok(366_u32));
        assert_eq!(f(367_u32), Err(E::OutOfRange));
    }

    #[test]
    fn u64_conversion_test() {
        type E = TryFromDayOfYearError;
        let f = |d: u64| DayOfYear::try_from(d);
        assert_eq!(f(0_u64), Err(E::OutOfRange));
        assert_eq!(f(1_u64).map(u64::from), Ok(1_u64));
        assert_eq!(f(366_u64).map(u64::from), Ok(366_u64));
        assert_eq!(f(367_u64), Err(E::OutOfRange));
    }

    #[test]
    fn max_min_test() -> anyhow::Result<()> {
        assert_eq!(DayOfYear::max(), DayOfYear::try_from(366)?);
        assert_eq!(DayOfYear::max_in_common_year(), DayOfYear::try_from(365)?);
        assert_eq!(DayOfYear::max_in_leap_year(), DayOfYear::try_from(366)?);
        assert_eq!(DayOfYear::min(), DayOfYear::try_from(1)?);
        Ok(())
    }

    #[test]
    fn pred_test() -> anyhow::Result<()> {
        assert_eq!(
            DayOfYear::try_from(366)?.pred(),
            Some(DayOfYear::try_from(365)?)
        );
        assert_eq!(
            DayOfYear::try_from(2)?.pred(),
            Some(DayOfYear::try_from(1)?)
        );
        assert_eq!(DayOfYear::try_from(1)?.pred(), None);
        Ok(())
    }

    #[test]
    fn succ_test() -> anyhow::Result<()> {
        assert_eq!(
            DayOfYear::try_from(1)?.succ(),
            Some(DayOfYear::try_from(2)?)
        );
        assert_eq!(
            DayOfYear::try_from(365)?.succ(),
            Some(DayOfYear::try_from(366)?)
        );
        assert_eq!(DayOfYear::try_from(366)?.succ(), None);
        Ok(())
    }

    #[test]
    fn add_days_test() -> anyhow::Result<()> {
        let d1 = DayOfYear::try_from(1)?;
        let d2 = DayOfYear::try_from(2)?;
        let d366 = DayOfYear::try_from(366)?;
        assert_eq!(d1 + Days::from(0_u16), d1);
        assert_eq!(d1 + Days::from(1_u16), d2);
        assert_eq!(d1 + Days::from(365_u16), d366);
        // should_panic
        // assert_eq!(d1 + Days::from(366), d31);
        assert_eq!(Days::from(0_u16) + d1, d1);
        assert_eq!(Days::from(1_u16) + d1, d2);
        assert_eq!(Days::from(365_u16) + d1, d366);
        Ok(())
    }

    #[test]
    fn days_test() -> anyhow::Result<()> {
        assert_eq!(DayOfYear::try_from(1)?.days(), Days::from(1_u16));
        Ok(())
    }
}
