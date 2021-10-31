use std::convert::TryFrom;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Second(u8);

const MAX_SECOND: Second = Second(59);

const MIN_SECOND: Second = Second(0);

impl Second {
    pub const fn max() -> Self {
        MAX_SECOND
    }

    pub const fn min() -> Self {
        MIN_SECOND
    }
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum ParseSecondError {
    #[error("invalid digit")]
    InvalidDigit,
    #[error("invalid length")]
    InvalidLength,
    #[error("out of range")]
    OutOfRange,
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum TryFromSecondError {
    #[error("out of range")]
    OutOfRange,
}

impl std::fmt::Display for Second {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.0)
    }
}

impl std::str::FromStr for Second {
    type Err = ParseSecondError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(Self::Err::InvalidLength);
        }
        let mut h = 0_u8;
        for c in s.chars() {
            let d = match c {
                '0'..='9' => c as u8 - b'0',
                _ => return Err(Self::Err::InvalidDigit),
            };
            h = h * 10 + d;
        }
        Self::try_from(h).map_err(|_| Self::Err::OutOfRange)
    }
}

impl From<Second> for i8 {
    fn from(second: Second) -> Self {
        i8::try_from(second.0).expect("second is [0, 59]")
    }
}

impl From<Second> for i16 {
    fn from(second: Second) -> Self {
        i16::from(second.0)
    }
}

impl From<Second> for i32 {
    fn from(second: Second) -> Self {
        i32::from(second.0)
    }
}

impl From<Second> for i64 {
    fn from(second: Second) -> Self {
        i64::from(second.0)
    }
}

impl From<Second> for u8 {
    fn from(second: Second) -> Self {
        second.0
    }
}

impl From<Second> for u16 {
    fn from(second: Second) -> Self {
        u16::from(second.0)
    }
}

impl From<Second> for u32 {
    fn from(second: Second) -> Self {
        u32::from(second.0)
    }
}

impl From<Second> for u64 {
    fn from(second: Second) -> Self {
        u64::from(second.0)
    }
}

impl std::convert::TryFrom<i8> for Second {
    type Error = TryFromSecondError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let value_as_u8 = u8::try_from(value).map_err(|_| Self::Error::OutOfRange)?;
        Self::try_from(value_as_u8)
    }
}

impl std::convert::TryFrom<i16> for Second {
    type Error = TryFromSecondError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let value_as_u8 = u8::try_from(value).map_err(|_| Self::Error::OutOfRange)?;
        Self::try_from(value_as_u8)
    }
}

impl std::convert::TryFrom<i32> for Second {
    type Error = TryFromSecondError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let value_as_u8 = u8::try_from(value).map_err(|_| Self::Error::OutOfRange)?;
        Self::try_from(value_as_u8)
    }
}

impl std::convert::TryFrom<i64> for Second {
    type Error = TryFromSecondError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let value_as_u8 = u8::try_from(value).map_err(|_| Self::Error::OutOfRange)?;
        Self::try_from(value_as_u8)
    }
}

impl std::convert::TryFrom<u8> for Second {
    type Error = TryFromSecondError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(0..=59).contains(&value) {
            return Err(Self::Error::OutOfRange);
        }
        Ok(Self(value))
    }
}

impl std::convert::TryFrom<u16> for Second {
    type Error = TryFromSecondError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let value_as_u8 = u8::try_from(value).map_err(|_| Self::Error::OutOfRange)?;
        Self::try_from(value_as_u8)
    }
}

impl std::convert::TryFrom<u32> for Second {
    type Error = TryFromSecondError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let value_as_u8 = u8::try_from(value).map_err(|_| Self::Error::OutOfRange)?;
        Self::try_from(value_as_u8)
    }
}

impl std::convert::TryFrom<u64> for Second {
    type Error = TryFromSecondError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let value_as_u8 = u8::try_from(value).map_err(|_| Self::Error::OutOfRange)?;
        Self::try_from(value_as_u8)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn max_test() -> anyhow::Result<()> {
        assert_eq!(Second::max(), Second::from_str("59")?);
        Ok(())
    }

