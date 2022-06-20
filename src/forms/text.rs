use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{events::Event, pixel, screen::Screen};

use super::{FormField, FormOptions, FormValidationResult, FormValue};

/// Generic text input
///
/// This form field generates a generic text input, that handles keyboard input (moving cursor, backspacing / deleting, home / end)
/// This field is inactive by default, you need to set it active once created
///
/// Custom options:
/// - `default`: prefilled value, even on reset
///
/// Outputs `FormValue::String`
///
/// see example `form-input` for basic usage
pub struct Text {
    screen: Screen,
    dirty: bool,
    active: bool,
    input_buffer: String,
    default_input: Option<String>,
    cursor_pos: usize,
    options: FormOptions,
}

impl Text {
    pub fn new(w: u32, options: FormOptions) -> Self {
        let default = if let Some(FormValue::String(value)) = options.custom.get("default") {
            Some(value.clone())
        } else {
            None
        };
        let mut text = Self {
            screen: Screen::new(w, 1),
            dirty: true,
            active: false,
            input_buffer: default.clone().unwrap_or_default(),
            default_input: default,
            cursor_pos: 0,
            options,
        };
        text.move_cursor(i32::MAX);
        text
    }

    /// Sets a specific value inside the field
    pub fn set_input_buffer(&mut self, input: &str) {
        self.dirty = true;
        self.input_buffer = String::from(input);
        self.move_cursor(i32::MAX);
    }

    /// Clear the field
    pub fn clear_input_buffer(&mut self) {
        self.dirty = true;
        self.input_buffer = String::new();
        self.cursor_pos = 0;
    }

    /// Insert a character at the position of the cursor
    pub fn put_char(&mut self, chr: char) {
        let mut new_buffer = String::with_capacity(self.input_buffer.capacity() + 1);
        new_buffer.extend(
            self.input_buffer
                .chars()
                .take(self.cursor_pos)
                .chain(std::iter::once(chr))
                .chain(self.input_buffer.chars().skip(self.cursor_pos)),
        );
        self.input_buffer = new_buffer;
        self.move_cursor(1);
    }

    /// Removes a certain amount of characters either on the left (positive) or right (negative) side of the cursor
    pub fn remove_char(&mut self, amount: i32) {
        if amount == 0 {
            return;
        }
        self.dirty = true;
        let off_l = amount.max(0) as usize; // offset to the left from cursor, `positive` or 0
        let off_r = amount.min(0).abs() as usize; // offset to the right from cursor,  `-negative` or 0
        let pos_l = self.cursor_pos.saturating_sub(off_l);
        let pos_r = self
            .cursor_pos
            .saturating_add(off_r)
            .min(self.input_buffer.len());
        self.input_buffer = self.input_buffer.chars().take(pos_l).collect::<String>()
            + &self.input_buffer.chars().skip(pos_r).collect::<String>(); // this skips the cursor +/- offsets
        self.move_cursor(-amount.max(0));
    }

    /// Moves the cursor left (negative) or right (positive)
    ///
    /// The cursor is clamped at its boundaries
    pub fn move_cursor(&mut self, amount: i32) {
        self.dirty = true;
        self.cursor_pos = (self.cursor_pos as i64 + amount as i64)
            .clamp(0, self.input_buffer.len() as i64) as usize;
    }
}

impl FormField for Text {
    fn make(w: u32, options: FormOptions) -> Self
    where
        Self: Sized,
    {
        Self::new(w, options)
    }

    fn reset(&mut self) {
        self.clear_input_buffer();
        if let Some(default_input) = self.default_input.clone() {
            self.set_input_buffer(&default_input);
        }
    }

    fn get_width(&self) -> u32 {
        self.screen.get_width()
    }

    fn get_height(&self) -> u32 {
        self.screen.get_height()
    }

    fn resize(&mut self, w: u32, _h: u32) {
        self.dirty = true;
        self.screen.resize(w, 1);
    }

