use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{events::Event, screen::Screen};

use super::{FormField, FormOptions, FormStyle};

/// Button
///
/// This form field generates a button, that handles keyboard input (space to click)
/// This field is inactive by default, you need to set it active once created
pub struct Button {
    screen: Screen,
    dirty: bool,
    active: bool,
    clicked: bool,
    style: FormStyle,
    options: FormOptions,
}
impl Button {
    pub fn new(w: u32, h: u32, options: Option<FormOptions>, style: Option<FormStyle>) -> Self {
        Self {
            screen: Screen::new(w, h),
            dirty: true,
            active: false,
            clicked: false,
            style: style.unwrap_or_default(),
            options: options.unwrap_or_default(),
        }
    }
}

impl FormField for Button {
    fn make(w: u32, h: u32, options: Option<FormOptions>, style: Option<FormStyle>) -> Self
    where
        Self: Sized,
    {
        Self::new(w, h, options, style)
    }

    fn get_width(&self) -> u32 {
        self.screen.get_width()
    }

    fn get_height(&self) -> u32 {
        self.screen.get_height()
    }

    fn resize(&mut self, w: u32, h: u32) {
        self.screen.resize(w, h)
    }

    fn handle_event(&mut self, event: &crate::events::Event) {
        if let Event::Key(KeyEvent { code, modifiers }) = event {
            if *code == KeyCode::Char(' ') && *modifiers == KeyModifiers::NONE {
                self.clicked = true
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn validate(&self, validation_result: &mut super::FormValidationResult) {
        todo!()
    }

    fn get_output(&self) -> super::FormOutput {
        super::FormOutput::Boolean(self.clicked)
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

    fn draw(&mut self, tick: usize) -> &Screen {
        todo!()
    }
}
