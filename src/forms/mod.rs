//! Forms within your terminal
use std::{any::Any, collections::HashMap};

use crate::{events::Event, rect_style::BorderStyle, screen::Screen};

mod choices;
pub mod constraints;
mod form;
mod text;

pub use choices::Checkbox;
pub use choices::Radio;
use crossterm::style::Color;
pub use form::Form;
pub use text::HiddenText;
pub use text::Text;

use self::constraints::FormConstraint;

/// Helper trait to allow downcasting FormFields
pub trait ToAny {
    fn to_any(self) -> Box<dyn Any>;
}
impl<T: 'static> ToAny for T {
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}

/// Necessary functions to build a Form Field
pub trait FormField: ToAny {
    /// Base function to allow building the Field programmatically (e.g. in Forms)
    ///
    /// This function is separate to `new` to allow Fields to define custom constructors
    fn make(w: u32, options: FormOptions) -> Self
    where
        Self: Sized;

    /// Reset the field
    fn reset(&mut self);

    /// Get the width of the field
    fn get_width(&self) -> u32;
    /// Get the height of the field
    fn get_height(&self) -> u32;
    fn get_min_height(&self) -> u32 {
        1
    }
    /// Resize (if possible) the field
    fn resize(&mut self, w: u32, h: u32);

    /// This function is the heart of FormFields : it allows the form to handle itself by passing a ConsoleEngine Event to it.
    fn handle_event(&mut self, event: &Event);

    /// Set the active state of a field (if applicable)
    fn set_active(&mut self, active: bool);
    /// Checks if the state of a field is active
    fn is_active(&self) -> bool;

    /// Allow the field to validate its content by itself.
    ///
    /// Make sure to call [self_validate](#method.self_validate) in it,
    /// so the Validation Constraints provided by FormOptions will be automatically checked against the output of the field
    fn validate(&self, validation_result: &mut FormValidationResult);
    /// Get the output of the field
    fn get_output(&self) -> FormValue;

    /// Sets the options of the Field
    fn set_options(&mut self, options: FormOptions);
    /// Gets the options of the Field
    fn get_options(&self) -> &FormOptions;
    /// Tell if we should display the label externally (some fields may want to display it themselves like buttons)
    fn should_display_label(&self) -> bool {
        self.get_options().label.is_some()
    }

    /// This is the other heart of FormFields : it asks the field to build itself as a `Screen`.
    /// It's the function that allow the field to be shown on screen.
    ///
    /// You can provide a tick parameter in order for some fields to do some animations.
    /// (e.g. blinking the selected element, or a text cursor, ...)
    fn draw(&mut self, tick: usize) -> &Screen;

    /// Validation function that only runs the constraints contained in the field options
    ///
    /// This function should not be used outside of a FormField impl! Use [validate](#method.validate) instead
    fn self_validate(&self, validation_result: &mut FormValidationResult) {
        let output = self.get_output();
        for constraint in self.get_options().constraints.iter() {
            if !constraint.validate(&output) {
                validation_result.push(String::from(constraint.get_message()))
            }
        }
    }
}

/// List of error messages encountered when validating a field
pub type FormValidationResult = Vec<String>;

/// Type that stores a potential output coming from a Form Field
#[derive(Debug, Clone)]
pub enum FormValue {
    Nothing,
    Boolean(bool),
    Index(usize),
    String(String),
    List(Vec<String>),
    Vec(Vec<FormValue>),
    Map(HashMap<String, FormValue>),
}

impl Default for FormValue {
    fn default() -> Self {
        Self::Nothing
    }
}

/// Structure that stores style information for Form Fields
#[derive(Clone, Copy)]
pub struct FormStyle {
    /// Border style if a field need to build a border or use some of the stored character
    pub border: Option<BorderStyle>,
    /// Foreground Color
    /// Note that this color will be reversed with the background for selected fields
    /// Thus, we can't use Color::Reset reliably
    pub fg: Color,
    /// Background Color
    /// Note that this color will be reversed with the foreground for selected fields
    /// Thus, we can't use Color::Reset reliably
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

/// Stores a bunch of options for Form Fields
#[derive(Default)]
pub struct FormOptions {
    pub style: FormStyle,
    /// Label of the field, if used in forms
    pub label: Option<&'static str>,
    /// List of Validation constraints used for validating the content of a Form Field
    pub constraints: Vec<Box<dyn FormConstraint>>,
    /// Additional values that Fields may need in order to initialize themselves
    pub custom: HashMap<String, FormValue>,
}
