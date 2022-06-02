use crossterm::event::{KeyCode, KeyEvent};

use crate::{events::Event, pixel, screen::Screen};

use super::{FormField, FormOptions, FormValidationResult, FormValue};

/// Radio choices
///
/// This form field display a list of elements, in which our user must chose one element
/// This field is inactive by default, you need to set it active once created
///
/// Outputs `FormValue::Index` or `FormValue::Nothing` if the list is empty
///
/// see example `form-choices` for basic usage
pub struct Radio {
    screen: Screen,
    list: Vec<String>,
    dirty: bool,
    active: bool,
    selected: usize,
    cursor_pos: usize,
    options: FormOptions,
}

impl Radio {
    pub fn new(w: u32, options: FormOptions) -> Self {
        let list = if let Some(FormValue::List(list)) = options.custom.get("choices").cloned() {
            list
        } else {
            vec![]
        };
        Radio {
            screen: Screen::new(w, list.len() as u32),
            list,
            dirty: true,
            active: false,
            selected: 0,
            cursor_pos: 0,
            options,
        }
    }

    /// Moves the cursor up (negative) or down (positive)
    ///
    /// The cursor is clamped at its boundaries
    pub fn move_cursor(&mut self, amount: i32) {
        if !self.list.is_empty() {
            self.dirty = true;
            self.cursor_pos = (self.cursor_pos as i64 + amount as i64)
                .clamp(0, self.list.len() as i64 - 1) as usize;
        }
    }

    /// Retrieve the stored list of choices
    pub fn get_list(&self) -> &Vec<String> {
        &self.list
    }
    /// Sets a list of choices
    pub fn set_list(&mut self, list: Vec<String>) {
        self.list = list;
        self.update_list();
    }

    fn update_list(&mut self) {
        self.screen.resize(self.get_width(), self.list.len() as u32)
    }
}

impl FormField for Radio {
    fn make(w: u32, options: FormOptions) -> Self
    where
        Self: Sized,
    {
        Self::new(w, options)
    }

    fn reset(&mut self) {
        self.selected = 0;
    }

    fn get_width(&self) -> u32 {
        self.screen.get_width()
    }

    fn get_height(&self) -> u32 {
        self.screen.get_height()
    }

    fn resize(&mut self, w: u32, _: u32) {
        self.screen.resize(w, self.get_height());
        self.dirty = true;
    }

