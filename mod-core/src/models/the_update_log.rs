use std::collections::LinkedList;

#[derive(Debug)]
pub struct UpdateLog {
    /// Desired outcome from this case
    target: String,
    updates: LinkedList<String>,
}

impl UpdateLog {
    fn new(desired_case_outcome: String) -> Result<Self, &'static str> {
        if desired_case_outcome.trim().is_empty() {
            Err("The desired case outcome cannot be empty")
        } else {
            Ok(Self {
                target: desired_case_outcome,
                updates: LinkedList::new(),
            })
        }
    }

    fn provide_update(&mut self, new_update: String) {
        self.updates.push_back(new_update);
    }

    fn read_target(&self) -> &str {
        self.target.as_str()
    }

    fn read_update_by_index(&self, update_index: u32) -> Option<&str> {
        self.updates
            .iter()
            .nth(update_index as usize)
            .map(|s| s.as_str())
    }

    fn read_last_update(&self) -> Option<&str> {
        self.read_update_by_index((self.updates.len() - 1) as u32)
    }

    fn reset_updates(&mut self) {
        self.updates.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_update_log_passes_for_valid() {
        let valid_update_log = UpdateLog::new(String::from("Sample target")).unwrap();
        assert_eq!(valid_update_log.target.as_str(), "Sample target")
    }

    #[test]
    fn new_update_log_rejects_empty_target() {
        assert_eq!(
            UpdateLog::new(String::new()).unwrap_err(),
            "The desired case outcome cannot be empty"
        )
    }

    #[test]
    fn provide_update_to_update_log_passes_for_valid() {
        let mut valid_update = UpdateLog::new(String::from("Sample target")).unwrap();
        valid_update.provide_update(String::from("Sample update msg"));
        assert_eq!(
            valid_update.read_update_by_index(0).unwrap(),
            "Sample update msg"
        );
        assert_eq!(
            valid_update.read_last_update().unwrap(),
            "Sample update msg"
        )
    }

    #[test]
    fn provide_update_rejects_empty_new_update() {}
}
