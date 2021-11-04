use crate::Seconds;
use std::convert::TryFrom;
use thiserror::Error;

// 1970-01-01 ... +0
// 9999-12-31 ... +2_932_896
// [0, 2_932_896]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Days(u32);

impl From<Days> for i32 {
    fn from(value: Days) -> Self {
        i32::try_from(value.0).expect("days is [0, 2_932_896]")
    }
}

impl From<Days> for i64 {
    fn from(value: Days) -> Self {
        i64::try_from(value.0).expect("days is [0, 2_932_896]")
    }
}

impl From<Days> for u32 {
    fn from(value: Days) -> Self {
        value.0
    }
}

impl From<Days> for u64 {
    fn from(value: Days) -> Self {
        u64::from(value.0)
    }
}

impl From<u8> for Days {
    fn from(value: u8) -> Self {
        Self(u32::from(value))
    }
}

impl From<u16> for Days {
    fn from(value: u16) -> Self {
        Self(u32::from(value))
    }
}

impl std::convert::TryFrom<Days> for i8 {
    type Error = TryFromDaysError;

    fn try_from(value: Days) -> Result<Self, Self::Error> {
        i8::try_from(value.0).map_err(|_| Self::Error::OutOfRange)
    }
}

impl std::convert::TryFrom<Days> for i16 {
    type Error = TryFromDaysError;

    fn try_from(value: Days) -> Result<Self, Self::Error> {
        i16::try_from(value.0).map_err(|_| Self::Error::OutOfRange)
    }
}

impl std::convert::TryFrom<Days> for u8 {
    type Error = TryFromDaysError;

    fn try_from(value: Days) -> Result<Self, Self::Error> {
        u8::try_from(value.0).map_err(|_| Self::Error::OutOfRange)
    }
}

impl std::convert::TryFrom<Days> for u16 {
    type Error = TryFromDaysError;

    fn try_from(value: Days) -> Result<Self, Self::Error> {
        u16::try_from(value.0).map_err(|_| Self::Error::OutOfRange)
    }
}

impl std::convert::TryFrom<i8> for Days {
    type Error = TryFromDaysError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let value_as_u32 = u32::try_from(value).map_err(|_| Self::Error::OutOfRange)?;
        Self::try_from(value_as_u32)
    }
}

impl std::convert::TryFrom<i16> for Days {
    type Error = TryFromDaysError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let value_as_u32 = u32::try_from(value).map_err(|_| Self::Error::OutOfRange)?;
        Self::try_from(value_as_u32)
    }
}

impl std::convert::TryFrom<i32> for Days {
    type Error = TryFromDaysError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let value_as_u32 = u32::try_from(value).map_err(|_| Self::Error::OutOfRange)?;
        Self::try_from(value_as_u32)
    }
}

impl std::convert::TryFrom<i64> for Days {
    type Error = TryFromDaysError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let value_as_u32 = u32::try_from(value).map_err(|_| Self::Error::OutOfRange)?;
        Self::try_from(value_as_u32)
    }
}

impl std::convert::TryFrom<u32> for Days {
    type Error = TryFromDaysError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if (0_u32..=2_932_896_u32).contains(&value) {
            Ok(Self(value))
        } else {
            Err(Self::Error::OutOfRange)
        }
    }
}

impl std::convert::TryFrom<u64> for Days {
    type Error = TryFromDaysError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let value_as_u32 = u32::try_from(value).map_err(|_| Self::Error::OutOfRange)?;
        Self::try_from(value_as_u32)
    }
}

// TODO: impl From<Days> for Hours
// TODO: impl From<Days> for Minutes

impl From<Days> for Seconds {
    fn from(value: Days) -> Self {
        Self::from(value.0 as u64 * 86_400_u64)
    }
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum TryFromDaysError {
    #[error("out of range")]
    OutOfRange,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn i8_conversion_test() {
        assert!(Days::try_from(-1_i8).is_err());
        assert_eq!(Days::try_from(0_i8).and_then(i8::try_from), Ok(0_i8));
        assert_eq!(Days::try_from(i8::MAX).and_then(i8::try_from), Ok(i8::MAX));
    }

    #[test]
    fn i16_conversion_test() {
        assert!(Days::try_from(-1_i16).is_err());
        assert_eq!(Days::try_from(0_i16).and_then(i16::try_from), Ok(0_i16));
        assert_eq!(
            Days::try_from(i16::MAX).and_then(i16::try_from),
            Ok(i16::MAX)
        );
    }

    #[test]
    fn i32_conversion_test() {
        assert!(Days::try_from(-1_i32).is_err());
        assert_eq!(Days::try_from(0_i32).map(i32::from), Ok(0_i32));
        assert_eq!(
            Days::try_from(2_932_896_i32).map(i32::from),
            Ok(2_932_896_i32)
        );
        assert!(Days::try_from(2_932_896_i32 + 1).is_err());
    }

    #[test]
    fn i64_conversion_test() {
        assert!(Days::try_from(-1_i64).is_err());
        assert_eq!(Days::try_from(0_i64).map(i64::from), Ok(0_i64));
        assert_eq!(
            Days::try_from(2_932_896_i64).map(i64::from),
            Ok(2_932_896_i64)
        );
        assert!(Days::try_from(2_932_896_i64 + 1).is_err());
    }

    #[test]
    fn u8_conversion_test() {
        assert_eq!(u8::try_from(Days::from(0_u8)), Ok(0_u8));
        assert_eq!(u8::try_from(Days::from(u8::MAX)), Ok(u8::MAX));
    }

    #[test]
    fn u16_conversion_test() {
        assert_eq!(u16::try_from(Days::from(0_u16)), Ok(0_u16));
        assert_eq!(u16::try_from(Days::from(u16::MAX)), Ok(u16::MAX));
    }

    #[test]
    fn u32_conversion_test() {
        assert_eq!(Days::try_from(0_u32).map(u32::from), Ok(0_u32));
        assert_eq!(
            Days::try_from(2_932_896_u32).map(u32::from),
            Ok(2_932_896_u32)
        );
        assert!(Days::try_from(2_932_896_u32 + 1).is_err());
    }

    #[test]
    fn u64_conversion_test() {
        assert_eq!(Days::try_from(0_u64).map(u64::from), Ok(0_u64));
        assert_eq!(
            Days::try_from(2_932_896_u64).map(u64::from),
            Ok(2_932_896_u64)
        );
        assert!(Days::try_from(2_932_896_u64 + 1).is_err());
    }

    #[test]
    fn seconds_conversion_test() {
        assert_eq!(Seconds::from(Days::from(0_u8)), Seconds::from(0_u64));
        assert_eq!(Seconds::from(Days::from(1_u8)), Seconds::from(86_400_u64));
    }
}
