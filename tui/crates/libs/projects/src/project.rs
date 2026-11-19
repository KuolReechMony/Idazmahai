use primitives::models::{DateTimeSelection, NonEmptyString};

use crate::models::TaskCompletionStatus;

#[derive(Debug)]
pub struct Project {
    id: i64,
    project_title: NonEmptyString,
    project_description: NonEmptyString,
    completion_status: TaskCompletionStatus,
    started: DateTimeSelection,
    completed: Option<DateTimeSelection>,
    pre_requisites: Vec<i64>,
}
