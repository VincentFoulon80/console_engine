use crate::forms::FormOutput;

use super::FormConstraint;

/// # Not Blank Constraint
///
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
    fn validate(&self, output: &FormOutput) -> bool {
        match output {
            FormOutput::Nothing => false,
            FormOutput::String(value) => !value.is_empty(),
            FormOutput::Compound(fields) => !fields.is_empty(),
        }
    }

    fn get_message(&self) -> &str {
        &self.message
    }
}

#[cfg(test)]
mod test {
    use crate::forms::constraints::FormConstraint;
    use crate::forms::FormOutput;
    use std::collections::HashMap;

    #[test]
    fn not_blank() {
        use super::NotBlank;

        let validator = NotBlank::new("Blank");

        assert!(!validator.validate(&FormOutput::Nothing));
        assert!(!validator.validate(&FormOutput::String(String::from(""))));
        assert!(validator.validate(&FormOutput::String(String::from("hello, world!"))));

        let mut hm: HashMap<String, FormOutput> = HashMap::new();
        assert!(!validator.validate(&FormOutput::Compound(hm.clone())));
        hm.insert(String::from("1"), FormOutput::Nothing);
        assert!(validator.validate(&FormOutput::Compound(hm)));
    }
}
