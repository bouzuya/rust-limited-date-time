use std::convert::TryFrom;
use thiserror::Error;

use crate::Days;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DayOfMonth(u8);

impl DayOfMonth {
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
        if self.0 < 31 {
            Some(Self(self.0 + 1))
        } else {
            None
        }
    }
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum ParseDayOfMonthError {
    #[error("invalid digit")]
    InvalidDigit,
    #[error("invalid length")]
    InvalidLength,
    #[error("out of range")]
    OutOfRange,
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum TryFromDayOfMonthError {
    #[error("out of range")]
    OutOfRange,
}

impl std::fmt::Display for DayOfMonth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.0)
    }
}

impl std::str::FromStr for DayOfMonth {
    type Err = ParseDayOfMonthError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(Self::Err::InvalidLength);
        }
        let mut dom = 0_u8;
        for c in s.chars() {
            let d = match c {
                '0'..='9' => c as u8 - b'0',
                _ => return Err(Self::Err::InvalidDigit),
            };
            dom = dom * 10 + d;
        }
        Self::try_from(dom).map_err(|_| Self::Err::OutOfRange)
    }
}

impl From<DayOfMonth> for i8 {
    fn from(day_of_month: DayOfMonth) -> Self {
        i8::try_from(day_of_month.0).expect("day_of_month is [1,31]")
    }
}

impl From<DayOfMonth> for i16 {
    fn from(day_of_month: DayOfMonth) -> Self {
        i16::from(day_of_month.0)
    }
}

impl From<DayOfMonth> for i32 {
    fn from(day_of_month: DayOfMonth) -> Self {
        i32::from(day_of_month.0)
    }
}

impl From<DayOfMonth> for i64 {
    fn from(day_of_month: DayOfMonth) -> Self {
        i64::from(day_of_month.0)
    }
}

impl From<DayOfMonth> for u8 {
    fn from(day_of_month: DayOfMonth) -> Self {
        day_of_month.0
    }
}

impl From<DayOfMonth> for u16 {
    fn from(day_of_month: DayOfMonth) -> Self {
        u16::from(day_of_month.0)
    }
}

impl From<DayOfMonth> for u32 {
    fn from(day_of_month: DayOfMonth) -> Self {
        u32::from(day_of_month.0)
    }
}

impl From<DayOfMonth> for u64 {
    fn from(day_of_month: DayOfMonth) -> Self {
        u64::from(day_of_month.0)
    }
}

impl std::convert::TryFrom<i8> for DayOfMonth {
    type Error = TryFromDayOfMonthError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let value_as_u8 = u8::try_from(value).map_err(|_| Self::Error::OutOfRange)?;
        Self::try_from(value_as_u8)
    }
}

impl std::convert::TryFrom<i16> for DayOfMonth {
    type Error = TryFromDayOfMonthError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let value_as_u8 = u8::try_from(value).map_err(|_| Self::Error::OutOfRange)?;
        Self::try_from(value_as_u8)
    }
}

impl std::convert::TryFrom<i32> for DayOfMonth {
    type Error = TryFromDayOfMonthError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let value_as_u8 = u8::try_from(value).map_err(|_| Self::Error::OutOfRange)?;
        Self::try_from(value_as_u8)
    }
}

impl std::convert::TryFrom<i64> for DayOfMonth {
    type Error = TryFromDayOfMonthError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let value_as_u8 = u8::try_from(value).map_err(|_| Self::Error::OutOfRange)?;
        Self::try_from(value_as_u8)
    }
}

impl std::convert::TryFrom<u8> for DayOfMonth {
    type Error = TryFromDayOfMonthError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=31).contains(&value) {
            return Err(Self::Error::OutOfRange);
        }
        Ok(Self(value))
    }
}

impl std::convert::TryFrom<u16> for DayOfMonth {
    type Error = TryFromDayOfMonthError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let value_as_u8 = u8::try_from(value).map_err(|_| Self::Error::OutOfRange)?;
        Self::try_from(value_as_u8)
    }
}

impl std::convert::TryFrom<u32> for DayOfMonth {
    type Error = TryFromDayOfMonthError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let value_as_u8 = u8::try_from(value).map_err(|_| Self::Error::OutOfRange)?;
        Self::try_from(value_as_u8)
    }
}

impl std::convert::TryFrom<u64> for DayOfMonth {
    type Error = TryFromDayOfMonthError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let value_as_u8 = u8::try_from(value).map_err(|_| Self::Error::OutOfRange)?;
        Self::try_from(value_as_u8)
    }
}

impl std::ops::Add<Days> for DayOfMonth {
    type Output = DayOfMonth;

    fn add(self, rhs: Days) -> Self::Output {
        u32::from(self.0)
            .checked_add(u32::from(rhs))
            .and_then(|d| u8::try_from(d).ok())
            .and_then(|d| DayOfMonth::try_from(d).ok())
            .unwrap_or_else(|| panic!("overflow"))
    }
}

impl std::ops::Add<DayOfMonth> for Days {
    type Output = DayOfMonth;

