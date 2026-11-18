use thiserror::Error;

/// Capture errors resulting from CRUD operations on the object
#[derive(Debug, Error)]
pub enum EntityError {
    /// Capture constraints expressed in the object type system
    #[error("Entity's {field:?} has a {violation:?}")]
    InvalidEntityArgs {
        field: &'static str,
        violation: &'static str,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_error_display_impl_works() {
        let e = EntityError::InvalidEntityArgs {
            field: "field 1",
            violation: "some violation",
        };
        assert_eq!(
            e.to_string(),
            "Entity's \"field 1\" has a \"some violation\""
        )
    }
}
