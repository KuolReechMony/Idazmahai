use chrono_tz::Tz;

pub struct UserConfig {
    /// Used to determine the date and time when
    /// planning future events, tasks etc
    timezone: Tz,
}
