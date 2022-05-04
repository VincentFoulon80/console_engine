mod basic;
mod numbers;

pub use basic::NotBlank;
pub use numbers::Number;

use super::FormOutput;
pub trait FormConstraint {
    fn validate(&self, output: &FormOutput) -> bool;

    fn get_message(&self) -> &str;
}
