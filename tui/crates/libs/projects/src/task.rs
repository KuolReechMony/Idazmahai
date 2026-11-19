use primitives::models::{DateTimeSelection, NonEmptyString};

use crate::models::TaskCompletionStatus;

#[derive(Debug)]
pub struct Task {
    id: i64,
    project_id: i64,
    task_title: NonEmptyString,
    task_description: NonEmptyString,
    completion_status: TaskCompletionStatus,
    started: DateTimeSelection,
    completed: Option<DateTimeSelection>,
    /// Ids for pre_requisite_tasks
    pre_requisite_tasks: Vec<i64>,
}
