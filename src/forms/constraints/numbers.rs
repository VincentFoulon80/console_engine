use crate::forms::FormValue;

use super::FormConstraint;

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
    fn validate(&self, output: &FormValue) -> bool {
        match output {
            FormValue::Nothing => true,
            FormValue::Boolean(_) => true,
            FormValue::Index(_) => true,
            FormValue::String(value) => {
                if value.is_empty() {
                    return false;
                }
                if let Some(chr) = value.chars().next() {
                    if !chr.is_digit(10) && chr != '-' && chr != '+' {
                        return false;
                    }
                }
                value.chars().skip(1).all(|x| x.is_digit(10))
            }
            FormValue::Map(entries) => entries.iter().all(|(_, x)| self.validate(x)),
            FormValue::List(entries) => entries
                .iter()
                .all(|x| self.validate(&FormValue::String(String::from(x)))),
            FormValue::Vec(entries) => entries.iter().all(|x| self.validate(x)),
        }
    }

    fn get_message(&self) -> &str {
        &self.message
    }
}

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
    fn validate(&self, output: &FormValue) -> bool {
        match output {
            FormValue::Nothing => true,
            FormValue::Boolean(_) => true,
            FormValue::Index(_) => true,
            FormValue::String(value) => value.parse::<f32>().is_ok(),
            FormValue::Map(fields) => fields.iter().all(|(_, x)| self.validate(x)),
            FormValue::List(entries) => entries
                .iter()
                .all(|x| self.validate(&FormValue::String(String::from(x)))),
            FormValue::Vec(entries) => entries.iter().all(|x| self.validate(x)),
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
    fn integer() {
        use super::Integer;

        let validator = Integer::new("not integer");

        assert!(validator.validate(&FormValue::Nothing));
        assert!(!validator.validate(&FormValue::String(String::from("hello, world!"))));
        assert!(validator.validate(&FormValue::String(String::from("37"))));
        assert!(validator.validate(&FormValue::String(String::from("-35"))));
        assert!(!validator.validate(&FormValue::String(String::from("3.5"))));
        assert!(validator.validate(&FormValue::String(String::from(
            "9999999999999999999999999999999999999999999999999"
        ))));
        assert!(!validator.validate(&FormValue::String(String::from("3e-5"))));

        let mut hm: HashMap<String, FormValue> = HashMap::new();
        assert!(validator.validate(&FormValue::Map(hm.clone())));
        hm.insert(String::from("1"), FormValue::Nothing);
        assert!(validator.validate(&FormValue::Map(hm.clone())));
        hm.insert(String::from("2"), FormValue::String(String::from("37")));
        assert!(validator.validate(&FormValue::Map(hm.clone())));
        hm.insert(String::from("3"), FormValue::String(String::from("-35")));
        assert!(validator.validate(&FormValue::Map(hm.clone())));
        hm.insert(String::from("4"), FormValue::String(String::from("3.5")));
        assert!(!validator.validate(&FormValue::Map(hm.clone())));
        hm.insert(String::from("5"), FormValue::String(String::from("-3.5")));
        assert!(!validator.validate(&FormValue::Map(hm)));
    }

    #[test]
    fn number() {
        use super::Number;

        let validator = Number::new("not number");

        assert!(validator.validate(&FormValue::Nothing));
        assert!(!validator.validate(&FormValue::String(String::from("hello, world!"))));
        assert!(validator.validate(&FormValue::String(String::from("37"))));
        assert!(validator.validate(&FormValue::String(String::from("-35"))));
        assert!(validator.validate(&FormValue::String(String::from("3.5"))));
        assert!(validator.validate(&FormValue::String(String::from("-3.5"))));
        assert!(validator.validate(&FormValue::String(String::from(
            "9999999999999999999999999999999999999999999999999"
        ))));
        assert!(validator.validate(&FormValue::String(String::from("3e-5"))));

        let mut hm: HashMap<String, FormValue> = HashMap::new();
        assert!(validator.validate(&FormValue::Map(hm.clone())));
        hm.insert(String::from("1"), FormValue::Nothing);
        assert!(validator.validate(&FormValue::Map(hm.clone())));
        hm.insert(String::from("2"), FormValue::String(String::from("37")));
        assert!(validator.validate(&FormValue::Map(hm.clone())));
        hm.insert(String::from("3"), FormValue::String(String::from("-35")));
        assert!(validator.validate(&FormValue::Map(hm.clone())));
        hm.insert(String::from("4"), FormValue::String(String::from("3.5")));
        assert!(validator.validate(&FormValue::Map(hm.clone())));
        hm.insert(String::from("5"), FormValue::String(String::from("-3.5")));
        assert!(validator.validate(&FormValue::Map(hm)));
    }
}