    fn handle_event(&mut self, event: Event) {
        if !self.active {
            return;
        }
        if let Event::Key(KeyEvent { code, modifiers }) = event {
            match code {
                KeyCode::Backspace => self.remove_char(1),
                KeyCode::Delete => self.remove_char(-1),
                KeyCode::Left => self.move_cursor(-1),
                KeyCode::Right => self.move_cursor(1),
                KeyCode::Home => self.move_cursor(i32::MIN),
                KeyCode::End => self.move_cursor(i32::MAX),
                KeyCode::Char(c) => {
                    if modifiers.is_empty()
                        || modifiers == KeyModifiers::CONTROL | KeyModifiers::ALT
                    {
                        self.put_char(c);
                    }
                    if modifiers == KeyModifiers::SHIFT {
                        // I don't understand why it works this way but not the other
                        if c.is_ascii_uppercase() {
                            self.put_char(c.to_ascii_uppercase());
                        } else {
                            self.put_char(c.to_ascii_lowercase());
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn validate(&self, validation_result: &mut FormValidationResult) {
        self.self_validate(validation_result);
    }

    fn get_output(&self) -> FormValue {
        FormValue::String(self.input_buffer.to_string())
    }

    fn set_options(&mut self, options: FormOptions) {
        self.options = options;
        self.default_input =
            if let Some(FormValue::String(value)) = self.options.custom.get("default") {
                Some(value.clone())
            } else {
                None
            };
    }

    fn get_options(&self) -> &FormOptions {
        &self.options
    }

    fn draw(&mut self, tick: usize) -> &Screen {
        if self.dirty {
            self.screen.fill(pixel::pxl_fbg(
                ' ',
                self.options.style.fg,
                self.options.style.bg,
            ));
            self.screen.print_fbg(
                if self.cursor_pos >= self.screen.get_width() as usize {
                    -((self.cursor_pos - self.screen.get_width() as usize) as i32) - 1
                } else {
                    0
                },
                0,
                &self.input_buffer,
                self.options.style.fg,
                self.options.style.bg,
            );
            self.dirty = false;
        }
        let current_cursor_pos =
            std::cmp::min(self.cursor_pos as i32, self.screen.get_width() as i32 - 1);
        if let Ok(mut cursor_pxl) = self.screen.get_pxl(current_cursor_pos, 0) {
            if self.active && tick % 2 == 0 {
                cursor_pxl.bg = self.options.style.fg;
                cursor_pxl.fg = self.options.style.bg;
            } else {
                cursor_pxl.bg = self.options.style.bg;
                cursor_pxl.fg = self.options.style.fg;
            }
            self.screen.set_pxl(current_cursor_pos, 0, cursor_pxl);
        }
        &self.screen
    }
}

/// Hidden text input
///
/// This form field generates a generic text input, that'll hide what the user writes in it. (e.g. for passwords)
/// This field is inactive by default, you need to set it active once created
///
/// Custom options:
/// - `default`: prefilled value, even on reset
///
/// Outputs `FormValue::String`
///
/// see example `form-input` for basic usage
pub struct HiddenText {
    screen: Screen,
    dirty: bool,
    active: bool,
    hide_character: char,
    input_buffer: String,
    default_input: Option<String>,
    cursor_pos: usize,
    options: FormOptions,
}

impl HiddenText {
    pub fn new(w: u32, hide_character: char, options: FormOptions) -> Self {
        let default = if let Some(FormValue::String(value)) = options.custom.get("default") {
            Some(value.clone())
        } else {
            None
        };
        let mut text = Self {
            screen: Screen::new(w, 1),
            dirty: true,
            active: false,
            hide_character,
            input_buffer: default.clone().unwrap_or_default(),
            default_input: default,
            cursor_pos: 0,
            options,
        };
        text.move_cursor(i32::MAX);
        text
    }

    pub fn set_input_buffer(&mut self, input: &str) {
        self.dirty = true;
        self.input_buffer = String::from(input);
        self.move_cursor(i32::MAX);
    }

    pub fn clear_input_buffer(&mut self) {
        self.dirty = true;
        self.input_buffer = String::new();
        self.cursor_pos = 0;
    }

    /// Insert a character at the position of the cursor
    pub fn put_char(&mut self, chr: char) {
        let mut new_buffer = String::with_capacity(self.input_buffer.capacity() + 1);
        new_buffer.extend(
            self.input_buffer
                .chars()
                .take(self.cursor_pos)
                .chain(std::iter::once(chr))
                .chain(self.input_buffer.chars().skip(self.cursor_pos)),
        );
        self.input_buffer = new_buffer;
        self.move_cursor(1);
    }

    /// Removes a certain amount of characters either on the left (positive) or right (negative) side of the cursor
    pub fn remove_char(&mut self, amount: i32) {
        if amount == 0 {
            return;
        }
        self.dirty = true;
        let off_l = amount.max(0) as usize; // offset to the left from cursor, `positive` or 0
        let off_r = amount.min(0).abs() as usize; // offset to the right from cursor,  `-negative` or 0
        let pos_l = self.cursor_pos.saturating_sub(off_l);
        let pos_r = self
            .cursor_pos
            .saturating_add(off_r)
            .min(self.input_buffer.len());
        self.input_buffer = self.input_buffer.chars().take(pos_l).collect::<String>()
            + &self.input_buffer.chars().skip(pos_r).collect::<String>(); // this skips the cursor +/- offsets
        self.move_cursor(-amount.max(0));
    }

    pub fn move_cursor(&mut self, amount: i32) {
        self.dirty = true;
        self.cursor_pos = (self.cursor_pos as i64 + amount as i64)
            .clamp(0, self.input_buffer.len() as i64) as usize;
    }
}

impl FormField for HiddenText {
    fn make(w: u32, options: FormOptions) -> Self
    where
        Self: Sized,
    {
        Self::new(w, '*', options)
    }

    fn reset(&mut self) {
        self.clear_input_buffer();
        if let Some(default_input) = self.default_input.clone() {
            self.set_input_buffer(&default_input);
        }
    }

    fn get_width(&self) -> u32 {
        self.screen.get_width()
    }

    fn get_height(&self) -> u32 {
        self.screen.get_height()
    }

    fn resize(&mut self, w: u32, _h: u32) {
        self.dirty = true;
        self.screen.resize(w, 1);
    }

    fn handle_event(&mut self, event: Event) {
        if !self.active {
            return;
        }
        if let Event::Key(KeyEvent { code, modifiers }) = event {
            match code {
                KeyCode::Backspace => self.remove_char(1),
                KeyCode::Delete => self.remove_char(-1),
                KeyCode::Left => self.move_cursor(-1),
                KeyCode::Right => self.move_cursor(1),
                KeyCode::Home => self.move_cursor(i32::MIN),
                KeyCode::End => self.move_cursor(i32::MAX),
                KeyCode::Char(c) => {
                    if modifiers.is_empty()
                        || modifiers == KeyModifiers::CONTROL | KeyModifiers::ALT
                    {
                        self.put_char(c);
                    }
                    if modifiers == KeyModifiers::SHIFT {
                        // I don't understand why it works this way but not the other
                        if c.is_ascii_uppercase() {
                            self.put_char(c.to_ascii_uppercase());
                        } else {
                            self.put_char(c.to_ascii_lowercase());
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn validate(&self, validation_result: &mut FormValidationResult) {
        self.self_validate(validation_result);
    }

    fn get_output(&self) -> FormValue {
        FormValue::String(self.input_buffer.to_string())
    }

    fn set_options(&mut self, options: FormOptions) {
        self.options = options;
        self.default_input =
            if let Some(FormValue::String(value)) = self.options.custom.get("default") {
                Some(value.clone())
            } else {
                None
            };
    }

    fn get_options(&self) -> &FormOptions {
        &self.options
    }

    fn draw(&mut self, tick: usize) -> &Screen {
        if self.dirty {
            self.screen.fill(pixel::pxl_fbg(
                ' ',
                self.options.style.fg,
                self.options.style.bg,
            ));
            if !self.input_buffer.is_empty() {
                self.screen.h_line(
                    if self.cursor_pos >= self.screen.get_width() as usize {
                        -((self.cursor_pos - self.screen.get_width() as usize) as i32) - 1
                    } else {
                        0
                    },
                    0,
                    self.input_buffer.len() as i32 - 1,
                    pixel::pxl_fbg(
                        self.hide_character,
                        self.options.style.fg,
                        self.options.style.bg,
                    ),
                );
            }
            self.dirty = false;
        }
        let current_cursor_pos =
            std::cmp::min(self.cursor_pos as i32, self.screen.get_width() as i32 - 1);
        if let Ok(mut cursor_pxl) = self.screen.get_pxl(current_cursor_pos, 0) {
            if self.active && tick % 2 == 0 {
                cursor_pxl.bg = self.options.style.fg;
                cursor_pxl.fg = self.options.style.bg;
            } else {
                cursor_pxl.bg = self.options.style.bg;
                cursor_pxl.fg = self.options.style.fg;
            }
            self.screen.set_pxl(current_cursor_pos, 0, cursor_pxl);
        }
        &self.screen
    }
}
