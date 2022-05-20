use crate::forms::FormOutput;

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
    fn validate(&self, output: &FormOutput) -> bool {
        match output {
            FormOutput::Nothing => true,
            FormOutput::Boolean(_) => true,
            FormOutput::String(value) => value.chars().all(|x| x.is_alphabetic()),
            FormOutput::HashMap(fields) => fields.iter().all(|(_, x)| self.validate(x)),
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
    fn validate(&self, output: &FormOutput) -> bool {
        match output {
            FormOutput::Nothing => true,
            FormOutput::Boolean(_) => true,
            FormOutput::String(value) => value.chars().all(|x| x.is_alphanumeric()),
            FormOutput::HashMap(fields) => fields.iter().all(|(_, x)| self.validate(x)),
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
    fn alphabetic() {
        use super::Alphabetic;

        let validator = Alphabetic::new("Not alphabetic");

        assert!(validator.validate(&FormOutput::Nothing));
        assert!(validator.validate(&FormOutput::String(String::from("Helloworld"))));
        assert!(!validator.validate(&FormOutput::String(String::from("123"))));
        assert!(!validator.validate(&FormOutput::String(String::from("Hello123"))));
        assert!(!validator.validate(&FormOutput::String(String::from("hello, world!"))));

        let mut hm: HashMap<String, FormOutput> = HashMap::new();
        hm.insert(String::from("1"), FormOutput::Nothing);
        assert!(validator.validate(&FormOutput::HashMap(hm.clone())));
        hm.insert(
            String::from("2"),
            FormOutput::String(String::from("Helloworld")),
        );
        assert!(validator.validate(&FormOutput::HashMap(hm.clone())));
        hm.insert(String::from("3"), FormOutput::String(String::from("123")));
        assert!(!validator.validate(&FormOutput::HashMap(hm.clone())));
    }

    #[test]
    fn alphanumeric() {
        use super::Alphanumeric;

        let validator = Alphanumeric::new("Not alphanumeric");

        assert!(validator.validate(&FormOutput::Nothing));
        assert!(validator.validate(&FormOutput::String(String::from("Helloworld"))));
        assert!(validator.validate(&FormOutput::String(String::from("123"))));
        assert!(validator.validate(&FormOutput::String(String::from("Hello123"))));
        assert!(!validator.validate(&FormOutput::String(String::from("hello, world!"))));

        let mut hm: HashMap<String, FormOutput> = HashMap::new();
        hm.insert(String::from("1"), FormOutput::Nothing);
        assert!(validator.validate(&FormOutput::HashMap(hm.clone())));
        hm.insert(
            String::from("2"),
            FormOutput::String(String::from("Helloworld")),
        );
        assert!(validator.validate(&FormOutput::HashMap(hm.clone())));
        hm.insert(String::from("3"), FormOutput::String(String::from("123")));
        assert!(validator.validate(&FormOutput::HashMap(hm.clone())));
        hm.insert(
            String::from("3"),
            FormOutput::String(String::from("hello, world!")),
        );
        assert!(!validator.validate(&FormOutput::HashMap(hm.clone())));
    }
}
