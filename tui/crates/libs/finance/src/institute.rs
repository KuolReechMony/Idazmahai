use primitives::models::{Location, NonEmptyString};
use url::Url;

#[derive(Debug)]
pub struct FinanceInstitute {
    id: i64,
    institute_title: NonEmptyString,
    /// In-person branch location
    institute_location: Location,
    website: Url,
    live_support: NonEmptyString,
    fraud_support: NonEmptyString,
}
