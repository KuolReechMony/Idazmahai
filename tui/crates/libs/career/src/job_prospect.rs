//! Defines a prospective job position. It has to be a job posting or a position that is guaranteed
//! to be available in the near future (preferably in less than 1-2 years)

use chrono::{NaiveDate, Utc};
use primitives::{
    errors::EntityError,
    models::{NonEmptyString, check_field_string_not_empty},
};
use url::Url;

use crate::models::{JobEnvironment, JobType};

/// Used to create the actual [JobProspect] in the `db`
#[derive(Debug)]
pub struct NewJobProspect {
    role_title: NonEmptyString,
    employer_title: NonEmptyString,
    employer_location: NonEmptyString,
    prospect_url: Url,
    role_environment: JobEnvironment,
    role_type: JobType,
    required_qualifications: Vec<NonEmptyString>,
    preffered_qualifications: Vec<NonEmptyString>,
    role_description: Vec<NonEmptyString>,
    review_deadline: NaiveDate,
}

#[derive(Debug)]
pub struct JobProspect {
    // Database autoincrement id
    id: i64,
    role_title: NonEmptyString,
    employer_title: NonEmptyString,
    employer_location: NonEmptyString,
    prospect_url: Url,
    role_environment: JobEnvironment,
    role_type: JobType,
    required_qualifications: Vec<NonEmptyString>,
    preffered_qualifications: Vec<NonEmptyString>,
    role_description: Vec<NonEmptyString>,
    review_deadline: NaiveDate,
}

impl JobProspect {
    fn create(
        role_title: String,
        employer_title: String,
        employer_location: String,
        prospect_url: String,
        role_environment: JobEnvironment,
        role_type: JobType,
        required_qualifications: Vec<String>,
        preffered_qualifications: Vec<String>,
        role_description: Vec<String>,
        review_deadline: NaiveDate,
    ) -> Result<(), EntityError> {
        let prospect_url = match Url::parse(&prospect_url) {
            Ok(url) => Ok(url),
            Err(_) => Err(EntityError::InvalidEntityArgs {
                field: "prospect_url",
                violation: "NonUrlString",
            }),
        };
        let review_deadline = if Utc::now().date_naive() < review_deadline {
            Ok(review_deadline)
        } else {
            Err(EntityError::InvalidEntityArgs {
                field: "review_deadline",
                violation: "DateElapsed",
            })
        };
        let prospect = NewJobProspect {
            role_title: check_field_string_not_empty(role_title, "role_title")?,
            employer_title: check_field_string_not_empty(employer_title, "employer_title")?,
            employer_location: check_field_string_not_empty(
                employer_location,
                "employer_location",
            )?,
            prospect_url: prospect_url?,
            role_environment,
            role_type,
            required_qualifications: NonEmptyString::from_string_vec(required_qualifications),
            preffered_qualifications: NonEmptyString::from_string_vec(preffered_qualifications),
            role_description: NonEmptyString::from_string_vec(role_description),
            review_deadline: review_deadline?,
        };
        Ok(())
    }
}