    fn add(self, rhs: DayOfMonth) -> Self::Output {
        rhs + self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_conversion_test() {
        type E = ParseDayOfMonthError;
        let f = |s: &str| s.parse::<DayOfMonth>();
        assert_eq!(f("01").map(|d| d.to_string()), Ok("01".to_string()));
        assert_eq!(f("31").map(|d| d.to_string()), Ok("31".to_string()));
        assert_eq!(f(""), Err(E::InvalidLength));
        assert_eq!(f("1"), Err(E::InvalidLength));
        assert_eq!(f("100"), Err(E::InvalidLength));
        assert_eq!(f("0a"), Err(E::InvalidDigit));
        assert_eq!(f("+1"), Err(E::InvalidDigit));
        assert_eq!(f("00"), Err(E::OutOfRange));
        assert_eq!(f("32"), Err(E::OutOfRange));
    }

    #[test]
    fn i8_conversion_test() {
        type E = TryFromDayOfMonthError;
        let f = |d: i8| DayOfMonth::try_from(d);
        assert_eq!(f(0_i8), Err(E::OutOfRange));
        assert_eq!(f(1_i8).map(i8::from), Ok(1_i8));
        assert_eq!(f(31_i8).map(i8::from), Ok(31_i8));
        assert_eq!(f(32_i8), Err(E::OutOfRange));
    }

    #[test]
    fn i16_conversion_test() {
        type E = TryFromDayOfMonthError;
        let f = |d: i16| DayOfMonth::try_from(d);
        assert_eq!(f(0_i16), Err(E::OutOfRange));
        assert_eq!(f(1_i16).map(i16::from), Ok(1_i16));
        assert_eq!(f(31_i16).map(i16::from), Ok(31_i16));
        assert_eq!(f(32_i16), Err(E::OutOfRange));
    }

    #[test]
    fn i32_conversion_test() {
        type E = TryFromDayOfMonthError;
        let f = |d: i32| DayOfMonth::try_from(d);
        assert_eq!(f(0_i32), Err(E::OutOfRange));
        assert_eq!(f(1_i32).map(i32::from), Ok(1_i32));
        assert_eq!(f(31_i32).map(i32::from), Ok(31_i32));
        assert_eq!(f(32_i32), Err(E::OutOfRange));
    }

    #[test]
    fn i64_conversion_test() {
        type E = TryFromDayOfMonthError;
        let f = |d: i64| DayOfMonth::try_from(d);
        assert_eq!(f(0_i64), Err(E::OutOfRange));
        assert_eq!(f(1_i64).map(i64::from), Ok(1_i64));
        assert_eq!(f(31_i64).map(i64::from), Ok(31_i64));
        assert_eq!(f(32_i64), Err(E::OutOfRange));
    }

    #[test]
    fn u8_conversion_test() {
        type E = TryFromDayOfMonthError;
        let f = |d: u8| DayOfMonth::try_from(d);
        assert_eq!(f(0_u8), Err(E::OutOfRange));
        assert_eq!(f(1_u8).map(u8::from), Ok(1_u8));
        assert_eq!(f(31_u8).map(u8::from), Ok(31_u8));
        assert_eq!(f(32_u8), Err(E::OutOfRange));
    }

    #[test]
    fn u16_conversion_test() {
        type E = TryFromDayOfMonthError;
        let f = |d: u16| DayOfMonth::try_from(d);
        assert_eq!(f(0_u16), Err(E::OutOfRange));
        assert_eq!(f(1_u16).map(u16::from), Ok(1_u16));
        assert_eq!(f(31_u16).map(u16::from), Ok(31_u16));
        assert_eq!(f(32_u16), Err(E::OutOfRange));
    }

    #[test]
    fn u32_conversion_test() {
        type E = TryFromDayOfMonthError;
        let f = |d: u32| DayOfMonth::try_from(d);
        assert_eq!(f(0_u32), Err(E::OutOfRange));
        assert_eq!(f(1_u32).map(u32::from), Ok(1_u32));
        assert_eq!(f(31_u32).map(u32::from), Ok(31_u32));
        assert_eq!(f(32_u32), Err(E::OutOfRange));
    }

    #[test]
    fn u64_conversion_test() {
        type E = TryFromDayOfMonthError;
        let f = |d: u64| DayOfMonth::try_from(d);
        assert_eq!(f(0_u64), Err(E::OutOfRange));
        assert_eq!(f(1_u64).map(u64::from), Ok(1_u64));
        assert_eq!(f(31_u64).map(u64::from), Ok(31_u64));
        assert_eq!(f(32_u64), Err(E::OutOfRange));
    }

    #[test]
    fn pred_test() -> anyhow::Result<()> {
        assert_eq!(
            DayOfMonth::try_from(31)?.pred(),
            Some(DayOfMonth::try_from(30)?)
        );
        assert_eq!(
            DayOfMonth::try_from(2)?.pred(),
            Some(DayOfMonth::try_from(1)?)
        );
        assert_eq!(DayOfMonth::try_from(1)?.pred(), None);
        Ok(())
    }

    #[test]
    fn succ_test() -> anyhow::Result<()> {
        assert_eq!(
            DayOfMonth::try_from(1)?.succ(),
            Some(DayOfMonth::try_from(2)?)
        );
        assert_eq!(
            DayOfMonth::try_from(30)?.succ(),
            Some(DayOfMonth::try_from(31)?)
        );
        assert_eq!(DayOfMonth::try_from(31)?.succ(), None);
        Ok(())
    }

    #[test]
    fn add_days_test() -> anyhow::Result<()> {
        let d1 = DayOfMonth::try_from(1)?;
        let d2 = DayOfMonth::try_from(2)?;
        let d31 = DayOfMonth::try_from(31)?;
        assert_eq!(d1 + Days::from(0_u16), d1);
        assert_eq!(d1 + Days::from(1_u16), d2);
        assert_eq!(d1 + Days::from(30_u16), d31);
        // should_panic
        // assert_eq!(d1 + Days::from(31), d31);
        assert_eq!(Days::from(0_u16) + d1, d1);
        assert_eq!(Days::from(1_u16) + d1, d2);
        assert_eq!(Days::from(30_u16) + d1, d31);
        Ok(())
    }

    #[test]
    fn days_test() -> anyhow::Result<()> {
        assert_eq!(DayOfMonth::try_from(1)?.days(), Days::from(1_u16));
        Ok(())
    }
}
