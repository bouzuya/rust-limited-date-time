use std::convert::TryFrom;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Month(u8);

impl Month {
    pub fn january() -> Self {
        Self(1)
    }

    pub fn february() -> Self {
        Self(2)
    }

    pub fn march() -> Self {
        Self(3)
    }

    pub fn april() -> Self {
        Self(4)
    }

    pub fn may() -> Self {
        Self(5)
    }

    pub fn june() -> Self {
        Self(6)
    }

    pub fn july() -> Self {
        Self(7)
    }

    pub fn august() -> Self {
        Self(8)
    }

    pub fn september() -> Self {
        Self(9)
    }

    pub fn october() -> Self {
        Self(10)
    }

    pub fn november() -> Self {
        Self(11)
    }

    pub fn december() -> Self {
        Self(12)
    }

    pub fn pred(&self) -> Option<Self> {
        if self.0 > 1 {
            Some(Self(self.0 - 1))
        } else {
            None
        }
    }

    pub fn succ(&self) -> Option<Self> {
        if self.0 < 12 {
            Some(Self(self.0 + 1))
        } else {
            None
        }
    }
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum ParseMonthError {
    #[error("invalid digit")]
    InvalidDigit,
    #[error("invalid length")]
    InvalidLength,
    #[error("out of range")]
    OutOfRange,
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum TryFromMonthError {
    #[error("out of range")]
    OutOfRange,
}

impl std::fmt::Display for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.0)
    }
}

impl std::str::FromStr for Month {
    type Err = ParseMonthError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(Self::Err::InvalidLength);
        }
        let mut m = 0_u8;
        for c in s.chars() {
            let d = match c {
                '0'..='9' => c as u8 - b'0',
                _ => return Err(Self::Err::InvalidDigit),
            };
            m = m * 10 + d;
        }
        Self::try_from(m).map_err(|_| Self::Err::OutOfRange)
    }
}

impl From<Month> for i8 {
    fn from(month: Month) -> Self {
        i8::try_from(month.0).expect("month is [1,12]")
    }
}

impl From<Month> for i16 {
    fn from(month: Month) -> Self {
        i16::from(month.0)
    }
}

impl From<Month> for i32 {
    fn from(month: Month) -> Self {
        i32::from(month.0)
    }
}

impl From<Month> for i64 {
    fn from(month: Month) -> Self {
        i64::from(month.0)
    }
}

impl From<Month> for u8 {
    fn from(month: Month) -> Self {
        month.0
    }
}

impl From<Month> for u16 {
    fn from(month: Month) -> Self {
        u16::from(month.0)
    }
}

impl From<Month> for u32 {
    fn from(month: Month) -> Self {
        u32::from(month.0)
    }
}

impl From<Month> for u64 {
    fn from(month: Month) -> Self {
        u64::from(month.0)
    }
}

impl std::convert::TryFrom<i8> for Month {
    type Error = TryFromMonthError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let value_as_u8 = u8::try_from(value).map_err(|_| Self::Error::OutOfRange)?;
        Self::try_from(value_as_u8)
    }
}

impl std::convert::TryFrom<i16> for Month {
    type Error = TryFromMonthError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let value_as_u8 = u8::try_from(value).map_err(|_| Self::Error::OutOfRange)?;
        Self::try_from(value_as_u8)
    }
}

impl std::convert::TryFrom<i32> for Month {
    type Error = TryFromMonthError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let value_as_u8 = u8::try_from(value).map_err(|_| Self::Error::OutOfRange)?;
        Self::try_from(value_as_u8)
    }
}

impl std::convert::TryFrom<i64> for Month {
    type Error = TryFromMonthError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let value_as_u8 = u8::try_from(value).map_err(|_| Self::Error::OutOfRange)?;
        Self::try_from(value_as_u8)
    }
}

impl std::convert::TryFrom<u8> for Month {
    type Error = TryFromMonthError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=12).contains(&value) {
            return Err(Self::Error::OutOfRange);
        }
        Ok(Self(value))
    }
}

