use crate::models::text::ValidatedText;

/// Abstraction for location data types
#[derive(Debug)]
pub struct Location {
    /// Currently a [`ValidatedText`].
    /// Planned to be a `ValidatedAddressLine`
    address: ValidatedText,
    unit: Option<ValidatedText>,
}

impl Location {
    fn new(address: ValidatedText, unit: Option<ValidatedText>) -> Self {
        Self { address, unit }
    }

    const fn address_line(&self) -> &str {
        self.address.as_str()
    }

    fn unit(&self) -> Option<&str> {
        match &self.unit {
            Some(unit) => Some(unit.as_str()),
            None => None,
        }
    }
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.address.as_str() == other.address.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmp_location_works() {
        let here = Location::new(ValidatedText::new("1st Street").unwrap(), None);
        let there = Location::new(
            ValidatedText::new("2nd Street").unwrap(),
            Some(ValidatedText::new("Unit 1").unwrap()),
        );
        assert_eq!(here.address_line(), "1st Street");
        assert!(here.unit().is_none());
        assert_eq!(there.unit().unwrap(), "Unit 1")
    }
}
