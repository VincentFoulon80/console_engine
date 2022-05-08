use crate::forms::FormOutput;

use super::FormConstraint;

/// # Integer Constraint
///
/// Validates that the input is an integer (positive or negative)
pub struct Integer {
    message: String,
}

impl Integer {
    pub fn new(message: &str) -> Box<Self> {
        Box::new(Self {
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
                    if !chr.is_digit(10) && chr != '-' && chr != '+' {
                        return false;
                    }
                }
                value.chars().skip(1).all(|x| x.is_digit(10))
            }
            FormOutput::Compound(fields) => fields.iter().all(|(_, x)| self.validate(x)),
        }
    }

    fn get_message(&self) -> &str {
        &self.message
    }
}

/// # Number Constraint
///
/// Validates that the input is a number, allowing one comma or dot if it's a decimal number
pub struct Number {
    message: String,
}

impl Number {
    pub fn new(message: &str) -> Box<Self> {
        Box::new(Self {
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

#[cfg(test)]
mod test {
    use crate::forms::constraints::FormConstraint;
    use crate::forms::FormOutput;
    use std::collections::HashMap;

    #[test]
    fn integer() {
        use super::Integer;

        let validator = Integer::new("not integer");

        assert!(validator.validate(&FormOutput::Nothing));
        assert!(!validator.validate(&FormOutput::String(String::from("hello, world!"))));
        assert!(validator.validate(&FormOutput::String(String::from("37"))));
        assert!(validator.validate(&FormOutput::String(String::from("-35"))));
        assert!(!validator.validate(&FormOutput::String(String::from("3.5"))));

        let mut hm: HashMap<String, FormOutput> = HashMap::new();
        assert!(validator.validate(&FormOutput::Compound(hm.clone())));
        hm.insert(String::from("1"), FormOutput::Nothing);
        assert!(validator.validate(&FormOutput::Compound(hm.clone())));
        hm.insert(String::from("2"), FormOutput::String(String::from("37")));
        assert!(validator.validate(&FormOutput::Compound(hm.clone())));
        hm.insert(String::from("3"), FormOutput::String(String::from("-35")));
        assert!(validator.validate(&FormOutput::Compound(hm.clone())));
        hm.insert(String::from("4"), FormOutput::String(String::from("3.5")));
        assert!(!validator.validate(&FormOutput::Compound(hm.clone())));
        hm.insert(String::from("5"), FormOutput::String(String::from("-3.5")));
        assert!(!validator.validate(&FormOutput::Compound(hm)));
    }

    #[test]
    fn number() {
        use super::Number;

        let validator = Number::new("not number");

        assert!(validator.validate(&FormOutput::Nothing));
        assert!(!validator.validate(&FormOutput::String(String::from("hello, world!"))));
        assert!(validator.validate(&FormOutput::String(String::from("37"))));
        assert!(validator.validate(&FormOutput::String(String::from("-35"))));
        assert!(validator.validate(&FormOutput::String(String::from("3.5"))));
        assert!(validator.validate(&FormOutput::String(String::from("-3.5"))));

        let mut hm: HashMap<String, FormOutput> = HashMap::new();
        assert!(validator.validate(&FormOutput::Compound(hm.clone())));
        hm.insert(String::from("1"), FormOutput::Nothing);
        assert!(validator.validate(&FormOutput::Compound(hm.clone())));
        hm.insert(String::from("2"), FormOutput::String(String::from("37")));
        assert!(validator.validate(&FormOutput::Compound(hm.clone())));
        hm.insert(String::from("3"), FormOutput::String(String::from("-35")));
        assert!(validator.validate(&FormOutput::Compound(hm.clone())));
        hm.insert(String::from("4"), FormOutput::String(String::from("3.5")));
        assert!(validator.validate(&FormOutput::Compound(hm.clone())));
        hm.insert(String::from("5"), FormOutput::String(String::from("-3.5")));
        assert!(validator.validate(&FormOutput::Compound(hm)));
    }
}
