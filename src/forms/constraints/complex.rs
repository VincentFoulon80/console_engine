use crate::forms::FormValue;

use super::FormConstraint;

/// Calls a custom function on the FormOutput in order to validate the data
pub struct Callback {
    callback: &'static dyn Fn(&FormValue) -> bool,
    message: String,
}

impl Callback {
    pub fn new(message: &str, callback: &'static dyn Fn(&FormValue) -> bool) -> Box<Self> {
        Box::new(Self {
            callback,
            message: String::from(message),
        })
    }
}

impl FormConstraint for Callback {
    fn validate(&self, output: &FormValue) -> bool {
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
    fn validate(&self, output: &FormValue) -> bool {
        match output {
            FormValue::Nothing => true,
            FormValue::Boolean(_) => true,
            FormValue::Index(_) => true,
            FormValue::String(value) => value.chars().all(|x| (self.callback)(x)),
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

#[cfg(test)]
mod test {
    use crate::forms::constraints::FormConstraint;
    use crate::forms::FormValue;
    use std::collections::HashMap;

    #[test]
    fn callback() {
        use super::Callback;

        let validator = Callback::new("invalid!", &|x| matches!(x, FormValue::String(_)));

        assert!(!validator.validate(&FormValue::Nothing));
        assert!(validator.validate(&FormValue::String(String::from("hello, world!"))));

        let mut hm: HashMap<String, FormValue> = HashMap::new();
        hm.insert(String::from("1"), FormValue::Nothing);
        assert!(!validator.validate(&FormValue::Map(hm)));
    }

    #[test]
    fn characters_callback() {
        use super::CharactersCallback;

        let validator = CharactersCallback::new("Not alphabetic", &|x| x.is_alphabetic());

        assert!(validator.validate(&FormValue::Nothing));
        assert!(validator.validate(&FormValue::String(String::from("Helloworld"))));
        assert!(!validator.validate(&FormValue::String(String::from("123"))));
        assert!(!validator.validate(&FormValue::String(String::from("Hello123"))));
        assert!(!validator.validate(&FormValue::String(String::from("hello, world!"))));

        let mut hm: HashMap<String, FormValue> = HashMap::new();
        hm.insert(String::from("1"), FormValue::Nothing);
        assert!(validator.validate(&FormValue::Map(hm.clone())));
        hm.insert(
            String::from("2"),
            FormValue::String(String::from("Helloworld")),
        );
        assert!(validator.validate(&FormValue::Map(hm.clone())));
        hm.insert(String::from("3"), FormValue::String(String::from("123")));
        assert!(!validator.validate(&FormValue::Map(hm.clone())));
    }
}
