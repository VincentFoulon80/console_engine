use crate::forms::FormValue;

use super::FormConstraint;

/// Validates that data exists and is not empty
pub struct NotBlank {
    message: String,
}

impl NotBlank {
    pub fn new(message: &str) -> Box<Self> {
        Box::new(Self {
            message: String::from(message),
        })
    }
}

impl FormConstraint for NotBlank {
    fn validate(&self, output: &FormValue) -> bool {
        match output {
            FormValue::Nothing => false,
            FormValue::Boolean(_) => true,
            FormValue::Index(_) => true,
            FormValue::String(value) => !value.is_empty(),
            FormValue::Map(entries) => !entries.is_empty(),
            FormValue::List(entries) => !entries.is_empty(),
            FormValue::Vec(entries) => !entries.is_empty(),
        }
    }

    fn get_message(&self) -> &str {
        &self.message
    }
}

#[cfg(test)]
mod test {
    use crate::forms::constraints::FormConstraint;
    use crate::forms::FormValue;
    use std::collections::HashMap;

    #[test]
    fn not_blank() {
        use super::NotBlank;

        let validator = NotBlank::new("Blank");

        assert!(!validator.validate(&FormValue::Nothing));
        assert!(!validator.validate(&FormValue::String(String::from(""))));
        assert!(validator.validate(&FormValue::String(String::from("hello, world!"))));

        let mut hm: HashMap<String, FormValue> = HashMap::new();
        assert!(!validator.validate(&FormValue::Map(hm.clone())));
        hm.insert(String::from("1"), FormValue::Nothing);
        assert!(validator.validate(&FormValue::Map(hm.clone())));
    }
}
