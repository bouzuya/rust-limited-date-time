mod hour;
mod minute;
mod second;

use crate::private::seconds_from_midnight_from_time;
use crate::Seconds;

use std::convert::TryFrom;

pub use self::hour::*;
pub use self::minute::*;
pub use self::second::*;

use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Time {
    hour: Hour,
    minute: Minute,
    second: Second,
}

const MAX_TIME: Time = Time {
    hour: Hour::max(),
    minute: Minute::max(),
    second: Second::max(),
};

const MIN_TIME: Time = Time {
    hour: Hour::min(),
    minute: Minute::min(),
    second: Second::min(),
};

#[derive(Debug, Eq, Error, PartialEq)]
pub enum ParseTimeError {
    #[error("invalid format")]
    InvalidFormat,
    #[error("invalid length")]
    InvalidLength,
    #[error("parse hour")]
    ParseHour(ParseHourError),
    #[error("parse minute")]
    ParseMinute(ParseMinuteError),
    #[error("parse second")]
    ParseSecond(ParseSecondError),
}

#[derive(Debug, Eq, Error, PartialEq)]
#[error("invalid time error")]
pub struct InvalidTimeError;

impl Time {
    pub fn from_hms(hour: Hour, minute: Minute, second: Second) -> Self {
        Self {
            hour,
            minute,
            second,
        }
    }

    pub const fn max() -> Self {
        MAX_TIME
    }

    pub const fn min() -> Self {
        MIN_TIME
    }

    pub fn hour(&self) -> Hour {
        self.hour
    }

    pub fn minute(&self) -> Minute {
        self.minute
    }

    pub fn second(&self) -> Second {
        self.second
    }

    pub(crate) fn seconds_from_midnight(&self) -> Seconds {
        let seconds_from_midnight = seconds_from_midnight_from_time((
            i64::from(self.hour()),
            i64::from(self.minute()),
            i64::from(self.second()),
        ));
        let seconds_from_midnight_as_u64 =
            u64::try_from(seconds_from_midnight).expect("Time seconds from midnight is [0, 86400)");
        Seconds::from(seconds_from_midnight_as_u64)
    }
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.hour, self.minute, self.second)
    }
}

impl std::str::FromStr for Time {
    type Err = ParseTimeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 8 {
            return Err(Self::Err::InvalidLength);
        }
        let hour = match Hour::from_str(&s[0..2]) {
            Ok(h) => h,
            Err(e) => match e {
                ParseHourError::InvalidDigit => return Err(Self::Err::ParseHour(e)),
                ParseHourError::InvalidLength => unreachable!(),
                ParseHourError::OutOfRange => return Err(Self::Err::ParseHour(e)),
            },
        };
        if s.as_bytes().get(2) != Some(&b':') {
            return Err(Self::Err::InvalidFormat);
        }
        let minute = match Minute::from_str(&s[3..5]) {
            Ok(m) => m,
            Err(e) => match e {
                ParseMinuteError::InvalidDigit => return Err(Self::Err::ParseMinute(e)),
                ParseMinuteError::InvalidLength => unreachable!(),
                ParseMinuteError::OutOfRange => return Err(Self::Err::ParseMinute(e)),
            },
        };
        if s.as_bytes().get(5) != Some(&b':') {
            return Err(Self::Err::InvalidFormat);
        }
        let second = match Second::from_str(&s[6..8]) {
            Ok(s) => s,
            Err(e) => match e {
                ParseSecondError::InvalidDigit => return Err(Self::Err::ParseSecond(e)),
                ParseSecondError::InvalidLength => unreachable!(),
                ParseSecondError::OutOfRange => return Err(Self::Err::ParseSecond(e)),
            },
        };
        Ok(Time {
            hour,
            minute,
            second,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn from_ymd_test() -> anyhow::Result<()> {
        assert_eq!(
            Time::from_hms(
                Hour::from_str("04")?,
                Minute::from_str("05")?,
                Second::from_str("06")?
            ),
            Time::from_str("04:05:06")?
        );
        Ok(())
    }

    #[test]
    fn max_test() -> anyhow::Result<()> {
        assert_eq!(Time::max(), Time::from_str("23:59:59")?);
        Ok(())
    }

    #[test]
    fn min_test() -> anyhow::Result<()> {
        assert_eq!(Time::min(), Time::from_str("00:00:00")?);
        Ok(())
    }

    #[test]
    fn str_conversion_test() {
        type E = ParseTimeError;
        let f = |s: &str| Time::from_str(s);

        assert!(matches!(f("04:05:06"), Ok(_)));
        assert!(matches!(f("004:05:06"), Err(E::InvalidLength)));
        assert!(matches!(f("04-05:06"), Err(E::InvalidFormat)));
        assert!(matches!(f("04:05-06"), Err(E::InvalidFormat)));
        assert!(matches!(f("+4:05:06"), Err(E::ParseHour(_))));
        assert!(matches!(f("04:+5:06"), Err(E::ParseMinute(_))));
        assert!(matches!(f("04:05:+6"), Err(E::ParseSecond(_))));

        assert_eq!(
            f("04:05:06").map(|d| d.to_string()),
            Ok("04:05:06".to_string())
        );
    }

    #[test]
    fn hour_test() -> anyhow::Result<()> {
        let time = Time::from_str("04:05:06")?;
        assert_eq!(time.hour(), Hour::from_str("04")?);
        Ok(())
    }

    #[test]
    fn minute_test() -> anyhow::Result<()> {
        let time = Time::from_str("04:05:06")?;
        assert_eq!(time.minute(), Minute::from_str("05")?);
        Ok(())
    }

    #[test]
    fn second_test() -> anyhow::Result<()> {
        let time = Time::from_str("04:05:06")?;
        assert_eq!(time.second(), Second::from_str("06")?);
        Ok(())
    }

    #[test]
    fn seconds_from_midnight_test() -> anyhow::Result<()> {
        for h in 0..24 {
            let hour = Hour::try_from(h)?;
            for m in 0..60 {
                let minute = Minute::try_from(m)?;
                for s in 0..60 {
                    let second = Second::try_from(s)?;
                    let time = Time::from_hms(hour, minute, second);
                    assert_eq!(
                        time.seconds_from_midnight(),
                        Seconds::from(h * 60 * 60 + m * 60 + s)
                    );
                }
            }
        }
        Ok(())
    }
}
