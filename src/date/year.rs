use std::convert::TryFrom;
use thiserror::Error;

use crate::{DayOfYear, Days};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Year(u16);

impl Year {
    pub fn first_day_of_year(&self) -> DayOfYear {
        DayOfYear::min()
    }

    pub fn last_day_of_year(&self) -> DayOfYear {
        if self.is_leap_year() {
            DayOfYear::max_in_leap_year()
        } else {
            DayOfYear::max_in_common_year()
        }
    }

    pub fn days(&self) -> Days {
        Days::from(if self.is_leap_year() { 366 } else { 365 })
    }

    pub fn pred(&self) -> Option<Self> {
        if self.0 > 1970 {
            Some(Self(self.0 - 1))
        } else {
            None
        }
    }

    pub fn succ(&self) -> Option<Self> {
        if self.0 < 9999 {
            Some(Self(self.0 + 1))
        } else {
            None
        }
    }
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum ParseYearError {
    #[error("invalid digit")]
    InvalidDigit,
    #[error("invalid length")]
    InvalidLength,
    #[error("out of range")]
    OutOfRange,
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum TryFromYearError {
    #[error("out of range")]
    OutOfRange,
}

impl Year {
    pub fn is_leap_year(&self) -> bool {
        (self.0 % 400 == 0) || ((self.0 % 100 != 0) && (self.0 % 4 == 0))
    }
}

impl std::fmt::Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}", self.0)
    }
}

impl std::str::FromStr for Year {
    type Err = ParseYearError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(Self::Err::InvalidLength);
        }
        let mut y = 0_u16;
        for c in s.chars() {
            let d = match c {
                '0'..='9' => c as u16 - '0' as u16,
                _ => return Err(Self::Err::InvalidDigit),
            };
            y = y * 10 + d;
        }
        Self::try_from(y).map_err(|_| Self::Err::OutOfRange)
    }
}

impl From<Year> for i16 {
    fn from(year: Year) -> Self {
        i16::try_from(year.0).expect("year.0 is [1970,9999]")
    }
}

impl From<Year> for i32 {
    fn from(year: Year) -> Self {
        i32::from(year.0)
    }
}

impl From<Year> for i64 {
    fn from(year: Year) -> Self {
        i64::from(year.0)
    }
}

impl From<Year> for u16 {
    fn from(year: Year) -> Self {
        year.0
    }
}

impl From<Year> for u32 {
    fn from(year: Year) -> Self {
        u32::from(year.0)
    }
}

impl From<Year> for u64 {
    fn from(year: Year) -> Self {
        u64::from(year.0)
    }
}

impl std::convert::TryFrom<i16> for Year {
    type Error = TryFromYearError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let value_as_u16 = u16::try_from(value).map_err(|_| TryFromYearError::OutOfRange)?;
        Year::try_from(value_as_u16)
    }
}

impl std::convert::TryFrom<i32> for Year {
    type Error = TryFromYearError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let value_as_u16 = u16::try_from(value).map_err(|_| TryFromYearError::OutOfRange)?;
        Year::try_from(value_as_u16)
    }
}

impl std::convert::TryFrom<i64> for Year {
    type Error = TryFromYearError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let value_as_u16 = u16::try_from(value).map_err(|_| TryFromYearError::OutOfRange)?;
        Year::try_from(value_as_u16)
    }
}

impl std::convert::TryFrom<u16> for Year {
    type Error = TryFromYearError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if !(1970..=9999).contains(&value) {
            return Err(Self::Error::OutOfRange);
        }
        Ok(Self(value))
    }
}

impl std::convert::TryFrom<u32> for Year {
    type Error = TryFromYearError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let value_as_u16 = u16::try_from(value).map_err(|_| TryFromYearError::OutOfRange)?;
        Year::try_from(value_as_u16)
    }
}

impl std::convert::TryFrom<u64> for Year {
    type Error = TryFromYearError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let value_as_u16 = u16::try_from(value).map_err(|_| TryFromYearError::OutOfRange)?;
        Year::try_from(value_as_u16)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn is_leap_year() {
        let f = |y: u16| Year::try_from(y).map(|y| Year::is_leap_year(&y));
        assert_eq!(f(2000), Ok(true));
        assert_eq!(f(2004), Ok(true));
        assert_eq!(f(2100), Ok(false));
    }

    #[test]
    fn str_conversion_test() {
        type E = ParseYearError;
        let f = |s: &str| s.parse::<Year>();
        assert_eq!(f("1970").map(|y| y.to_string()), Ok("1970".to_string()));
        assert_eq!(f("9999").map(|y| y.to_string()), Ok("9999".to_string()));
        assert_eq!(f(""), Err(E::InvalidLength));
        assert_eq!(f("0"), Err(E::InvalidLength));
        assert_eq!(f("10000"), Err(E::InvalidLength));
        assert_eq!(f("000a"), Err(E::InvalidDigit));
        assert_eq!(f("+000"), Err(E::InvalidDigit));
        assert_eq!(f("1969"), Err(E::OutOfRange));
    }

