use crate::forms::FormValue;

use super::FormConstraint;

/// Checks if the input only contains alphabetic characters as returned by [is_alphabetic](https://doc.rust-lang.org/std/primitive.char.html#method.is_alphabetic)
pub struct Alphabetic {
    message: String,
}

impl Alphabetic {
    pub fn new(message: &str) -> Box<Self> {
        Box::new(Self {
            message: String::from(message),
        })
    }
}

impl FormConstraint for Alphabetic {
    fn validate(&self, output: &FormValue) -> bool {
        match output {
            FormValue::String(value) => value.chars().all(|x| x.is_alphabetic()),
            FormValue::Map(fields) => fields.iter().all(|(_, x)| self.validate(x)),
            FormValue::List(entries) => entries
                .iter()
                .all(|x| self.validate(&FormValue::String(String::from(x)))),
            FormValue::Vec(entries) => entries.iter().all(|x| self.validate(x)),
            // we don't support all FormValues
            _ => false,
        }
    }

    fn get_message(&self) -> &str {
        &self.message
    }
}

/// Checks if the input only contains alphanumeric characters as returned by [is_alphanumeric](https://doc.rust-lang.org/std/primitive.char.html#method.is_alphanumeric)
pub struct Alphanumeric {
    message: String,
}

impl Alphanumeric {
    pub fn new(message: &str) -> Box<Self> {
        Box::new(Self {
            message: String::from(message),
        })
    }
}

impl FormConstraint for Alphanumeric {
    fn validate(&self, output: &FormValue) -> bool {
        match output {
            FormValue::String(value) => value.chars().all(|x| x.is_alphanumeric()),
            FormValue::Map(fields) => fields.iter().all(|(_, x)| self.validate(x)),
            FormValue::List(entries) => entries
                .iter()
                .all(|x| self.validate(&FormValue::String(String::from(x)))),
            FormValue::Vec(entries) => entries.iter().all(|x| self.validate(x)),
            // we don't support all FormValues
            _ => false,
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
    fn alphabetic() {
        use super::Alphabetic;

        let validator = Alphabetic::new("should be alphabetic");

        assert!(!validator.validate(&FormValue::Nothing));
        assert!(validator.validate(&FormValue::String(String::from("Helloworld"))));
        assert!(!validator.validate(&FormValue::String(String::from("123"))));
        assert!(!validator.validate(&FormValue::String(String::from("Hello123"))));
        assert!(!validator.validate(&FormValue::String(String::from("hello, world!"))));

        let mut hm: HashMap<String, FormValue> = HashMap::new();
        hm.insert(String::from("1"), FormValue::Nothing);
        assert!(!validator.validate(&FormValue::Map(hm)));

        let mut hm: HashMap<String, FormValue> = HashMap::new();
        hm.insert(
            String::from("2"),
            FormValue::String(String::from("Helloworld")),
        );
        assert!(validator.validate(&FormValue::Map(hm.clone())));
        hm.insert(String::from("3"), FormValue::String(String::from("123")));
        assert!(!validator.validate(&FormValue::Map(hm.clone())));
    }

    #[test]
    fn alphanumeric() {
        use super::Alphanumeric;

        let validator = Alphanumeric::new("should be alphanumeric");

        assert!(!validator.validate(&FormValue::Nothing));
        assert!(validator.validate(&FormValue::String(String::from("Helloworld"))));
        assert!(validator.validate(&FormValue::String(String::from("123"))));
        assert!(validator.validate(&FormValue::String(String::from("Hello123"))));
        assert!(!validator.validate(&FormValue::String(String::from("hello, world!"))));

        let mut hm: HashMap<String, FormValue> = HashMap::new();
        hm.insert(String::from("1"), FormValue::Nothing);
        assert!(!validator.validate(&FormValue::Map(hm)));

        let mut hm: HashMap<String, FormValue> = HashMap::new();
        hm.insert(
            String::from("2"),
            FormValue::String(String::from("Helloworld")),
        );
        assert!(validator.validate(&FormValue::Map(hm.clone())));
        hm.insert(String::from("3"), FormValue::String(String::from("123")));
        assert!(validator.validate(&FormValue::Map(hm.clone())));
        hm.insert(
            String::from("3"),
            FormValue::String(String::from("hello, world!")),
        );
        assert!(!validator.validate(&FormValue::Map(hm.clone())));
    }
}
