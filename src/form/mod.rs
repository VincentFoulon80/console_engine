use std::{any::Any, collections::HashMap};

use crossterm::event::{KeyCode, KeyEvent};

use crate::{events::Event, rect_style::BorderStyle, screen::Screen};

pub mod input;

pub trait ToAny {
    fn to_any(self) -> Box<dyn Any>;
}
impl<T: 'static> ToAny for T {
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}

pub trait ConsoleWindow: ToAny {
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
    fn resize(&mut self, w: u32, h: u32);

    fn handle_event(&mut self, event: &Event);

    fn set_active(&mut self, active: bool);
    fn is_active(&self) -> bool;
    fn get_output(&self) -> ConsoleWindowOutput;

    fn draw(&mut self, tick: usize) -> &Screen;
}

pub enum ConsoleWindowOutput {
    None,
    String(String),
    Compound(HashMap<String, ConsoleWindowOutput>),
}

impl Default for ConsoleWindowOutput {
    fn default() -> Self {
        Self::None
    }
}

pub struct Form {
    screen: Screen,
    style: BorderStyle,
    active: bool,
    index: usize,
    dirty: bool,
    fields: Vec<(String, Box<dyn ConsoleWindow>)>,
}

impl Form {
    pub fn new(w: u32, h: u32, border_style: BorderStyle) -> Self {
        Form {
            screen: Screen::new(w, h),
            style: border_style,
            index: 0,
            active: false,
            dirty: true,
            fields: vec![],
        }
    }

    pub fn add_field<T: ConsoleWindow + 'static>(&mut self, name: &str, mut field: T) {
        field.resize(self.get_width() - 2, field.get_height());
        self.fields.push((String::from(name), Box::new(field)));
    }

    pub fn get_field<T>(&mut self, name: &str) -> Option<Box<T>>
    where
        T: ConsoleWindow + 'static,
    {
        let mut index = None;
        for (id, (field_name, _)) in self.fields.iter().enumerate() {
            if name == *field_name {
                index = Some(id);
                break;
            }
        }
        if let Some(id) = index {
            let (_, field) = self.fields.remove(id);
            field.to_any().downcast::<T>().ok()
        } else {
            None
        }
    }

    pub fn get_result(&self, name: &str) -> Option<ConsoleWindowOutput> {
        for (field_name, field) in self.fields.iter() {
            if name == *field_name {
                return Some(field.get_output());
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
}

impl ConsoleWindow for Form {
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
                KeyCode::Enter | KeyCode::Tab => {
                    self.index = (self.index + 1).clamp(0, self.fields.len());
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

    fn get_output(&self) -> ConsoleWindowOutput {
        let mut output: HashMap<String, ConsoleWindowOutput> = HashMap::new();
        for (name, field) in self.fields.iter() {
            output.insert(name.to_string(), field.get_output());
        }
        ConsoleWindowOutput::Compound(output)
    }

    fn draw(&mut self, tick: usize) -> &Screen {
        if self.dirty {
            self.screen.rect_border(
                0,
                0,
                self.get_width() as i32 - 1,
                self.get_height() as i32 - 1,
                self.style,
            );
            let mut current_pos = 1;
            for (name, field) in self.fields.iter_mut() {
                self.screen.print(1, current_pos, name);
                current_pos += 1;
                self.screen.print_screen(1, current_pos, field.draw(tick));
                current_pos += field.get_height() as i32;
            }
        }
        &self.screen
    }
}
