use termion::color;

/// # Pixel structure
/// contains color data and character data
#[derive(Clone)]
pub struct Pixel {
    pub colors: String,
    pub chr: char
}
impl std::fmt::Display for Pixel {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        fmt.write_str(self.colors.as_str()).unwrap();
        fmt.write_str(self.chr.to_string().as_str()).unwrap();
        Ok(())
    }
}

/// Generate a pixel using a character, a foreground and background color
/// 
/// usage: 
/// ```
/// use console_engine::pixel;
/// use console_engine::termion::color;
/// // ...
/// engine.set_pxl(0,0,pixel::pxl_fbg('X', color::Blue, color::White));
/// ```
pub fn pxl_fbg<C1: color::Color, C2: color::Color>(value: char, fg: C1, bg: C2) -> Pixel {
    Pixel {
        colors: format!("{}{}", color::Fg(fg), color::Bg(bg)),
        chr: value
    }
}

/// Generate a pixel using a character and a foreground color.  
/// Background color is always black.
/// 
/// usage: 
/// ```
/// use console_engine::pixel;
/// use console_engine::termion::color;
/// // ...
/// engine.set_pxl(0,0,pixel::pxl_fg('X', color::Cyan));
/// ```
pub fn pxl_fg<C: color::Color>(value: char, fg: C) -> Pixel {
    Pixel {
        colors: format!("{}{}", color::Fg(fg), color::Bg(color::Black)),
        chr: value
    }
}
/// Generate a pixel using a character and a background color.  
/// Foreground color is always White.
/// 
/// usage: 
/// ```
/// use console_engine::pixel;
/// use console_engine::termion::color;
/// // ...
/// engine.set_pxl(0,0,pixel::pxl_bg('X', color::Magenta));
/// ```
pub fn pxl_bg<C: color::Color>(value: char, bg: C) -> Pixel {
    Pixel {
        colors: format!("{}{}", color::Fg(color::White), color::Bg(bg)),
        chr: value
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
        colors: format!("{}{}", color::Fg(color::White), color::Bg(color::Black)),
        chr: value
    }
}