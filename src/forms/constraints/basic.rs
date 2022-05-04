use crate::forms::FormOutput;

use super::FormConstraint;

pub struct NotBlank {
    message: String,
}

impl NotBlank {
    pub fn new(message: &str) -> Box<Self> {
        Box::new(NotBlank {
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
