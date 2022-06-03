use std::{borrow::Borrow, collections::HashMap};

use crossterm::event::{KeyCode, KeyEvent};

use crate::{events::Event, pixel, screen::Screen};

use super::{FormError, FormField, FormOptions, FormValidationResult, FormValue};

/// Special FormField that manages multiple fields
///
/// Forms hosts a collection of fields and manages them sequencially
/// This field is inactive by default, you need to set it active once created
///
/// If the form can't handle all the fields (e.g. due to limited height),
/// a scrollbar will be provided to the user (if your form style contains a border)
///
/// Outputs `FormValue::Map<field_name, field_output>`
///
/// see example `form-simple` for basic usage
///
/// Navigation through forms is automatically handled with the following mapping:  
/// Tab: next field  
/// Shift-Tab: previous field  
/// Enter: next field / validate form  
/// PageUp: scroll up (if available)  
/// PageDown: scroll down (if available)  
pub struct Form {
    screen: Screen,
    height: u32,
    options: FormOptions,
    active: bool,
    index: usize,
    fields: Vec<(String, Box<dyn FormField>)>,
    scroll_index: usize,
    viewport: Screen,
}

impl Form {
    /// Constructs a new Form with the given width, height, style and options
    pub fn new(w: u32, h: u32, options: FormOptions) -> Self {
        Form {
            screen: Screen::new(w, h),
            height: h,
            options,
            index: 0,
            active: false,
            fields: vec![],
            scroll_index: 0,
            viewport: Screen::new_empty(w, 1),
        }
    }

    /// Scrolls the form viewport up (negative) or down (positive)
    ///
    /// The viewport is automatically clamped at its boundaries
    pub fn scroll(&mut self, amount: i32) {
        let mut max_scroll = self.screen.get_height() as i64 - self.get_height() as i64;
        if self.options.style.border.is_some() {
            max_scroll += 1;
        }
        self.scroll_index =
            (self.scroll_index as i64 + amount as i64).clamp(0, max_scroll.max(0)) as usize;
    }

    /// Scroll to a certain position
    pub fn scroll_to(&mut self, index: usize) {
        self.scroll(index as i32 - self.scroll_index as i32);
    }

    /// Adds the provided FormField into the Form
    ///
    /// The Field will be resized to match the width of the form  
    /// You may want to use [build_field](#methods.build_field) instead since it automatically creates the FormField instance.
    pub fn add_field<T: FormField + 'static>(&mut self, name: &str, mut field: T) {
        field.resize(self.get_width() - 2, field.get_height());
        self.fields.push((String::from(name), Box::new(field)));
    }

    /// Build a field and includes it into the Form
    ///
    /// it's an easier approach into building a Form, see the `form-simple` example to compare it with [add_field](#methods.add_field).
    pub fn build_field<T: FormField + 'static>(&mut self, name: &str, options: FormOptions) {
        let field = T::make(self.get_width(), options);
        self.add_field(name, field)
    }

    /// Get a specific field if it exists within the Form
    pub fn get_field(&self, name: &str) -> Option<&dyn FormField> {
        self.fields
            .iter()
            .find(|(field_name, _)| field_name == name)
            .map(|(_, field)| field.borrow())
    }

    /// Get the output of a specific field if it exists and valid within the Form
    ///
    /// in case of validation failing, Err value will bear the validation messages directly
    ///
    /// See example `form-validation`
    pub fn get_validated_field_output(&self, name: &str) -> Result<FormValue, FormError> {
        if let Some(field) = self.get_field(name) {
            let mut errors = FormValidationResult::new();
            field.validate(&mut errors);
            if errors.is_empty() {
                Ok(field.get_output())
            } else {
                Err(FormError::ValidationFailed(errors))
            }
        } else {
            Err(FormError::FieldNotFound)
        }
    }

    /// Get the (unvalidated) output of a specific field if it exists within the Form
    pub fn get_field_output(&self, name: &str) -> Option<FormValue> {
        self.get_field(name).map(|field| field.get_output())
    }

    /// Validate a specific field.
    pub fn validate_field(&self, name: &str) -> Option<FormValidationResult> {
        self.get_field(name).map(|field| {
            let mut errors = FormValidationResult::new();
            field.validate(&mut errors);
            errors
        })
    }

    /// Change focus on the currently active field
    fn update_active_field(&mut self) {
        let mut height = 0;
        let mut active_min_height = 0;
        for (id, (_, field)) in self.fields.iter_mut().enumerate() {
            let active = self.active && id == self.index;
            field.set_active(active);
            if active {
                active_min_height = height;
            }
            height += field.get_height();
            if field.should_display_label() {
                height += 1;
            }
        }
        // detect when the active field is outside of the scroll
        if self.scroll_index < active_min_height as usize
            || self.scroll_index + self.get_height() as usize > active_min_height as usize
        {
            self.scroll_to(active_min_height as usize);
        }
    }

    /// Checks whenever the user went through the entire form, and confirmed on the last field
    pub fn is_finished(&self) -> bool {
        self.index >= self.fields.len()
    }

    /// Checks if the form is entirely valid. If any field fails its `validate` method, the function returns `false`
    ///
    /// To retrieve errors from a specific field, use [validate_field](#methods.validate_field)  
    /// To retrieve all errors regardless of the field, use [validate](#methods.validate)
    pub fn is_valid(&mut self) -> bool {
        let mut errors = FormValidationResult::new();
        self.validate(&mut errors);

        errors.is_empty()
    }
}

impl FormField for Form {
    fn make(w: u32, options: FormOptions) -> Self {
        Self::new(w, 3, options)
    }

