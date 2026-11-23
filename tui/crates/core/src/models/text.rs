/// Abstraction for text input validation
#[derive(Debug, Clone)]
pub struct ValidatedText {
    s: String,
}

impl ValidatedText {
    /// Rejects empty `String`s and accounts for whitespace
    #[inline]
    pub fn new(s: impl Into<String>) -> Option<Self> {
        let s = s.into();
        let trimmed = s.trim();
        if trimmed.len() > 0 {
            Some(Self {
                s: trimmed.to_owned(),
            })
        } else {
            None
        }
    }

    #[inline]
    pub const fn as_str(&self) -> &str {
        self.s.as_str()
    }

    #[inline]
    pub fn into_inner(self) -> String {
        self.s
    }

    /// Filter any input that can be turned into an [Iterator] that yields
    /// values (`impl Into<String>`) that can be turned into [String] then yield
    /// only [ValidatedText] values.
    /// Takes ownership of input so `Invalid Text` is lost
    ///
    /// # Example
    /// ```
    /// // let v: &[&str] = ["One", "  Three " , "Two", ""];
    /// // let s = v[1:3];
    /// // let t = String::from("text");
    /// // let k = v.into_iter().retain_valid();
    /// ```
    #[inline]
    fn retain_valid<I>(iter: I) -> Vec<Self>
    where
        I: IntoIterator,
        I::Item: AsRef<str> + Into<String>,
    {
        iter.into_iter()
            .filter_map(|v| ValidatedText::new(v))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::ValidatedText;

    #[test]
    fn new_accepts_valid_text_input() {
        let valid_text = String::from("Valid text");
        assert!(ValidatedText::new(valid_text.clone()).is_some());
        assert!(ValidatedText::new(String::new()).is_none())
        // assert!(ValidatedText::new(valid_text.as_bytes()).is_some())
    }

    #[test]
    fn self_as_str_works() {
        let s = String::from("string");
        let v = ValidatedText::new(s).unwrap();
        assert_eq!(v.as_str(), "string")
    }

    #[test]
    fn self_as_string_works() {
        let s = String::from("string");
        let v = ValidatedText::new(s).unwrap();
        assert_eq!(v.into_inner(), String::from("string"))
    }

    #[test]
    fn iter_works() {
        let v: &[&str] = &["One   ", "", " Two  ", "Three", "Four", ""];
        assert_eq!(ValidatedText::retain_valid(v.to_vec()).len(), 4);
        assert_eq!(
            ValidatedText::retain_valid(v.to_vec())[0]
                .clone()
                .into_inner(),
            ValidatedText::new("One").unwrap().into_inner()
        );
        assert_eq!(
            ValidatedText::retain_valid(v.to_vec())[1]
                .clone()
                .into_inner(),
            ValidatedText::new("Two").unwrap().into_inner()
        )
    }
}
