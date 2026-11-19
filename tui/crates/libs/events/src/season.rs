use primitives::models::{DateTimeSelection, NonEmptyString};
use rrule::RRuleSet;

/// Defines events that span over more than a couple of days
/// and have many different constituent events as part of it
#[derive(Debug)]
pub struct Season {
    id: i64,
    season_title: NonEmptyString,
    season_description: NonEmptyString,
    season_location: NonEmptyString,
    start: DateTimeSelection,
    end: DateTimeSelection,
    recurring: RRuleSet,
}
