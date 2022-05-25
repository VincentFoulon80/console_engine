//! Form Validation Constraints
mod basic;
mod complex;
mod numbers;
mod string;

pub use basic::NotBlank;
pub use complex::Callback;
pub use complex::CharactersCallback;
pub use numbers::Integer;
pub use numbers::Number;
pub use string::Alphabetic;
pub use string::Alphanumeric;

use super::FormValue;

/// Trait that define validation constraints
pub trait FormConstraint {
    fn validate(&self, output: &FormValue) -> bool;

    fn get_message(&self) -> &str;
}
