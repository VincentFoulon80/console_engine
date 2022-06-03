use crate::forms::FormValue;

use super::FormConstraint;

/// Negates the provided constraint
pub struct Not {
    message: String,
    constraint: Box<dyn FormConstraint>,
}
impl Not {
    pub fn new(message: &str, constraint: Box<dyn FormConstraint>) -> Box<Self> {
        Box::new(Self {
            message: String::from(message),
            constraint,
        })
    }
}

impl FormConstraint for Not {
    fn validate(&self, output: &FormValue) -> bool {
        !self.constraint.validate(output)
    }

    fn get_message(&self) -> &str {
        &self.message
    }
}

/// Validates any of the provided constraint, like a logical OR
pub struct AnyOf {
    message: String,
    constraints: Vec<Box<dyn FormConstraint>>,
}

impl AnyOf {
    pub fn new(message: &str, constraints: Vec<Box<dyn FormConstraint>>) -> Box<Self> {
        Box::new(Self {
            message: String::from(message),
            constraints,
        })
    }
}

impl FormConstraint for AnyOf {
    fn validate(&self, output: &FormValue) -> bool {
        self.constraints
            .iter()
            .any(|constraint| constraint.validate(output))
    }

    fn get_message(&self) -> &str {
        &self.message
    }
}

/// Validates all of the provided constraint, like a logical AND
pub struct AllOf {
    message: String,
    constraints: Vec<Box<dyn FormConstraint>>,
}

impl AllOf {
    pub fn new(message: &str, constraints: Vec<Box<dyn FormConstraint>>) -> Box<Self> {
        Box::new(Self {
            message: String::from(message),
            constraints,
        })
    }
}

impl FormConstraint for AllOf {
    fn validate(&self, output: &FormValue) -> bool {
        self.constraints
            .iter()
            .all(|constraint| constraint.validate(output))
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
    fn not() {
        use super::super::NotBlank;
        use super::Not;

        let validator = Not::new("Not blank", NotBlank::new("blank"));

        assert!(validator.validate(&FormValue::Nothing));
        assert!(validator.validate(&FormValue::String(String::from(""))));
        assert!(!validator.validate(&FormValue::String(String::from("hello, world!"))));

        let mut hm: HashMap<String, FormValue> = HashMap::new();
        assert!(validator.validate(&FormValue::Map(hm.clone())));
        hm.insert(String::from("1"), FormValue::Nothing);
        assert!(!validator.validate(&FormValue::Map(hm.clone())));
    }

    #[test]
    fn any_of() {
        use super::super::{Integer, NotBlank};
        use super::{AnyOf, Not};

        // the outputs needs to be either blank, or an integer
        let validator = AnyOf::new(
            "should be blank or a valid integer",
            vec![
                Not::new("Not blank", NotBlank::new("blank")),
                Integer::new("not integer"),
            ],
        );

        assert!(validator.validate(&FormValue::Nothing));
        assert!(validator.validate(&FormValue::String(String::from(""))));
        assert!(!validator.validate(&FormValue::String(String::from("hello, world!"))));
        assert!(validator.validate(&FormValue::String(String::from("734"))));

        let mut hm: HashMap<String, FormValue> = HashMap::new();
        assert!(validator.validate(&FormValue::Map(hm.clone())));
        hm.insert(String::from("1"), FormValue::Nothing);
        assert!(validator.validate(&FormValue::Map(hm.clone())));
    }

    #[test]
    fn all_of() {
        use super::super::{Alphanumeric, Integer};
        use super::AllOf;

        // the outputs needs to be an alphanumeric string that correspond to an integer
        let validator = AllOf::new(
            "should be alphanumeric and a valid integer",
            vec![
                Alphanumeric::new("not alphanumeric"),
                Integer::new("not integer"),
            ],
        );

        assert!(validator.validate(&FormValue::Nothing));
        assert!(!validator.validate(&FormValue::String(String::from(""))));
        assert!(!validator.validate(&FormValue::String(String::from("hello, world!"))));
        assert!(validator.validate(&FormValue::String(String::from("734"))));
        assert!(!validator.validate(&FormValue::String(String::from("!734"))));
    }
}
