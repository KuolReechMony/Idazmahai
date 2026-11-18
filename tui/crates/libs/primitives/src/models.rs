use chrono::{DateTime, NaiveDateTime, TimeZone, Utc, offset::LocalResult};
use chrono_tz::Tz;

use crate::errors::EntityError;

/// Simple abstraction for non empty string types
#[derive(Debug)]
pub struct NonEmptyString {
    s: String,
}

impl NonEmptyString {
    /// Only handles whitespace endings and String::len() == 0
    pub fn new(s: impl Into<String>) -> Option<NonEmptyString> {
        let s = s.into();
        if !s.trim().is_empty() {
            Some(NonEmptyString {
                s: String::from(s.trim()),
            })
        } else {
            None
        }
    }

    /// Easily borrow contents
    pub fn as_str(&self) -> &str {
        self.s.as_str()
    }

    /// Filter out
    pub fn from_string_vec(v: Vec<String>) -> Vec<Self> {
        v.iter()
            .filter(|x| !x.trim().is_empty())
            .map(|y| NonEmptyString::new(y).unwrap())
            .collect()
    }
}

impl From<NonEmptyString> for String {
    /// Abstraction to easily convert into String
    /// Example
    /// ```
    /// use primitives::models::NonEmptyString;
    /// let static_string: &'static str = "A static string";
    /// let a_string = String::from("A string");
    /// let static_string: String = NonEmptyString::new(static_string).unwrap().into();
    /// ```
    fn from(v: NonEmptyString) -> Self {
        v.s
    }
}

pub fn check_field_string_not_empty(
    s: String,
    field: &'static str,
) -> Result<NonEmptyString, EntityError> {
    match NonEmptyString::new(s) {
        Some(st) => Ok(st),
        None => Err(EntityError::InvalidEntityArgs {
            field,
            violation: "EmptyString",
        }),
    }
}

/// Error states resulting from converting [NaiveDateTime] + [Tz]
/// into [DateTime<Utc>]
/// This situation is caused by the existence of DST [Daylight savings time](https://en.wikipedia.org/wiki/Daylight_saving_time)
#[derive(Debug)]
pub enum DateTimeConversionError {
    /// Conversion returns an impossible time
    NonExistent,
    /// Conversion results in 2 possible values.
    /// Current decision - Return to user and let the user pick
    Ambiguous(DateTime<Utc>, DateTime<Utc>),
}

/// Pick a time instant that is the same irregardless of [Tz]
#[derive(Debug)]
pub struct DateTimeSelection {
    /// The time instant
    datetime: NaiveDateTime,
    /// Relative to this timezone
    timezone: Tz,
}

impl DateTimeSelection {
    fn new(datetime: NaiveDateTime, timezone: Tz) -> Self {
        Self { datetime, timezone }
    }

    /// Convert the time instant to a time instance
    /// with respect to UTC
    fn to_instant(self) -> Result<DateTime<Utc>, DateTimeConversionError> {
        /// For more description of the method's functionality,
        /// see its definition
        match self.timezone.from_local_datetime(&self.datetime) {
            LocalResult::Single(dt) => Ok(dt.with_timezone(&Utc)),
            LocalResult::Ambiguous(a, b) => Err(DateTimeConversionError::Ambiguous(
                a.with_timezone(&Utc),
                b.with_timezone(&Utc),
            )),
            LocalResult::None => Err(DateTimeConversionError::NonExistent),
        }
    }
}

impl PartialEq for DateTimeSelection {
    fn eq(&self, other: &Self) -> bool {
        if self.timezone == other.timezone {
            self.datetime == other.datetime
        } else {
            todo!("Implement PartialEq for different values of 'Tz' and 'NaiveDateTime'")
        }
    }
}

impl PartialOrd for DateTimeSelection {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.timezone == other.timezone {
            self.datetime.partial_cmp(&other.datetime)
        } else {
            todo!("Implement PartialOrd for different values of 'Tz' and 'NaiveDateTime'")
        }
    }
}

#[cfg(test)]
mod non_empty_string_tests {
    use super::*;

    #[test]
    fn new_valid_non_empty_string_passes() {
        assert!(NonEmptyString::new("Foo").is_some());
        assert!(NonEmptyString::new(String::from("Bar")).is_some())
    }

    #[test]
    fn empty_non_empty_string_rejects() {
        assert!(NonEmptyString::new("").is_none());
    }
}

#[cfg(test)]
mod date_time_selection_tests {
    use chrono::{NaiveDate, NaiveTime};

    use super::{DateTime, DateTimeConversionError, DateTimeSelection, NaiveDateTime, Tz};

    #[test]
    fn cmp_date_time_selection_works() {
        let time_a = DateTimeSelection {
            datetime: NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2020, 06, 01).unwrap(),
                NaiveTime::from_hms_opt(13, 30, 00).unwrap(),
            ),
            timezone: Tz::America__New_York,
        }; // 01 June 2020: 1:30:00 PM 
        let time_b = DateTimeSelection {
            datetime: NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2020, 06, 01).unwrap(),
                NaiveTime::from_hms_opt(14, 30, 00).unwrap(),
            ),
            timezone: Tz::America__New_York,
        }; // 01 June 2020: 1:30:00 PM
        let time_c = DateTimeSelection {
            datetime: NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2020, 06, 01).unwrap(),
                NaiveTime::from_hms_opt(13, 30, 00).unwrap(),
            ),
            timezone: Tz::America__New_York,
        }; // 01 June 2020: 1:30:00 PM
        assert!(time_a < time_b);
        assert!(time_a == time_c);
    }
}
