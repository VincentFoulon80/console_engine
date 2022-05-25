use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent};

use crate::{events::Event, forms::ToAny, pixel, screen::Screen};

use super::{FormField, FormOptions, FormValidationResult, FormValue};

/// Special FormField that manages multiple fields
///
/// Forms hosts a collection of fields and manages them sequencially
/// This field is inactive by default, you need to set it active once created
///
/// If the form can't handle all the fields (e.g. due to limited height),
/// a scrollbar will be provided to the user (if your form contains a border)
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
    dirty: bool,
    fields: Vec<(String, Box<dyn FormField>)>,
    errors: HashMap<String, FormValidationResult>,
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
            dirty: true,
            fields: vec![],
            errors: HashMap::new(),
            scroll_index: 0,
            viewport: Screen::new_empty(w, 1),
        }
    }

    pub fn scroll(&mut self, amount: i32) {
        let mut max_scroll = self.screen.get_height() as i64 - self.get_height() as i64;
        if self.options.style.border.is_some() {
            max_scroll += 1;
        }
        self.scroll_index =
            (self.scroll_index as i64 + amount as i64).clamp(0, max_scroll.max(0)) as usize;
    }

    pub fn scroll_to(&mut self, index: usize) {
        self.scroll(index as i32 - self.scroll_index as i32);
    }

    /// Adds the provided FormField into the Form
    /// The Field will be resized to match the width of the form
    /// You may want to use [build_field](#methods.build_field) instead since it automatically creates the FormField instance.
    pub fn add_field<T: FormField + 'static>(&mut self, name: &str, mut field: T) {
        field.resize(self.get_width() - 2, field.get_height());
        self.fields.push((String::from(name), Box::new(field)));
    }

    /// Build a field and includes it into the Form
    /// it's an easier approach into building a Form, see the `form-simple` example to compare it with [add_field](#methods.add_field).
    pub fn build_field<T: FormField + 'static>(&mut self, name: &str, options: FormOptions) {
        let field = T::make(self.get_width(), options);
        self.add_field(name, field)
    }

    /// Get a specific field if it exists within the Form
    /// Note that the field will be removed from the Form
    pub fn get_field<T>(&mut self, name: &str) -> Option<Box<T>>
    where
        T: FormField + 'static,
    {
        for (id, (field_name, _)) in self.fields.iter().enumerate() {
            if name == *field_name {
                let (_, field) = self.fields.remove(id);
                return field.to_any().downcast::<T>().ok();
            }
        }
        None
    }

    /// Get the output of a specific field if it exists within the Form
    pub fn get_result(&self, name: &str) -> Option<FormValue> {
        for (field_name, field) in self.fields.iter() {
            if name == *field_name {
                return Some(field.get_output());
            }
        }
        None
    }

    /// Get the errors generated from a specific field.
    /// You must run [is_valid](#methods.is_valid) first in order to be able to retrieve the errors
    pub fn get_error(&self, name: &str) -> Option<&FormValidationResult> {
        for (field_name, errors) in self.errors.iter() {
            if name == *field_name {
                return Some(errors);
            }
        }
        None
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
            if field.display_label() {
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
    /// and a list of every reported error is stored in the form, waiting for [get_error](#methods.get_error) to be called
    pub fn is_valid(&mut self) -> bool {
        self.errors.clear();
        for (name, field) in self.fields.iter() {
            let mut field_errors: FormValidationResult = vec![];
            field.validate(&mut field_errors);
            if !field_errors.is_empty() {
                self.errors.insert(String::from(name), field_errors);
            }
        }
        let mut self_errors: FormValidationResult = vec![];
        self.validate(&mut self_errors);

        self.errors.is_empty() && self_errors.is_empty()
    }
}

impl FormField for Form {
    fn make(w: u32, options: FormOptions) -> Self {
        Self::new(w, 3, options)
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

    fn handle_event(&mut self, event: &Event) {
        for (_, field) in self.fields.iter_mut() {
            field.handle_event(event);
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
                _ => {}
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
        self.dirty = true;
        self.options = options;
    }

    fn get_options(&self) -> &FormOptions {
        &self.options
    }

    fn draw(&mut self, tick: usize) -> &Screen {
        let mut total_height = 1;
        for (_, field) in self.fields.iter_mut() {
            total_height += field.get_height();
            if field.display_label() {
                total_height += 1;
            }
        }
        if self.screen.get_height() != total_height {
            self.screen.resize(self.screen.get_width(), total_height);
        }
        if self.dirty {
            let padding = if self.options.style.border.is_some() {
                1
            } else {
                0
            };

            let mut current_pos = padding;
            if let Some(label) = self.options.label {
                self.screen.print(1, 0, label);
                current_pos = 1;
            }
            for (_, field) in self.fields.iter_mut() {
                if field.display_label() {
                    if let Some(label) = field.get_options().label {
                        self.screen.print(padding, current_pos, label);
                        current_pos += 1;
                    }
                }

                self.screen
                    .print_screen(padding, current_pos, field.draw(tick));
                current_pos += field.get_height() as i32;
            }
        }
        self.viewport = self.screen.extract(
            0,
            self.scroll_index as i32,
            self.get_width() as i32 - 1,
            self.scroll_index as i32 + self.get_height() as i32 - 1,
            pixel::pxl(' '),
        );
        if let Some(border) = self.options.style.border {
            self.viewport.rect_border(
                0,
                0,
                self.get_width() as i32 - 1,
                self.get_height() as i32 - 1,
                border,
            );
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