    fn handle_event(&mut self, event: Event) {
        if !self.active {
            return;
        }
        if let Event::Key(KeyEvent { code, modifiers: _ }) = event {
            match code {
                KeyCode::Up => self.move_cursor(-1),
                KeyCode::Down => self.move_cursor(1),
                KeyCode::Char(' ') => {
                    self.selected = self.cursor_pos;
                    self.dirty = true
                }
                _ => {}
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.dirty = true;
        self.active = active;
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn validate(&self, validation_result: &mut FormValidationResult) {
        self.self_validate(validation_result)
    }

    fn get_output(&self) -> FormValue {
        if self.list.is_empty() {
            FormValue::Nothing
        } else {
            FormValue::Index(self.selected)
        }
    }

    fn set_options(&mut self, options: FormOptions) {
        self.options = options;
    }

    fn get_options(&self) -> &FormOptions {
        &self.options
    }

    fn draw(&mut self, _tick: usize) -> &Screen {
        if self.dirty {
            self.dirty = false;
            self.screen.fill(pixel::pxl_fbg(
                ' ',
                self.options.style.fg,
                self.options.style.bg,
            ));
            for (id, entry) in self.list.iter().enumerate() {
                let (fg, bg) = if self.active && self.cursor_pos == id {
                    (self.options.style.bg, self.options.style.fg)
                } else {
                    (self.options.style.fg, self.options.style.bg)
                };
                self.screen.print_fbg(
                    0,
                    id as i32,
                    &format!(
                        "({}) {}",
                        if self.selected == id { 'x' } else { ' ' },
                        entry
                    ),
                    fg,
                    bg,
                )
            }
        }
        &self.screen
    }
}

/// Checkbox choices
///
/// This form field display a list of elements, in which our user can chose one or more element
/// This field is inactive by default, you need to set it active once created
///
/// Outputs `FormValue::Vec<FormValue::Index>`
///
/// see example `form-choices` for basic usage
pub struct Checkbox {
    screen: Screen,
    list: Vec<String>,
    dirty: bool,
    active: bool,
    selected: Vec<usize>,
    cursor_pos: usize,
    options: FormOptions,
}

impl Checkbox {
    pub fn new(w: u32, options: FormOptions) -> Self {
        let list = if let Some(FormValue::List(list)) = options.custom.get("choices").cloned() {
            list
        } else {
            vec![]
        };
        Checkbox {
            screen: Screen::new(w, list.len() as u32),
            list,
            dirty: true,
            active: false,
            selected: vec![],
            cursor_pos: 0,
            options,
        }
    }

    /// Moves the cursor up (negative) or down (positive)
    ///
    /// The cursor is clamped at its boundaries
    pub fn move_cursor(&mut self, amount: i32) {
        if !self.list.is_empty() {
            self.dirty = true;
            self.cursor_pos = (self.cursor_pos as i64 + amount as i64)
                .clamp(0, self.list.len() as i64 - 1) as usize;
        }
    }

    /// Retrieve the stored list of choices
    pub fn get_list(&self) -> &Vec<String> {
        &self.list
    }

    /// Sets a list of choices
    pub fn set_list(&mut self, list: Vec<String>) {
        self.list = list;
        self.update_list();
    }

    fn update_list(&mut self) {
        self.screen.resize(self.get_width(), self.list.len() as u32)
    }
}

impl FormField for Checkbox {
    fn make(w: u32, options: FormOptions) -> Self
    where
        Self: Sized,
    {
        Self::new(w, options)
    }

    fn reset(&mut self) {
        self.selected.clear();
    }

    fn get_width(&self) -> u32 {
        self.screen.get_width()
    }

    fn get_height(&self) -> u32 {
        self.screen.get_height()
    }

    fn resize(&mut self, w: u32, _: u32) {
        self.screen.resize(w, self.get_height());
        self.dirty = true;
    }

    fn handle_event(&mut self, event: Event) {
        if !self.active {
            return;
        }
        if let Event::Key(KeyEvent { code, modifiers: _ }) = event {
            match code {
                KeyCode::Up => self.move_cursor(-1),
                KeyCode::Down => self.move_cursor(1),
                KeyCode::Char(' ') => {
                    if self.selected.contains(&self.cursor_pos) {
                        self.selected.retain(|x| *x != self.cursor_pos);
                    } else {
                        self.selected.push(self.cursor_pos);
                    }
                    self.dirty = true
                }
                _ => {}
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.dirty = true;
        self.active = active;
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn validate(&self, validation_result: &mut FormValidationResult) {
        self.self_validate(validation_result)
    }

    fn get_output(&self) -> FormValue {
        let mut output_vec = vec![];
        for id in self.selected.iter() {
            output_vec.push(FormValue::Index(*id));
        }

        FormValue::Vec(output_vec)
    }

    fn set_options(&mut self, options: FormOptions) {
        self.options = options;
    }

    fn get_options(&self) -> &FormOptions {
        &self.options
    }

    fn draw(&mut self, _tick: usize) -> &Screen {
        if self.dirty {
            self.screen.fill(pixel::pxl_fbg(
                ' ',
                self.options.style.fg,
                self.options.style.bg,
            ));
            self.dirty = false;
            for (id, entry) in self.list.iter().enumerate() {
                let (fg, bg) = if self.active && self.cursor_pos == id {
                    (self.options.style.bg, self.options.style.fg)
                } else {
                    (self.options.style.fg, self.options.style.bg)
                };
                self.screen.print_fbg(
                    0,
                    id as i32,
                    &format!(
                        "[{}] {}",
                        if self.selected.contains(&id) {
                            'x'
                        } else {
                            ' '
                        },
                        entry
                    ),
                    fg,
                    bg,
                )
            }
        }
        &self.screen
    }
}
