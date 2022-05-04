use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent};

use crate::{events::Event, forms::ToAny, screen::Screen};

use super::{ConsoleForm, FormOptions, FormOutput, FormStyle, FormValidationResult};

pub struct Form {
    screen: Screen,
    style: FormStyle,
    options: FormOptions,
    active: bool,
    index: usize,
    dirty: bool,
    fields: Vec<(String, Box<dyn ConsoleForm>)>,
    errors: HashMap<String, FormValidationResult>,
}

impl Form {
    pub fn new(w: u32, h: u32, style: FormStyle, options: Option<FormOptions>) -> Self {
        Form {
            screen: Screen::new(w, h),
            style,
            options: options.unwrap_or_default(),
            index: 0,
            active: false,
            dirty: true,
            fields: vec![],
            errors: HashMap::new(),
        }
    }

    pub fn add_field<T: ConsoleForm + 'static>(&mut self, name: &str, mut field: T) {
        field.resize(self.get_width() - 2, field.get_height());
        self.fields.push((String::from(name), Box::new(field)));
    }

    pub fn build_field<T: ConsoleForm + 'static>(
        &mut self,
        name: &str,
        options: Option<FormOptions>,
        style: Option<FormStyle>,
    ) {
        let mut field = T::make(self.get_width(), 1, options, style);
        if style.is_none() {
            field.set_style(self.style)
        }
        self.add_field(name, field)
    }

    pub fn get_field<T>(&mut self, name: &str) -> Option<Box<T>>
    where
        T: ConsoleForm + 'static,
    {
        for (id, (field_name, _)) in self.fields.iter().enumerate() {
            if name == *field_name {
                let (_, field) = self.fields.remove(id);
                return field.to_any().downcast::<T>().ok();
            }
        }
        None
    }

    pub fn get_result(&self, name: &str) -> Option<FormOutput> {
        for (field_name, field) in self.fields.iter() {
            if name == *field_name {
                return Some(field.get_output());
            }
        }
        None
    }

    pub fn get_error(&self, name: &str) -> Option<&FormValidationResult> {
        for (field_name, errors) in self.errors.iter() {
            if name == *field_name {
                return Some(errors);
            }
        }
        None
    }

    pub fn update_active_field(&mut self) {
        for (id, (_, field)) in self.fields.iter_mut().enumerate() {
            field.set_active(self.active && id == self.index);
        }
    }

    pub fn is_finished(&self) -> bool {
        self.index >= self.fields.len()
    }

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

impl ConsoleForm for Form {
    fn make(w: u32, h: u32, options: Option<FormOptions>, style: Option<FormStyle>) -> Self {
        Self::new(w, h, style.unwrap_or_default(), options)
    }

    fn get_width(&self) -> u32 {
        self.screen.get_width()
    }

    fn get_height(&self) -> u32 {
        self.screen.get_height()
    }

    fn resize(&mut self, w: u32, h: u32) {
        self.screen.resize(w, h);
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

    fn get_output(&self) -> FormOutput {
        let mut output: HashMap<String, FormOutput> = HashMap::new();
        for (name, field) in self.fields.iter() {
            output.insert(name.to_string(), field.get_output());
        }
        FormOutput::Compound(output)
    }

    fn set_style(&mut self, style: FormStyle) {
        self.dirty = true;
        self.style = style;
        let size = if style.border.is_some() {
            self.get_width() - 2
        } else {
            self.get_width()
        };
        for (_, field) in self.fields.iter_mut() {
            if field.get_width() != size {
                field.resize(size, field.get_height());
            }
        }
    }

    fn get_style(&self) -> &FormStyle {
        &self.style
    }

    fn set_options(&mut self, options: FormOptions) {
        self.dirty = true;
        self.options = options;
    }

    fn get_options(&self) -> &FormOptions {
        &self.options
    }

    fn draw(&mut self, tick: usize) -> &Screen {
        if self.dirty {
            let mut padding = 0;
            if let Some(border) = self.style.border {
                self.screen.rect_border(
                    0,
                    0,
                    self.get_width() as i32 - 1,
                    self.get_height() as i32 - 1,
                    border,
                );
                padding = 1;
            }

            let mut current_pos = padding;
            if let Some(label) = self.options.label {
                self.screen.print(1, 0, label);
                current_pos = 1;
            }
            for (_, field) in self.fields.iter_mut() {
                if let Some(label) = field.get_options().label {
                    self.screen.print(padding, current_pos, label);
                    current_pos += 1;
                }
                self.screen
                    .print_screen(padding, current_pos, field.draw(tick));
                current_pos += field.get_height() as i32;
            }
        }
        &self.screen
    }
}
