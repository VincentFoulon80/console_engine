//! Form Validation Constraints
mod basic;
mod complex;
mod logic;
mod numbers;
mod string;

pub use basic::IsTrue;
pub use basic::NotBlank;
pub use complex::Callback;
pub use complex::CharactersCallback;
pub use logic::AllOf;
pub use logic::AnyOf;
pub use logic::Not;
pub use numbers::Integer;
pub use numbers::Number;
pub use string::Alphabetic;
pub use string::Alphanumeric;

use super::FormValue;

/// Trait that define validation constraints
///
/// see example `form-validation` for basic usage
pub trait FormConstraint {
    /// Validates a given input
    ///
    /// This function must return true if the given Constraint is validated
    fn validate(&self, output: &FormValue) -> bool;
    /// Returns what message this Constraint should display in case of non-validated input
    fn get_message(&self) -> &str;
}
