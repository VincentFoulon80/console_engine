use crossterm::style::Color;

use crate::pixel::{self, Pixel};

/// Borders for styled-border rectangle
#[derive(Copy, Clone)]
pub struct BorderStyle {
    pub corner_top_left: Pixel,
    pub corner_top_right: Pixel,
    pub corner_bottom_left: Pixel,
    pub corner_bottom_right: Pixel,
    pub top_bottom: Pixel,
    pub left_right: Pixel,
}

impl BorderStyle {
    /// Simple border (uses only ascii characters +, -, |)
    pub fn new_simple() -> Self {
        Self {
            corner_top_right: pixel::pxl('+'),
            corner_top_left: pixel::pxl('+'),
            corner_bottom_left: pixel::pxl('+'),
            corner_bottom_right: pixel::pxl('+'),
            top_bottom: pixel::pxl('-'),
            left_right: pixel::pxl('|'),
        }
    }

    /// Solid (Uses only the block character from ascii)
    pub fn new_solid() -> Self {
        Self {
            corner_top_right: pixel::pxl('█'),
            corner_top_left: pixel::pxl('█'),
            corner_bottom_left: pixel::pxl('█'),
            corner_bottom_right: pixel::pxl('█'),
            top_bottom: pixel::pxl('█'),
            left_right: pixel::pxl('█'),
        }
    }

    /// Light border (uses Box Drawings Light set from unicode)
    pub fn new_light() -> Self {
        Self {
            corner_top_right: pixel::pxl('┐'),
            corner_top_left: pixel::pxl('┌'),
            corner_bottom_left: pixel::pxl('└'),
            corner_bottom_right: pixel::pxl('┘'),
            top_bottom: pixel::pxl('─'),
            left_right: pixel::pxl('│'),
        }
    }

    /// Heavy border (uses Box Drawings Heavy set from unicode)
    pub fn new_heavy() -> Self {
        Self {
            corner_top_right: pixel::pxl('┓'),
            corner_top_left: pixel::pxl('┏'),
            corner_bottom_left: pixel::pxl('┗'),
            corner_bottom_right: pixel::pxl('┛'),
            top_bottom: pixel::pxl('━'),
            left_right: pixel::pxl('┃'),
        }
    }

    /// Double border (uses Box Drawings Double set from unicode)
    pub fn new_double() -> Self {
        Self {
            corner_top_right: pixel::pxl('╗'),
            corner_top_left: pixel::pxl('╔'),
            corner_bottom_left: pixel::pxl('╚'),
            corner_bottom_right: pixel::pxl('╝'),
            top_bottom: pixel::pxl('═'),
            left_right: pixel::pxl('║'),
        }
    }

    /// Creates user-defined border style with specified Pixel's structs
    pub fn new(
        corner_top_left: Pixel,
        corner_top_right: Pixel,
        corner_bottom_left: Pixel,
        corner_bottom_right: Pixel,
        top_bottom: Pixel,
        left_right: Pixel,
    ) -> Self {
        Self {
            corner_top_right,
            corner_top_left,
            corner_bottom_left,
            corner_bottom_right,
            top_bottom,
            left_right,
        }
    }

    /// Changes the border's colors
    pub fn with_colors(&mut self, fg: Color, bg: Color) {
        self.corner_top_right.fg = fg;
        self.corner_top_right.bg = bg;
        self.corner_top_left.fg = fg;
        self.corner_top_left.bg = bg;
        self.corner_bottom_left.fg = fg;
        self.corner_bottom_left.bg = bg;
        self.corner_bottom_right.fg = fg;
        self.corner_bottom_right.bg = bg;
        self.top_bottom.fg = fg;
        self.top_bottom.bg = bg;
        self.left_right.fg = fg;
        self.left_right.bg = bg;
    }
}
