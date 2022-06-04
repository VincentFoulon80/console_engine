#![cfg(feature = "compatibility-drawille")]

extern crate drawille;

use drawille::{Canvas, PixelColor};

use crate::{pixel, screen::Screen, Color};

use super::AsScreen;

impl AsScreen for Canvas {
    fn as_screen(&self) -> Option<crate::screen::Screen> {
        let mut screen = Screen::new(self.width() as u32, self.height() as u32);

        let mut maxrow = self.width();
        let mut maxcol = self.height();
        for &(x, y) in self.chars().keys() {
            if x > maxrow {
                maxrow = x;
            }
            if y > maxcol {
                maxcol = y;
            }
        }

        let mut result = Vec::with_capacity(maxcol as usize + 1);
        for y in 0..=maxcol {
            let mut row = Vec::with_capacity(maxrow as usize + 1);
            for x in 0..=maxrow {
                let cell = self.chars().get(&(x, y)).cloned().unwrap_or((
                    0,
                    ' ',
                    false,
                    PixelColor::White,
                ));
                match cell {
                    (0, _, _, _) => screen.set_pxl(x, y, pixel::pxl(cell.1)),
                    (_, _, false, _) => screen.set_pxl(
                        x,
                        y,
                        pixel::pxl(char::from_u32(0x2800 + cell.0 as u32).unwrap()),
                    ),
                    (_, _, true, _) => screen.set_pxl(
                        x,
                        y,
                        pixel::pxl_fg(
                            char::from_u32(0x2800 + cell.0 as u32).unwrap(),
                            match cell.3 {
                                PixelColor::Black => Color::Black,
                                PixelColor::Red => Color::DarkRed,
                                PixelColor::Green => Color::DarkGreen,
                                PixelColor::Yellow => Color::DarkYellow,
                                PixelColor::Blue => Color::DarkBlue,
                                PixelColor::Magenta => Color::DarkMagenta,
                                PixelColor::Cyan => Color::DarkCyan,
                                PixelColor::White => Color::Grey,
                                PixelColor::BrightBlack => Color::DarkGrey,
                                PixelColor::BrightRed => Color::Red,
                                PixelColor::BrightGreen => Color::Green,
                                PixelColor::BrightYellow => Color::Yellow,
                                PixelColor::BrightBlue => Color::Blue,
                                PixelColor::BrightMagenta => Color::Magenta,
                                PixelColor::BrightCyan => Color::Cyan,
                                PixelColor::BrightWhite => Color::White,
                                PixelColor::TrueColor { r, g, b } => Color::Rgb { r, g, b },
                            },
                        ),
                    ),
                };
            }
        }
        Some(screen)
    }
}
