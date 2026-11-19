//! Defines a prospective position that has been taken into action in the
//! form of a job application. Includes anything that has been drafted
//! till a job offer or a position not gotten.

use crate::models::JobApplicationStatus;

/// A [JobProspect] that has been acted on
#[derive(Debug)]
pub struct JobApplication {
    /// Database autoincrement id
    id: i64,
    /// A job application MUST be tied to a job prospect
    job_prospect_id: i64,
    application_status: JobApplicationStatus,
}
