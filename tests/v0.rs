use std::{convert::TryFrom, str::FromStr};

use limited_date_time::{Date, DateTime, DayOfMonth, Days, OffsetDateTime, Year, YearMonth};

#[test]
fn use_case_offset_date_time_plus_days() -> anyhow::Result<()> {
    let plus_days =
        |offset_date_time: OffsetDateTime, days: Days| -> anyhow::Result<OffsetDateTime> {
            let updated_offset_date_time = OffsetDateTime::from_instant(
                offset_date_time.instant() + days,
                offset_date_time.offset(),
            )?;
            Ok(updated_offset_date_time)
        };

    let offset_date_time = OffsetDateTime::from_str("2021-02-03T04:05:06+09:00")?;
    let days = Days::from(2_u8);
    let updated_offset_date_time = plus_days(offset_date_time, days)?;
    assert_eq!(
        updated_offset_date_time.to_string(),
        "2021-02-05T04:05:06+09:00"
    );

    Ok(())
}

#[test]
fn use_case_offset_date_time_with_day_of_month() -> anyhow::Result<()> {
    let with_day_of_month = |offset_date_time: OffsetDateTime,
                             day_of_month: DayOfMonth|
     -> anyhow::Result<OffsetDateTime> {
        let date_time = offset_date_time.date_time();
        let offset = offset_date_time.offset();
        let date = date_time.date();
        let time = date_time.time();

        let updated_date = Date::from_ymd(date.year(), date.month(), day_of_month)?;

        let updated_date_time = DateTime::from_date_time(updated_date, time);
        let updated_offset_date_time = OffsetDateTime::new(updated_date_time, offset);
        Ok(updated_offset_date_time)
    };

    let offset_date_time = OffsetDateTime::from_str("2021-02-03T04:05:06+09:00")?;
    let day_of_month = DayOfMonth::try_from(14)?;
    let updated_offset_date_time = with_day_of_month(offset_date_time, day_of_month)?;
    assert_eq!(
        updated_offset_date_time.to_string(),
        "2021-02-14T04:05:06+09:00"
    );

    Ok(())
}

#[test]
fn use_case_offset_date_time_next_date() -> anyhow::Result<()> {
    let next_date = |offset_date_time: OffsetDateTime| -> anyhow::Result<OffsetDateTime> {
        let date_time = offset_date_time.date_time();
        let offset = offset_date_time.offset();
        let date = date_time.date();
        let time = date_time.time();
        let updated_date = date
            .succ()
            .ok_or_else(|| anyhow::anyhow!("Date out of range"))?;
        let updated_date_time = DateTime::from_date_time(updated_date, time);
        let updated_offset_date_time = OffsetDateTime::new(updated_date_time, offset);
        Ok(updated_offset_date_time)
    };

    let offset_date_time = OffsetDateTime::from_str("2021-02-03T04:05:06+09:00")?;
    let updated_offset_date_time = next_date(offset_date_time)?;
    assert_eq!(
        updated_offset_date_time.to_string(),
        "2021-02-04T04:05:06+09:00"
    );

    Ok(())
}

#[test]
fn use_case_offset_date_time_next_month() -> anyhow::Result<()> {
    let next_month = |offset_date_time: OffsetDateTime| -> anyhow::Result<OffsetDateTime> {
        let date_time = offset_date_time.date_time();
        let offset = offset_date_time.offset();
        let date = date_time.date();
        let time = date_time.time();
        let next_year_month = date
            .year_month()
            .succ()
            .ok_or_else(|| anyhow::anyhow!("YearMonth out of range"))?;
        let updated_date = Date::from_ymd(
            next_year_month.year(),
            next_year_month.month(),
            date.day_of_month(),
        )?;
        let updated_date_time = DateTime::from_date_time(updated_date, time);
        let updated_offset_date_time = OffsetDateTime::new(updated_date_time, offset);
        Ok(updated_offset_date_time)
    };

    let offset_date_time = OffsetDateTime::from_str("2021-02-03T04:05:06+09:00")?;
    let updated_offset_date_time = next_month(offset_date_time)?;
    assert_eq!(
        updated_offset_date_time.to_string(),
        "2021-03-03T04:05:06+09:00"
    );

    Ok(())
}

#[test]
fn use_case_get_days() -> anyhow::Result<()> {
    assert_eq!(DayOfMonth::from_str("03")?.days(), Days::from(1_u16));
    // Month::days is not supported
    assert_eq!(YearMonth::from_str("2021-02")?.days(), Days::from(28_u16));
    assert_eq!(Year::from_str("2021")?.days(), Days::from(365_u16));
    // Date::days is not supported (yet?)

    // TODO: date range -> days
    Ok(())
}
