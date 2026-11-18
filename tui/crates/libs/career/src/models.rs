//! Defines primitives types for the `Career` module

use chrono::DateTime;
use chrono_tz::Tz;

const MIN_HOURLY_WAGE: f32 = 7.5;
const YEARLY_WORK_WEEKS: u8 = 48;

/// Tracks stage of job application.
/// Is meant to be a one-way process (No reverse direction travel)
#[derive(Debug)]
pub enum JobApplicationStatus {
    /// Application has not been sent to the employer
    Draft,
    /// Application has been sent to the emmployer
    Submitted,
    /// Employer has responded. This does not included auto-generated responses
    UnderReview,
    /// Includes from time assessments are planned till when their results are
    /// received. Assessments include: Interviews(Virtual, In-person), Online tests, etc
    Assessment,
    /// Employer has submitted a job offer
    /// Includes the offer's expiry timestamp
    Offer { offer_expiry: DateTime<Tz> },
    /// Job application withdrawn by user.
    Withdrawn,
    /// User not selected for the position
    NotSelected,
}

/// Work environment types
#[derive(Debug, Clone)]
pub enum JobEnvironment {
    OnsiteOnly,
    RemoteOnly,
    OnsiteAndRemote,
}

/// Work type
#[derive(Debug, Clone)]
pub enum JobType {
    FullTime { salary: u32 },
    PartTime { hourly_rate: f32, weekly_hours: u8 },
}

impl JobType {
    /// Returns total yearly compensation
    fn compensation(&self) -> u32 {
        match self {
            Self::FullTime { salary } => *salary,
            Self::PartTime {
                hourly_rate,
                weekly_hours,
            } => (*hourly_rate * (*weekly_hours as f32) * (YEARLY_WORK_WEEKS as f32)) as u32,
        }
    }
}

impl PartialEq for JobType {
    fn eq(&self, other: &Self) -> bool {
        self.compensation() == other.compensation()
    }
}

impl PartialOrd for JobType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.compensation().partial_cmp(&other.compensation())
    }
}

#[cfg(test)]
mod tests {
    use super::JobType;

    #[test]
    fn compensation_comparison_works() {
        let ft_1 = JobType::FullTime {
            salary: 57_600 as u32,
        };
        let ft_2 = JobType::FullTime {
            salary: 150_000 as u32,
        };
        let hourly1 = JobType::PartTime {
            hourly_rate: 25 as f32,
            weekly_hours: 40 as u8,
        };
        let hourly2 = JobType::PartTime {
            hourly_rate: 30 as f32,
            weekly_hours: 40 as u8,
        };
        assert!(ft_1 < ft_2);
        assert!(ft_1 > hourly1);
        assert!(hourly1 < hourly2);
        assert!(hourly2 == ft_1)
    }
}
