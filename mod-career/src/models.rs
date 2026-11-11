use chrono::NaiveDateTime;

pub const WEEKS_IN_YEAR: u8 = 52;
pub const MINIMUM_HOURLY_WAGE: f32 = 7.25;

/// Compensation Models
#[derive(Debug)]
pub enum CompensationModel {
    Hourly { rate: f32, hours: u8 },
    Annually { salary: u32 },
}

impl CompensationModel {
    fn new_hourly_comp(
        hourly_rate: f32,
        hours_per_week: u8,
    ) -> Result<CompensationModel, &'static str> {
        if hours_per_week > 168 || hours_per_week <= 0 {
            Err("Weekly work hours must be between 0 and 168")
        } else if hourly_rate < MINIMUM_HOURLY_WAGE {
            Err("Hourly pay rate below state(IN) minimum wage")
        } else {
            Ok(CompensationModel::Hourly {
                rate: hourly_rate,
                hours: hours_per_week,
            })
        }
    }

    fn total(&self) -> f32 {
        match self {
            CompensationModel::Hourly {
                rate: hourly_rate,
                hours: no_weekly_hours,
            } => hourly_rate * (*no_weekly_hours as f32) * WEEKS_IN_YEAR as f32,
            CompensationModel::Annually {
                salary: annual_salary,
            } => *annual_salary as f32,
        }
    }
}

impl PartialEq for CompensationModel {
    fn eq(&self, other: &Self) -> bool {
        self.total() == other.total()
    }
}

impl PartialOrd for CompensationModel {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.total().partial_cmp(&other.total())
    }
}

/// Variants of work locations and environments
#[derive(Debug, PartialEq)]
pub enum RoleWorkEnvironment {
    OnsiteOnly,
    RemoteOnly,
    OnsiteAndRemote,
}

/// Variants of role application statuses
#[derive(Debug, PartialEq)]
pub enum RoleApplicationStatus {
    Draft,
    Submitted,
    /// Submitted applications are [RoleApplicationStatus::Submitted] until
    /// the employer reaches out then they move onto [RoleApplicationStatus::UnderReview] or
    /// [RoleApplicationStatus::NotSelected]
    UnderReview,
    Interview,
    JobOffer,
    Hired,
    Withdrawn,
    NotSelected,
}

/// Potential roles at some specific employer
#[derive(Debug)]
pub struct ProspectiveRole {
    // Database id
    id: u32,
    role_title: String,
    employer: String,
    responsibilities: Vec<String>,
    required_qualifications: Vec<String>,
    preffered_qualifications: Vec<String>,
    due_date: NaiveDateTime,
}

/// Application for a `ProspectiveRole`
pub struct RoleApplication {
    // Database id
    id: u32,
    role_prospect_id: u32,
    application_status: RoleApplicationStatus,
}