    #[test]
    fn i16_conversion_test() {
        type E = TryFromYearError;
        let f = |y: i16| Year::try_from(y);
        assert_eq!(f(1969_i16), Err(E::OutOfRange));
        assert_eq!(f(1970_i16).map(i16::from), Ok(1970_i16));
        assert_eq!(f(9999_i16).map(i16::from), Ok(9999_i16));
        assert_eq!(f(10000_i16), Err(E::OutOfRange));
    }

    #[test]
    fn i32_conversion_test() {
        type E = TryFromYearError;
        let f = |y: i32| Year::try_from(y);
        assert_eq!(f(1969_i32), Err(E::OutOfRange));
        assert_eq!(f(1970_i32).map(i32::from), Ok(1970_i32));
        assert_eq!(f(9999_i32).map(i32::from), Ok(9999_i32));
        assert_eq!(f(10000_i32), Err(E::OutOfRange));
    }

    #[test]
    fn i64_conversion_test() {
        type E = TryFromYearError;
        let f = |y: i64| Year::try_from(y);
        assert_eq!(f(1969_i64), Err(E::OutOfRange));
        assert_eq!(f(1970_i64).map(i64::from), Ok(1970_i64));
        assert_eq!(f(9999_i64).map(i64::from), Ok(9999_i64));
        assert_eq!(f(10000_i64), Err(E::OutOfRange));
    }

    #[test]
    fn u16_conversion_test() {
        type E = TryFromYearError;
        let f = |y: u16| Year::try_from(y);
        assert_eq!(f(1969_u16), Err(E::OutOfRange));
        assert_eq!(f(1970_u16).map(u16::from), Ok(1970_u16));
        assert_eq!(f(9999_u16).map(u16::from), Ok(9999_u16));
        assert_eq!(f(10000_u16), Err(E::OutOfRange));
    }

    #[test]
    fn u32_conversion_test() {
        type E = TryFromYearError;
        let f = |y: u32| Year::try_from(y);
        assert_eq!(f(1969_u32), Err(E::OutOfRange));
        assert_eq!(f(1970_u32).map(u32::from), Ok(1970_u32));
        assert_eq!(f(9999_u32).map(u32::from), Ok(9999_u32));
        assert_eq!(f(10000_u32), Err(E::OutOfRange));
    }

    #[test]
    fn u64_conversion_test() {
        type E = TryFromYearError;
        let f = |y: u64| Year::try_from(y);
        assert_eq!(f(1969_u64), Err(E::OutOfRange));
        assert_eq!(f(1970_u64).map(u64::from), Ok(1970_u64));
        assert_eq!(f(9999_u64).map(u64::from), Ok(9999_u64));
        assert_eq!(f(10000_u64), Err(E::OutOfRange));
    }

    #[test]
    fn first_day_of_year_test() -> anyhow::Result<()> {
        assert_eq!(
            Year::from_str("2000")?.first_day_of_year(),
            DayOfYear::min()
        );
        Ok(())
    }

    #[test]
    fn last_day_of_year_test() -> anyhow::Result<()> {
        let leap_year = Year::from_str("2000")?;
        assert_eq!(leap_year.last_day_of_year(), DayOfYear::max_in_leap_year());
        let common_year = Year::from_str("2001")?;
        assert_eq!(
            common_year.last_day_of_year(),
            DayOfYear::max_in_common_year()
        );
        Ok(())
    }

    #[test]
    fn pred_test() -> anyhow::Result<()> {
        assert_eq!(
            Year::from_str("9999")?.pred(),
            Some(Year::from_str("9998")?)
        );
        assert_eq!(
            Year::from_str("1971")?.pred(),
            Some(Year::from_str("1970")?)
        );
        assert_eq!(Year::from_str("1970")?.pred(), None);
        Ok(())
    }

    #[test]
    fn succ_test() -> anyhow::Result<()> {
        assert_eq!(
            Year::from_str("1970")?.succ(),
            Some(Year::from_str("1971")?)
        );
        assert_eq!(
            Year::from_str("9998")?.succ(),
            Some(Year::from_str("9999")?)
        );
        assert_eq!(Year::from_str("9999")?.succ(), None);
        Ok(())
    }

    #[test]
    fn days_test() -> anyhow::Result<()> {
        assert_eq!(Year::from_str("2000")?.days(), Days::from(366));
        assert_eq!(Year::from_str("2001")?.days(), Days::from(365));
        Ok(())
    }
}
