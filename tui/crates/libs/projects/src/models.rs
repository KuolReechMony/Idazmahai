use chrono::NaiveDate;
use primitives::models::NonEmptyString;

#[derive(Debug)]
pub enum TaskCompletionStatus {
    NotStarted,
    InProgress,
    OnHold {
        reason: NonEmptyString,
        review_date: NaiveDate,
    },
    Completed,
    Cancelled,
}
