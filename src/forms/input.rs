use std::cmp::Ordering;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{events::Event, screen::Screen};

use super::{ConsoleForm, FormOptions, FormOutput, FormStyle, FormValidationResult};

pub struct TextInput {
    screen: Screen,
    dirty: bool,
    active: bool,
    input_buffer: String,
    cursor_pos: usize,
    style: FormStyle,
    options: FormOptions,
}

impl TextInput {
    pub fn new(w: u32, options: Option<FormOptions>, style: Option<FormStyle>) -> Self {
        Self {
            screen: Screen::new(w, 1),
            dirty: true,
            active: false,
            input_buffer: String::new(),
            cursor_pos: 0,
            style: style.unwrap_or_default(),
            options: options.unwrap_or_default(),
        }
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

    pub fn put_char(&mut self, chr: char) {
        self.dirty = true;
        let mut new_buffer = self
            .input_buffer
            .chars()
            .take(self.cursor_pos)
            .collect::<String>();
        new_buffer.push(chr);
        new_buffer.push_str(
            &self
                .input_buffer
                .chars()
                .skip(self.cursor_pos)
                .collect::<String>(),
        );
        self.input_buffer = new_buffer;
        self.move_cursor(1);
    }

    pub fn remove_char(&mut self, amount: i32) {
        match amount.cmp(&0) {
            Ordering::Greater => {
                self.dirty = true;
                let mut new_buffer = self
                    .input_buffer
                    .chars()
                    .take((self.cursor_pos as i32 - amount) as usize)
                    .collect::<String>();
                new_buffer.push_str(
                    &self
                        .input_buffer
                        .chars()
                        .skip(self.cursor_pos)
                        .collect::<String>(),
                );
                self.input_buffer = new_buffer;
                self.move_cursor(-amount);
            }
            Ordering::Less => {
                self.dirty = true;
                let mut new_buffer = self
                    .input_buffer
                    .chars()
                    .take(self.cursor_pos)
                    .collect::<String>();
                new_buffer.push_str(
                    &self
                        .input_buffer
                        .chars()
                        .skip((self.cursor_pos as i32 - amount) as usize)
                        .collect::<String>(),
                );
                self.input_buffer = new_buffer;
            }
            Ordering::Equal => {}
        }
    }

    pub fn move_cursor(&mut self, amount: i32) {
        self.dirty = true;
        self.cursor_pos = (self.cursor_pos as i64 + amount as i64)
            .clamp(0, self.input_buffer.len() as i64) as usize;
    }
}

impl ConsoleForm for TextInput {
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

    fn handle_event(&mut self, event: &Event) {
        if self.active {
            if let Event::Key(KeyEvent { code, modifiers }) = event {
                match code {
                    KeyCode::Backspace => self.remove_char(1),
                    KeyCode::Delete => self.remove_char(-1),
                    KeyCode::Left => self.move_cursor(-1),
                    KeyCode::Right => self.move_cursor(1),
                    KeyCode::Home => self.move_cursor(i32::MIN),
                    KeyCode::End => self.move_cursor(i32::MAX),
                    KeyCode::Char(c) => {
                        if modifiers.is_empty() {
                            self.put_char(*c);
                        }
                        if *modifiers == KeyModifiers::SHIFT {
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
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn get_output(&self) -> FormOutput {
        FormOutput::String(self.input_buffer.to_string())
    }

    fn draw(&mut self, tick: usize) -> &Screen {
        if self.dirty {
            self.screen.clear();
            self.screen.print_fbg(
                if self.cursor_pos >= self.screen.get_width() as usize {
                    -((self.cursor_pos - self.screen.get_width() as usize) as i32) - 1
                } else {
                    0
                },
                0,
                &self.input_buffer,
                self.style.fg,
                self.style.bg,
            );
            self.dirty = false;
        }
        let current_cursor_pos =
            std::cmp::min(self.cursor_pos as i32, self.screen.get_width() as i32 - 1);
        if self.active && tick % 2 == 0 {
            if let Ok(mut cursor_pxl) = self.screen.get_pxl(current_cursor_pos, 0) {
                cursor_pxl.bg = self.style.fg;
                cursor_pxl.fg = self.style.bg;
                self.screen.set_pxl(current_cursor_pos, 0, cursor_pxl);
            }
        } else if let Ok(mut cursor_pxl) = self.screen.get_pxl(current_cursor_pos, 0) {
            cursor_pxl.bg = self.style.bg;
            cursor_pxl.fg = self.style.fg;
            self.screen.set_pxl(current_cursor_pos, 0, cursor_pxl);
        }
        &self.screen
    }

    fn make(w: u32, _h: u32, options: Option<FormOptions>, style: Option<FormStyle>) -> Self
    where
        Self: Sized,
    {
        Self::new(w, options, style)
    }

    fn validate(&self, validation_result: &mut FormValidationResult) {
        self.self_validate(validation_result);
    }

    fn set_style(&mut self, style: FormStyle) {
        self.style = style
    }

    fn get_style(&self) -> &FormStyle {
        &self.style
    }

    fn set_options(&mut self, options: FormOptions) {
        self.options = options
    }

    fn get_options(&self) -> &FormOptions {
        &self.options
    }
}