impl std::convert::TryFrom<u16> for Month {
    type Error = TryFromMonthError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let value_as_u8 = u8::try_from(value).map_err(|_| Self::Error::OutOfRange)?;
        Self::try_from(value_as_u8)
    }
}

impl std::convert::TryFrom<u32> for Month {
    type Error = TryFromMonthError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let value_as_u8 = u8::try_from(value).map_err(|_| Self::Error::OutOfRange)?;
        Self::try_from(value_as_u8)
    }
}

impl std::convert::TryFrom<u64> for Month {
    type Error = TryFromMonthError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let value_as_u8 = u8::try_from(value).map_err(|_| Self::Error::OutOfRange)?;
        Self::try_from(value_as_u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_conversion_test() {
        type E = ParseMonthError;
        let f = |s: &str| s.parse::<Month>();
        assert_eq!(f("01").map(|m| m.to_string()), Ok("01".to_string()));
        assert_eq!(f("12").map(|m| m.to_string()), Ok("12".to_string()));
        assert_eq!(f(""), Err(E::InvalidLength));
        assert_eq!(f("1"), Err(E::InvalidLength));
        assert_eq!(f("100"), Err(E::InvalidLength));
        assert_eq!(f("0a"), Err(E::InvalidDigit));
        assert_eq!(f("+1"), Err(E::InvalidDigit));
        assert_eq!(f("00"), Err(E::OutOfRange));
        assert_eq!(f("13"), Err(E::OutOfRange));
    }

    #[test]
    fn i8_conversion_test() {
        type E = TryFromMonthError;
        let f = |d: i8| Month::try_from(d);
        assert_eq!(f(0_i8), Err(E::OutOfRange));
        assert_eq!(f(1_i8).map(i8::from), Ok(1_i8));
        assert_eq!(f(12_i8).map(i8::from), Ok(12_i8));
        assert_eq!(f(13_i8), Err(E::OutOfRange));
    }

    #[test]
    fn i16_conversion_test() {
        type E = TryFromMonthError;
        let f = |d: i16| Month::try_from(d);
        assert_eq!(f(0_i16), Err(E::OutOfRange));
        assert_eq!(f(1_i16).map(i16::from), Ok(1_i16));
        assert_eq!(f(12_i16).map(i16::from), Ok(12_i16));
        assert_eq!(f(13_i16), Err(E::OutOfRange));
    }

    #[test]
    fn i32_conversion_test() {
        type E = TryFromMonthError;
        let f = |d: i32| Month::try_from(d);
        assert_eq!(f(0_i32), Err(E::OutOfRange));
        assert_eq!(f(1_i32).map(i32::from), Ok(1_i32));
        assert_eq!(f(12_i32).map(i32::from), Ok(12_i32));
        assert_eq!(f(13_i32), Err(E::OutOfRange));
    }

    #[test]
    fn i64_conversion_test() {
        type E = TryFromMonthError;
        let f = |d: i64| Month::try_from(d);
        assert_eq!(f(0_i64), Err(E::OutOfRange));
        assert_eq!(f(1_i64).map(i64::from), Ok(1_i64));
        assert_eq!(f(12_i64).map(i64::from), Ok(12_i64));
        assert_eq!(f(13_i64), Err(E::OutOfRange));
    }

    #[test]
    fn u8_conversion_test() {
        type E = TryFromMonthError;
        let f = |d: u8| Month::try_from(d);
        assert_eq!(f(0_u8), Err(E::OutOfRange));
        assert_eq!(f(1_u8).map(u8::from), Ok(1_u8));
        assert_eq!(f(12_u8).map(u8::from), Ok(12_u8));
        assert_eq!(f(13_u8), Err(E::OutOfRange));
    }

    #[test]
    fn u16_conversion_test() {
        type E = TryFromMonthError;
        let f = |d: u16| Month::try_from(d);
        assert_eq!(f(0_u16), Err(E::OutOfRange));
        assert_eq!(f(1_u16).map(u16::from), Ok(1_u16));
        assert_eq!(f(12_u16).map(u16::from), Ok(12_u16));
        assert_eq!(f(13_u16), Err(E::OutOfRange));
    }

    #[test]
    fn u32_conversion_test() {
        type E = TryFromMonthError;
        let f = |d: u32| Month::try_from(d);
        assert_eq!(f(0_u32), Err(E::OutOfRange));
        assert_eq!(f(1_u32).map(u32::from), Ok(1_u32));
        assert_eq!(f(12_u32).map(u32::from), Ok(12_u32));
        assert_eq!(f(13_u32), Err(E::OutOfRange));
    }

    #[test]
    fn u64_conversion_test() {
        type E = TryFromMonthError;
        let f = |d: u64| Month::try_from(d);
        assert_eq!(f(0_u64), Err(E::OutOfRange));
        assert_eq!(f(1_u64).map(u64::from), Ok(1_u64));
        assert_eq!(f(12_u64).map(u64::from), Ok(12_u64));
        assert_eq!(f(13_u64), Err(E::OutOfRange));
    }

    #[test]
    fn pred_test() -> anyhow::Result<()> {
        assert_eq!(Month::try_from(12)?.pred(), Some(Month::try_from(11)?));
        assert_eq!(Month::try_from(11)?.pred(), Some(Month::try_from(10)?));
        assert_eq!(Month::try_from(10)?.pred(), Some(Month::try_from(9)?));
        assert_eq!(Month::try_from(9)?.pred(), Some(Month::try_from(8)?));
        assert_eq!(Month::try_from(8)?.pred(), Some(Month::try_from(7)?));
        assert_eq!(Month::try_from(7)?.pred(), Some(Month::try_from(6)?));
        assert_eq!(Month::try_from(6)?.pred(), Some(Month::try_from(5)?));
        assert_eq!(Month::try_from(5)?.pred(), Some(Month::try_from(4)?));
        assert_eq!(Month::try_from(4)?.pred(), Some(Month::try_from(3)?));
        assert_eq!(Month::try_from(3)?.pred(), Some(Month::try_from(2)?));
        assert_eq!(Month::try_from(2)?.pred(), Some(Month::try_from(1)?));
        assert_eq!(Month::try_from(1)?.pred(), None);
        Ok(())
    }

    #[test]
    fn succ_test() -> anyhow::Result<()> {
        assert_eq!(Month::try_from(1)?.succ(), Some(Month::try_from(2)?));
        assert_eq!(Month::try_from(2)?.succ(), Some(Month::try_from(3)?));
        assert_eq!(Month::try_from(3)?.succ(), Some(Month::try_from(4)?));
        assert_eq!(Month::try_from(4)?.succ(), Some(Month::try_from(5)?));
        assert_eq!(Month::try_from(5)?.succ(), Some(Month::try_from(6)?));
        assert_eq!(Month::try_from(6)?.succ(), Some(Month::try_from(7)?));
        assert_eq!(Month::try_from(7)?.succ(), Some(Month::try_from(8)?));
        assert_eq!(Month::try_from(8)?.succ(), Some(Month::try_from(9)?));
        assert_eq!(Month::try_from(9)?.succ(), Some(Month::try_from(10)?));
        assert_eq!(Month::try_from(10)?.succ(), Some(Month::try_from(11)?));
        assert_eq!(Month::try_from(11)?.succ(), Some(Month::try_from(12)?));
        assert_eq!(Month::try_from(12)?.succ(), None);
        Ok(())
    }

    #[test]
    fn name_test() -> anyhow::Result<()> {
        assert_eq!(Month::try_from(1)?, Month::january());
        assert_eq!(Month::try_from(2)?, Month::february());
        assert_eq!(Month::try_from(3)?, Month::march());
        assert_eq!(Month::try_from(4)?, Month::april());
        assert_eq!(Month::try_from(5)?, Month::may());
        assert_eq!(Month::try_from(6)?, Month::june());
        assert_eq!(Month::try_from(7)?, Month::july());
        assert_eq!(Month::try_from(8)?, Month::august());
        assert_eq!(Month::try_from(9)?, Month::september());
        assert_eq!(Month::try_from(10)?, Month::october());
        assert_eq!(Month::try_from(11)?, Month::november());
        assert_eq!(Month::try_from(12)?, Month::december());
        Ok(())
    }
}
