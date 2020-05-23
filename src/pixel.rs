//! Character and color management

use crossterm::style::Color;

/// # Pixel structure
/// contains color data and character data
#[derive(Clone, Eq, PartialEq)]
pub struct Pixel {
    pub bg: Color,
    pub fg: Color,
    pub chr: char,
}
impl Pixel {
    pub fn get_colors(&self) -> (Color, Color) {
        (self.fg, self.bg)
    }
}

/// Generate a pixel using a character, a foreground and background color
///
/// usage:
/// ```
/// use console_engine::pixel;
/// use console_engine::Color;
/// // ...
/// engine.set_pxl(0,0,pixel::pxl_fbg('X', Color::Blue, Color::White));
/// ```
pub fn pxl_fbg(value: char, fg: Color, bg: Color) -> Pixel {
    Pixel { bg, fg, chr: value }
}

/// Generate a pixel using a character and a foreground color.  
/// Background color is always black.
///
/// usage:
/// ```
/// use console_engine::pixel;
/// use console_engine::Color;
/// // ...
/// engine.set_pxl(0,0,pixel::pxl_fg('X', Color::Cyan));
/// ```
pub fn pxl_fg(value: char, fg: Color) -> Pixel {
    Pixel {
        fg,
        bg: Color::Reset,
        chr: value,
    }
}
/// Generate a pixel using a character and a background color.  
/// Foreground color is always White.
///
/// usage:
/// ```
/// use console_engine::pixel;
/// use console_engine::Color;
/// // ...
/// engine.set_pxl(0,0,pixel::pxl_bg('X', Color::Magenta));
/// ```
pub fn pxl_bg(value: char, bg: Color) -> Pixel {
    Pixel {
        fg: Color::Reset,
        bg,
        chr: value,
    }
}

/// Generate a pixel using a character  
/// Foreground color is always White.  
/// Background color is always black.
///
/// usage:
/// ```
/// use console_engine::pixel;
/// // ...
/// engine.set_pxl(0,0,pixel::pxl('X'));
/// ```
pub fn pxl(value: char) -> Pixel {
    Pixel {
        fg: Color::Reset,
        bg: Color::Reset,
        chr: value,
    }
}
