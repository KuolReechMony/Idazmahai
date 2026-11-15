use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use chrono_tz::Tz;

#[derive(Debug)]
pub struct TimeSelection {
    time: NaiveTime,
    timezone: Tz,
}

#[derive(Debug)]
pub struct DateSelection {
    date: NaiveDate,
    timezone: Tz,
}

#[derive(Debug)]
pub struct DateTimeSelection {
    date_time: NaiveDateTime,
    timezone: Tz,
}
