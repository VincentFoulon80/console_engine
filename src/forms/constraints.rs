use super::FormOutput;
pub trait FormConstraint {
    fn validate(&self, output: &FormOutput) -> bool;

    fn get_message(&self) -> &str;
}

pub struct NotEmpty {
    message: String,
}

impl NotEmpty {
    pub fn new(message: &str) -> Self {
        NotEmpty {
            message: String::from(message),
        }
    }
}

impl FormConstraint for NotEmpty {
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
