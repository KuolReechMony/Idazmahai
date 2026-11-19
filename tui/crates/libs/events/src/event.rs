//! Defines [Event] object

use primitives::models::{DateTimeSelection, Location, NonEmptyString};
use rrule::RRuleSet;

#[derive(Debug)]
pub struct Event {
    id: i64,
    season_id: i64,
    event_title: NonEmptyString,
    event_description: NonEmptyString,
    event_location: Location,
    start: DateTimeSelection,
    // Check now() < both start and end && start < end
    end: DateTimeSelection,
    // Check DTSTART and DTEND are < than now()
    recurring: RRuleSet,
    // Time in minutes before the start time. Every reminder must be between now() and start
    reminders: Vec<u16>,
}
