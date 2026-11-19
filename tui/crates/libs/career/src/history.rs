use chrono::NaiveDate;
use primitives::models::NonEmptyString;

///! A historical of past and currently held jobs
/// Targetted use cases for this records are:
/// 1. Easily generate [JobApplication] documents (resumes, employment history)

#[derive(Debug)]
pub struct EmploymentRecord {
    /// Database autoincement id
    id: i64,

    /// Mapping for historical records
    job_prospect_id: i64,
    job_application_id: i64,

    department: NonEmptyString,
    start: NaiveDate,
    /// For currently held positions
    end: Option<NaiveDate>,
}
