use crate::forms::FormOutput;

use super::FormConstraint;

pub struct Integer {
    message: String,
}

impl Integer {
    #[allow(dead_code)]
    pub fn new(message: &str) -> Box<Self> {
        Box::new(Integer {
            message: String::from(message),
        })
    }
}

impl FormConstraint for Integer {
    fn validate(&self, output: &FormOutput) -> bool {
        match output {
            FormOutput::Nothing => true,
            FormOutput::String(value) => {
                if let Some(chr) = value.chars().next() {
                    if !chr.is_numeric() && chr != '-' && chr != '+' {
                        return false;
                    }
                }
                value.chars().skip(1).all(|x| x.is_numeric())
            }
            FormOutput::Compound(fields) => fields.iter().all(|(_, x)| self.validate(x)),
        }
    }

    fn get_message(&self) -> &str {
        &self.message
    }
}

pub struct Number {
    message: String,
}

impl Number {
    pub fn new(message: &str) -> Box<Self> {
        Box::new(Number {
            message: String::from(message),
        })
    }
}

impl FormConstraint for Number {
    fn validate(&self, output: &FormOutput) -> bool {
        match output {
            FormOutput::Nothing => true,
            FormOutput::String(value) => {
                if let Some(chr) = value.chars().next() {
                    if !chr.is_numeric() && chr != '-' && chr != '+' {
                        return false;
                    }
                }
                value
                    .chars()
                    .skip(1)
                    .all(|x| x.is_numeric() || x == '.' || x == ',')
                    && value.chars().filter(|&x| x == '.' || x == ',').count() <= 1
            }
            FormOutput::Compound(fields) => fields.iter().all(|(_, x)| self.validate(x)),
        }
    }

    fn get_message(&self) -> &str {
        &self.message
    }
}
