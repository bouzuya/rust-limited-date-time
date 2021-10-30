use std::convert::TryFrom;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Hour(u8);

const MAX_HOUR: Hour = Hour(23);

const MIN_HOUR: Hour = Hour(0);

impl Hour {
    pub const fn max() -> Self {
        MAX_HOUR
    }

    pub const fn min() -> Self {
        MIN_HOUR
    }
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum ParseHourError {
    #[error("invalid digit")]
    InvalidDigit,
    #[error("invalid length")]
    InvalidLength,
    #[error("out of range")]
    OutOfRange,
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum TryFromHourError {
    #[error("out of range")]
    OutOfRange,
}

impl std::fmt::Display for Hour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.0)
    }
}

impl std::str::FromStr for Hour {
    type Err = ParseHourError;

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

impl From<Hour> for u8 {
    fn from(hour: Hour) -> Self {
        hour.0
    }
}

impl std::convert::TryFrom<u8> for Hour {
    type Error = TryFromHourError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(0..=23).contains(&value) {
            return Err(Self::Error::OutOfRange);
        }
        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn max_test() -> anyhow::Result<()> {
        assert_eq!(Hour::max(), Hour::from_str("23")?);
        Ok(())
    }

    #[test]
    fn min_test() -> anyhow::Result<()> {
        assert_eq!(Hour::min(), Hour::from_str("00")?);
        Ok(())
    }

    #[test]
    fn str_conversion_test() {
        type E = ParseHourError;
        let f = |s: &str| s.parse::<Hour>();
        assert_eq!(f("00").map(|d| d.to_string()), Ok("00".to_string()));
        assert_eq!(f("23").map(|d| d.to_string()), Ok("23".to_string()));
        assert_eq!(f(""), Err(E::InvalidLength));
        assert_eq!(f("1"), Err(E::InvalidLength));
        assert_eq!(f("100"), Err(E::InvalidLength));
        assert_eq!(f("0a"), Err(E::InvalidDigit));
        assert_eq!(f("+1"), Err(E::InvalidDigit));
        assert_eq!(f("24"), Err(E::OutOfRange));
    }

    #[test]
    fn u8_conversion_test() {
        type E = TryFromHourError;
        let f = |d: u8| Hour::try_from(d);
        assert_eq!(f(0_u8).map(u8::from), Ok(0_u8));
        assert_eq!(f(23_u8).map(u8::from), Ok(23_u8));
        assert_eq!(f(24_u8), Err(E::OutOfRange));
    }
}