    fn reset(&mut self) {
        for (_, field) in self.fields.iter_mut() {
            field.reset();
        }
        self.index = 0;
        self.scroll_index = 0;
        self.update_active_field();
    }

    fn should_display_label(&self) -> bool {
        false
    }

    fn get_width(&self) -> u32 {
        self.screen.get_width()
    }

    fn get_height(&self) -> u32 {
        self.height
    }
    fn get_min_height(&self) -> u32 {
        3
    }

    fn resize(&mut self, w: u32, h: u32) {
        self.height = h;
        self.screen.resize(w, self.screen.get_height());
        for (_, field) in self.fields.iter_mut() {
            field.resize(w, field.get_height());
        }
    }

    fn handle_event(&mut self, event: Event) {
        if !self.active {
            return;
        }
        if let Event::Key(KeyEvent { code, modifiers: _ }) = event {
            match code {
                KeyCode::Enter => {
                    self.index = (self.index + 1).clamp(0, self.fields.len());
                    self.update_active_field();
                }
                KeyCode::Tab => {
                    self.index = (self.index + 1).clamp(0, self.fields.len() - 1);
                    self.update_active_field();
                }
                KeyCode::BackTab => {
                    self.index =
                        (self.index as i64 - 1).clamp(0, self.fields.len() as i64) as usize;
                    self.update_active_field();
                }
                KeyCode::PageDown => {
                    self.scroll(1);
                }
                KeyCode::PageUp => {
                    self.scroll(-1);
                }
                _ => {
                    for (_, field) in self.fields.iter_mut() {
                        if field.is_active() {
                            field.handle_event(event);
                            break;
                        }
                    }
                }
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
        self.update_active_field();
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn validate(&self, validation_result: &mut FormValidationResult) {
        for (_, field) in self.fields.iter() {
            field.validate(validation_result);
        }
        self.self_validate(validation_result);
    }

    fn get_output(&self) -> FormValue {
        let mut output: HashMap<String, FormValue> = HashMap::new();
        for (name, field) in self.fields.iter() {
            output.insert(name.to_string(), field.get_output());
        }
        FormValue::Map(output)
    }

    fn set_options(&mut self, options: FormOptions) {
        self.options = options;
    }

    fn get_options(&self) -> &FormOptions {
        &self.options
    }

    fn draw(&mut self, tick: usize) -> &Screen {
        // calculate total height of the form
        let mut total_height = 1;
        for (_, field) in self.fields.iter_mut() {
            total_height += field.get_height();
            if field.should_display_label() {
                total_height += 1;
            }
        }
        // resize the form screen if if doesn't fit anymore
        if self.screen.get_height() != total_height {
            self.screen.resize(self.screen.get_width(), total_height);
        }
        self.screen.fill(pixel::pxl_fbg(
            ' ',
            self.options.style.fg,
            self.options.style.bg,
        ));
        let padding = self.options.style.border.is_some() as i32;

        let mut current_pos = padding;
        // display form label inside the form if there is no border
        if self.options.style.border.is_none() {
            if let Some(label) = self.options.label {
                self.screen
                    .print_fbg(1, 0, label, self.options.style.fg, self.options.style.bg);
                current_pos = 1;
            }
        }
        // display fields
        for (_, field) in self.fields.iter_mut() {
            if field.should_display_label() {
                if let Some(label) = field.get_options().label {
                    self.screen.print_fbg(
                        padding,
                        current_pos,
                        label,
                        self.options.style.fg,
                        self.options.style.bg,
                    );
                    current_pos += 1;
                }
            }

            self.screen
                .print_screen(padding, current_pos, field.draw(tick));
            current_pos += field.get_height() as i32;
        }
        // Extract the form into a viewport of the real size of the form
        self.viewport = self.screen.extract(
            0,
            self.scroll_index as i32,
            self.get_width() as i32 - 1,
            self.scroll_index as i32 + self.get_height() as i32 - 1,
            pixel::pxl(' '),
        );
        if let Some(border) = self.options.style.border {
            // Display the border
            self.viewport.rect_border(
                0,
                0,
                self.get_width() as i32 - 1,
                self.get_height() as i32 - 1,
                border,
            );
            // Display the form label on the border
            if let Some(label) = self.options.label {
                self.viewport
                    .print_fbg(1, 0, label, self.options.style.fg, self.options.style.bg);
            }
            // Display a scrollbar if the form can't fit inside the viewport
            if total_height > self.get_height() - 1 {
                let mut max_scroll = total_height as i64 - self.get_height() as i64;
                if self.options.style.border.is_some() {
                    max_scroll += 1;
                }
                self.viewport.v_line(
                    self.get_width() as i32 - 1,
                    1,
                    self.get_height() as i32 - 2,
                    pixel::pxl_fbg('|', self.options.style.fg, self.options.style.bg),
                );
                self.viewport.set_pxl(
                    self.get_width() as i32 - 1,
                    1,
                    pixel::pxl_fbg('↑', self.options.style.fg, self.options.style.bg),
                );
                self.viewport.set_pxl(
                    self.get_width() as i32 - 1,
                    self.get_height() as i32 - 2,
                    pixel::pxl_fbg('↓', self.options.style.fg, self.options.style.bg),
                );
                self.viewport.set_pxl(
                    self.get_width() as i32 - 1,
                    2 + ((self.scroll_index as f32 / max_scroll as f32)
                        * (self.get_height() as f32 - 5f32)) as i32,
                    pixel::pxl_fbg('█', self.options.style.fg, self.options.style.bg),
                );
            }
        }
        &self.viewport
    }
}