    #[test]
    fn min_test() -> anyhow::Result<()> {
        assert_eq!(Second::min(), Second::from_str("00")?);
        Ok(())
    }

    #[test]
    fn str_conversion_test() {
        type E = ParseSecondError;
        let f = |s: &str| s.parse::<Second>();
        assert_eq!(f("00").map(|d| d.to_string()), Ok("00".to_string()));
        assert_eq!(f("59").map(|d| d.to_string()), Ok("59".to_string()));
        assert_eq!(f(""), Err(E::InvalidLength));
        assert_eq!(f("1"), Err(E::InvalidLength));
        assert_eq!(f("100"), Err(E::InvalidLength));
        assert_eq!(f("0a"), Err(E::InvalidDigit));
        assert_eq!(f("+1"), Err(E::InvalidDigit));
        assert_eq!(f("60"), Err(E::OutOfRange));
    }

    #[test]
    fn i8_conversion_test() {
        type E = TryFromSecondError;
        let f = |d: i8| Second::try_from(d);
        assert_eq!(f(-1_i8), Err(E::OutOfRange));
        assert_eq!(f(0_i8).map(i8::from), Ok(0_i8));
        assert_eq!(f(59_i8).map(i8::from), Ok(59_i8));
        assert_eq!(f(60_i8), Err(E::OutOfRange));
    }

    #[test]
    fn i16_conversion_test() {
        type E = TryFromSecondError;
        let f = |d: i16| Second::try_from(d);
        assert_eq!(f(-1_i16), Err(E::OutOfRange));
        assert_eq!(f(0_i16).map(i16::from), Ok(0_i16));
        assert_eq!(f(59_i16).map(i16::from), Ok(59_i16));
        assert_eq!(f(60_i16), Err(E::OutOfRange));
    }

    #[test]
    fn i32_conversion_test() {
        type E = TryFromSecondError;
        let f = |d: i32| Second::try_from(d);
        assert_eq!(f(-1_i32), Err(E::OutOfRange));
        assert_eq!(f(0_i32).map(i32::from), Ok(0_i32));
        assert_eq!(f(59_i32).map(i32::from), Ok(59_i32));
        assert_eq!(f(60_i32), Err(E::OutOfRange));
    }

    #[test]
    fn i64_conversion_test() {
        type E = TryFromSecondError;
        let f = |d: i64| Second::try_from(d);
        assert_eq!(f(-1_i64), Err(E::OutOfRange));
        assert_eq!(f(0_i64).map(i64::from), Ok(0_i64));
        assert_eq!(f(59_i64).map(i64::from), Ok(59_i64));
        assert_eq!(f(60_i64), Err(E::OutOfRange));
    }

    #[test]
    fn u8_conversion_test() {
        type E = TryFromSecondError;
        let f = |d: u8| Second::try_from(d);
        assert_eq!(f(0_u8).map(u8::from), Ok(0_u8));
        assert_eq!(f(59_u8).map(u8::from), Ok(59_u8));
        assert_eq!(f(60_u8), Err(E::OutOfRange));
    }

    #[test]
    fn u16_conversion_test() {
        type E = TryFromSecondError;
        let f = |d: u16| Second::try_from(d);
        assert_eq!(f(0_u16).map(u16::from), Ok(0_u16));
        assert_eq!(f(59_u16).map(u16::from), Ok(59_u16));
        assert_eq!(f(60_u16), Err(E::OutOfRange));
    }

    #[test]
    fn u32_conversion_test() {
        type E = TryFromSecondError;
        let f = |d: u32| Second::try_from(d);
        assert_eq!(f(0_u32).map(u32::from), Ok(0_u32));
        assert_eq!(f(59_u32).map(u32::from), Ok(59_u32));
        assert_eq!(f(60_u32), Err(E::OutOfRange));
    }

    #[test]
    fn u64_conversion_test() {
        type E = TryFromSecondError;
        let f = |d: u64| Second::try_from(d);
        assert_eq!(f(0_u64).map(u64::from), Ok(0_u64));
        assert_eq!(f(59_u64).map(u64::from), Ok(59_u64));
        assert_eq!(f(60_u64), Err(E::OutOfRange));
    }
}
