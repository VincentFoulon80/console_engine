use crate::forms::FormOutput;

use super::FormConstraint;

/// Calls a custom function on the FormOutput in order to validate the data
pub struct Callback {
    callback: &'static dyn Fn(&FormOutput) -> bool,
    message: String,
}

impl Callback {
    pub fn new(message: &str, callback: &'static dyn Fn(&FormOutput) -> bool) -> Box<Self> {
        Box::new(Self {
            callback,
            message: String::from(message),
        })
    }
}

impl FormConstraint for Callback {
    fn validate(&self, output: &FormOutput) -> bool {
        (self.callback)(output)
    }

    fn get_message(&self) -> &str {
        &self.message
    }
}

/// On Strings output, call a custom function for every character in order to validate the data
/// Recursively parse Compound output
pub struct CharactersCallback {
    callback: &'static dyn Fn(char) -> bool,
    message: String,
}

impl CharactersCallback {
    pub fn new(message: &str, callback: &'static dyn Fn(char) -> bool) -> Box<Self> {
        Box::new(Self {
            callback,
            message: String::from(message),
        })
    }
}

impl FormConstraint for CharactersCallback {
    fn validate(&self, output: &FormOutput) -> bool {
        match output {
            FormOutput::Nothing => true,
            FormOutput::String(value) => value.chars().all(|x| (self.callback)(x)),
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
    fn callback() {
        use super::Callback;

        let validator = Callback::new("invalid!", &|x| matches!(x, FormOutput::String(_)));

        assert!(!validator.validate(&FormOutput::Nothing));
        assert!(validator.validate(&FormOutput::String(String::from("hello, world!"))));

        let mut hm: HashMap<String, FormOutput> = HashMap::new();
        hm.insert(String::from("1"), FormOutput::Nothing);
        assert!(!validator.validate(&FormOutput::Compound(hm)));
    }

    #[test]
    fn characters_callback() {
        use super::CharactersCallback;

        let validator = CharactersCallback::new("Not alphabetic", &|x| x.is_alphabetic());

        assert!(validator.validate(&FormOutput::Nothing));
        assert!(validator.validate(&FormOutput::String(String::from("Helloworld"))));
        assert!(!validator.validate(&FormOutput::String(String::from("123"))));
        assert!(!validator.validate(&FormOutput::String(String::from("Hello123"))));
        assert!(!validator.validate(&FormOutput::String(String::from("hello, world!"))));

        let mut hm: HashMap<String, FormOutput> = HashMap::new();
        hm.insert(String::from("1"), FormOutput::Nothing);
        assert!(validator.validate(&FormOutput::Compound(hm.clone())));
        hm.insert(
            String::from("2"),
            FormOutput::String(String::from("Helloworld")),
        );
        assert!(validator.validate(&FormOutput::Compound(hm.clone())));
        hm.insert(String::from("3"), FormOutput::String(String::from("123")));
        assert!(!validator.validate(&FormOutput::Compound(hm.clone())));
    }
}
