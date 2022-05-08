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

use super::FormOutput;
pub trait FormConstraint {
    fn validate(&self, output: &FormOutput) -> bool;

    fn get_message(&self) -> &str;
}
