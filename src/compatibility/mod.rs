#![cfg(feature = "compatibility")]

use crate::screen::Screen;

#[cfg(feature = "compatibility-drawille")]
pub mod drawille;

pub trait AsScreen {
    fn as_screen(&self) -> Option<Screen>;
}
