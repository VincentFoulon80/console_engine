use std::{any::Any, collections::HashMap};

use crate::{events::Event, rect_style::BorderStyle, screen::Screen};

pub mod constraints;
mod form;
mod input;

use crossterm::style::Color;
pub use form::Form;
pub use input::TextInput;

use self::constraints::FormConstraint;

pub trait ToAny {
    fn to_any(self) -> Box<dyn Any>;
}
impl<T: 'static> ToAny for T {
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}

pub trait ConsoleForm: ToAny {
    fn make(w: u32, h: u32, options: Option<FormOptions>, style: Option<FormStyle>) -> Self
    where
        Self: Sized;

    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
    fn resize(&mut self, w: u32, h: u32);

    fn handle_event(&mut self, event: &Event);

    fn set_active(&mut self, active: bool);
    fn is_active(&self) -> bool;
    fn validate(&self, validation_result: &mut FormValidationResult);
    fn get_output(&self) -> FormOutput;

    fn set_style(&mut self, style: FormStyle);
    fn get_style(&self) -> &FormStyle;
    fn set_options(&mut self, options: FormOptions);
    fn get_options(&self) -> &FormOptions;

    fn draw(&mut self, tick: usize) -> &Screen;

    fn self_validate(&self, validation_result: &mut FormValidationResult) {
        let output = self.get_output();
        for constraint in self.get_options().constraints.iter() {
            if !constraint.validate(&output) {
                validation_result.push(String::from(constraint.get_message()))
            }
        }
    }
}

type FormValidationResult = Vec<String>;

#[derive(Debug, Clone)]
pub enum FormOutput {
    Nothing,
    String(String),
    Compound(HashMap<String, FormOutput>),
}

impl Default for FormOutput {
    fn default() -> Self {
        Self::Nothing
    }
}

#[derive(Clone, Copy)]
pub struct FormStyle {
    pub border: Option<BorderStyle>,
    pub fg: Color,
    pub bg: Color,
}

impl Default for FormStyle {
    fn default() -> Self {
        Self {
            border: None,
            fg: Color::Grey,
            bg: Color::Black,
        }
    }
}

#[derive(Default)]
pub struct FormOptions {
    pub label: Option<&'static str>,
    pub constraints: Vec<Box<dyn FormConstraint>>,
}
