//! Standalone screens

use crate::rect_style::BorderStyle;

use super::crossterm::style::Color;
use super::crossterm::{execute, style};
use super::pixel;
use super::pixel::Pixel;

/// Screen structure
///
/// A standalone structure that provides every drawing function that ConsoleEngine provides.
///
/// You can get the full content of the screen via the [draw](#method.draw) method.
#[derive(Clone)]
pub struct Screen {
    width: u32,
    height: u32,
    screen: Vec<Pixel>,
    empty: bool,
}

/// # Basic Usage :
///
/// ```
/// use console_engine::pixel;
/// use console_engine::screen::Screen;
/// use console_engine::Color;
///
/// fn main() {
///     // create a screen of 20x11 characters
///     let mut scr = screen::Screen::new(20,11);
///
///     // draw some shapes and prints some text
///     scr.rect(0,0, 19,10,pixel::pxl('#'));
///     scr.fill_circle(5,5, 3, pixel::pxl_fg('*', Color::Blue));
///     scr.print(11,4, "Hello,");
///     scr.print(11,5, "World!");
///
///     // print the screen to the terminal
///     scr.draw();
/// }
/// ```
#[allow(clippy::needless_doctest_main)]
impl Screen {
    /// Creates a new Screen object with the provided width and height.
    pub fn new(width: u32, height: u32) -> Screen {
        Self::new_fill(width, height, pixel::pxl(' '))
    }

    /// Creates a new empty Screen object with the provided widht and height
    /// Makes sure to [`clear`](#method.clear) or [`fill`](#method.fill) it before drawing anything
    pub fn new_empty(width: u32, height: u32) -> Screen {
        let mut scr = Self::new_fill(width, height, pixel::pxl('\u{0}'));
        scr.empty = true;
        scr
    }

    /// Creates a new Screen object with the provided width and height filled with a specific Pixel
    pub fn new_fill(width: u32, height: u32, pixel: Pixel) -> Screen {
        Screen {
            width,
            height,
            screen: vec![pixel; (width * height) as usize],
            empty: false,
        }
    }

    /// Creates a new Screen object with the provided Vec<Pixel> structure fitting the width and height parameters.
    /// The Vec length must correspond to width*height
    pub fn from_vec(vec: Vec<Pixel>, width: u32, height: u32) -> Screen {
        assert!(vec.len() == (width*height) as usize, "The Vec structure must have the length corresponding to width*height (={}) but the given Vec has a length of {}.", width*height, vec.len());
        Screen {
            width,
            height,
            screen: vec,
            empty: false,
        }
    }

    /// Creates a new Screen object with the provided String and colors fitting the width and height parameters.
    /// The String length must correspond to width*height
    pub fn from_string(string: String, fg: Color, bg: Color, width: u32, height: u32) -> Screen {
        assert!(string.chars().count() == (width*height) as usize, "The String must have the length corresponding to width*height (={}) but the given String has a length of {}.", width*height, string.chars().count());
        let vec: Vec<Pixel> = string
            .chars()
            .map(|chr| pixel::pxl_fbg(chr, fg, bg))
            .collect();
        Screen::from_vec(vec, width, height)
    }

    /// Get the screen width
    pub fn get_width(&self) -> u32 {
        self.width
    }
    /// Get the screen height
    pub fn get_height(&self) -> u32 {
        self.height
    }

    /// Reset the screen to a blank state
    pub fn clear(&mut self) {
        self.fill(pixel::pxl(' '));
    }

    /// Fill the entire screen to the given pixel
    pub fn fill(&mut self, pixel: Pixel) {
        self.empty = pixel.chr == '\u{0}';
        self.screen = vec![pixel; (self.width * self.height) as usize];
    }

    /// checks whenever the screen is full of "zero" characters
    /// refresh internal "empty" value
    pub fn check_empty(&mut self) -> bool {
        for pxl in self.screen.iter() {
            if pxl.chr != '\u{0}' {
                self.empty = false;
                return false;
            }
        }
        self.empty = true;
        true
    }

    /// Returns a cached result of [check_empty](#method.check_empty)
    pub fn is_empty(&self) -> bool {
        self.empty
    }

    /// prints a string at the specified coordinates.  
    /// The string will be cropped if it reach the right border
    ///
    /// usage:
    /// ```
    /// screen.print(0, 0, "Hello, world!");
    /// screen.print(0, 4, format!("Score: {}", score).as_str());
    /// ```
    pub fn print(&mut self, x: i32, y: i32, string: &str) {
        self.print_fbg(x, y, string, Color::Reset, Color::Reset)
    }

    /// prints a string at the specified coordinates with the specified foreground and background color  
    /// The string will be cropped if it reach the right border
    ///
    /// usage:
    /// ```
    /// use console_engine::Color;
    ///
    /// // print "Hello, world" in blue on white background
    /// screen.print(0, 0, "Hello, world!", Color::Blue, Color::White);
    /// ```
    pub fn print_fbg(&mut self, x: i32, y: i32, string: &str, fg: Color, bg: Color) {
        if x < self.width as i32 && y < self.height as i32 {
            let mut string = string;
            let mut y = y;
            // if the cursor is above the screen
            if y < 0 {
                // cuts the string per \n character
                // until the cursor enters the screen
                let mut delta_y = -y;
                while delta_y > 0 {
                    if let Some(pos) = string.find('\n') {
                        string = &string[pos + 1..];
                        delta_y -= 1;
                    } else {
                        // no more rows
                        string = "";
                        break;
                    }
                }
                y = 0;
            }
            // get screen index, initializes a counter
            let mut pos = self.coord_to_index(std::cmp::max(0, x), y);
            let delta_x = if x < 0 { -x } else { 0 };
            // set an ignore count to skip a certain number of chars if the text is hidden on the left
            let mut ignore_count = delta_x;
            let mut origin_row = pos / self.get_width() as usize;
            // place each characters one by one. Stops before overflowing
            for str_chr in string.chars() {
                let mut chr = str_chr;
                // process carret return and new line characters
                if chr == '\n' {
                    y += 1;
                    origin_row += 1;
                    ignore_count = delta_x;
                    if y >= self.height as i32 {
                        break;
                    }
                }
                if chr == '\n' || chr == '\r' {
                    // the cursor is sent back to the x index
                    // instead of rolling back on the left of the screen
                    pos = self.coord_to_index(std::cmp::max(0, x), y);
                    continue;
                }
                // tabs are ignored, replaced by space
                if chr == '\t' {
                    chr = ' ';
                }

                if ignore_count <= 0 {
                    // write on the screen until the row changes,
                    // skip the rest until a \n character is found
                    if origin_row == pos / self.get_width() as usize {
                        self.screen[pos] = pixel::pxl_fbg(chr, fg, bg);
                        pos += 1;
                    }
                } else {
                    ignore_count -= 1;
                }
            }
        }
    }

    /// Prints another screen on specified coordinates.
    /// Useful when you want to manage several "subscreen"
    ///
    /// usage:
    /// ```
    /// use console_engine::pixel;
    /// use console_engine::screen::Screen;
    ///
    /// // create a new Screen struct and draw a square inside it
    /// let mut my_square = Screen::new(8,8);
    /// my_square.rect(0,0,7,7,pixel::pxl('#'));
    /// my_square.print(1,1,"square");
    ///
    /// // prints the square in the main window at a specific location
    /// screen.print_screen(5,2, &my_square);
    /// ```
    pub fn print_screen(&mut self, x: i32, y: i32, source: &Screen) {
        for j in 0..source.get_height() as i32 {
            for i in 0..source.get_width() as i32 {
                // unwrap here because we are sure that we won't get out of range
                self.set_pxl(x + i, y + j, source.get_pxl(i, j).unwrap());
            }
        }
    }

    /// Prints another screen on specified coordinates, ignoring a specific character while printing
    /// Ignoring a character will behave like transparency
    ///
    /// see [print_screen](#method.print_screen) for usage
    pub fn print_screen_alpha(&mut self, x: i32, y: i32, source: &Screen, alpha_character: char) {
        for j in 0..source.get_height() as i32 {
            for i in 0..source.get_width() as i32 {
                // unwrap here because we are sure that we won't get out of range
                let pxl = source.get_pxl(i, j).unwrap();
                if pxl.chr != alpha_character {
                    self.set_pxl(x + i, y + j, pxl);
                }
            }
        }
    }

    /// Optimized horizontal line drawing
    /// Automatically called by [line](#method.line) if needed
    pub fn h_line(&mut self, start_x: i32, start_y: i32, end_x: i32, character: Pixel) {
        let start = if start_x > end_x { end_x } else { start_x };
        let end = if start_x > end_x {
            start_x + 1
        } else {
            end_x + 1
        };
        for i in start..end {
            self.set_pxl(i, start_y, character);
        }
    }

    /// Optimized vertical line drawing
    /// Automatically called by [line](#method.line) if needed
    pub fn v_line(&mut self, start_x: i32, start_y: i32, end_y: i32, character: Pixel) {
        let start = if start_y > end_y { end_y } else { start_y };
        let end = if start_y > end_y {
            start_y + 1
        } else {
            end_y + 1
        };
        for j in start..end {
            self.set_pxl(start_x, j, character);
        }
    }

    /// draws a line of the provided character between two sets of coordinates  
    /// see: [Bresenham's line algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm)
    ///
    /// Note : Your line can start or end out of bounds. These pixels won't be drawn
    ///
    /// usage:
    /// ```
    /// use console_engine::pixel;
    /// // ...
    /// screen.line(0, 0, 9, 9, pixel::pxl('#'));
    /// ```
    pub fn line(&mut self, start_x: i32, start_y: i32, end_x: i32, end_y: i32, character: Pixel) {
        let delta_x = end_x - start_x;
        let delta_y = end_y - start_y;
        // use optimized functions for pure horizontal or vertical lines
        if delta_y == 0 {
            self.h_line(start_x, start_y, end_x, character);
            return;
        }
        if delta_x == 0 {
            self.v_line(start_x, start_y, end_y, character);
            return;
        }

        // Bresenham's line algorithm
        let line_low = |screen: &mut Screen, x0: i32, y0: i32, x1: i32, y1: i32| {
            let dx: i32 = x1 - x0;
            let mut dy: i32 = y1 - y0;
            let mut yi = 1;
            if dy < 0 {
                yi = -1;
                dy = -dy;
            }
            let mut d = 2 * dy - dx;
            let mut y = y0;

            for x in x0..x1 + 1 {
                screen.set_pxl(x, y, character);
                if d > 0 {
                    y += yi;
                    d -= 2 * dx;
                }
                d += 2 * dy;
            }
        };

        let line_high = |screen: &mut Screen, x0: i32, y0: i32, x1: i32, y1: i32| {
            let mut dx = x1 - x0;
            let dy = y1 - y0;
            let mut xi = 1;
            if dx < 0 {
                xi = -1;
                dx = -dx;
            }
            let mut d = 2 * dx - dy;
            let mut x = x0;

            for y in y0..y1 + 1 {
                screen.set_pxl(x, y, character);
                if d > 0 {
                    x += xi;
                    d -= 2 * dy;
                }
                d += 2 * dx;
            }
        };

        if (end_y - start_y).abs() < (end_x - start_x).abs() {
            if start_x > end_x {
                line_low(self, end_x, end_y, start_x, start_y);
            } else {
                line_low(self, start_x, start_y, end_x, end_y);
            }
        } else if start_y > end_y {
            line_high(self, end_x, end_y, start_x, start_y);
        } else {
            line_high(self, start_x, start_y, end_x, end_y);
        }
    }

    /// Draws a rectangle of the provided character between two sets of coordinates  
    ///
    /// usage:
    /// ```
    /// use console_engine::pixel;
    /// // ...
    /// screen.rect(0, 0, 9, 9, pixel::pxl('#'));
    /// ```
    pub fn rect(&mut self, start_x: i32, start_y: i32, end_x: i32, end_y: i32, character: Pixel) {
        self.h_line(start_x, start_y, end_x, character); // top
        self.v_line(end_x, start_y, end_y, character); // right
        self.h_line(end_x, end_y, start_x, character); // bottom
        self.v_line(start_x, end_y, start_y, character); // left
    }

    /// Draws a rectangle with custom borders of the provided between two sets of coordinates. Check the BorderStyle struct to learn how to use built-in or custom styles
    ///
    /// usage:
    /// ```
    /// use console_engine::rect_style::BorderStyle;
    /// // ...
    /// screen.rect_border(0, 0, 9, 9, BorderStyle::new_simple());
    /// ```
    pub fn rect_border(
        &mut self,
        start_x: i32,
        start_y: i32,
        end_x: i32,
        end_y: i32,
        rect_style: BorderStyle,
    ) {
        self.h_line(start_x, start_y, end_x, rect_style.top_bottom); // top
        self.v_line(end_x, start_y, end_y, rect_style.left_right); // right
        self.h_line(end_x, end_y, start_x, rect_style.top_bottom); // bottom
        self.v_line(start_x, end_y, start_y, rect_style.left_right); // top left

        // borders
        self.set_pxl(start_x, start_y, rect_style.corner_top_left); // top left corner
        self.set_pxl(end_x, start_y, rect_style.corner_top_right); // top right corner
        self.set_pxl(start_x, end_y, rect_style.corner_bottom_left); // bottom left corner
        self.set_pxl(end_x, end_y, rect_style.corner_bottom_right); // bottom right corner
    }

    /// Fill a rectangle of the provided character between two sets of coordinates  
    ///
    /// usage:
    /// ```
    /// use console_engine::pixel;
    /// // ...
    /// screen.fill_rect(0, 0, 9, 9, pixel::pxl('#'));
    /// ```
    pub fn fill_rect(
        &mut self,
        start_x: i32,
        start_y: i32,
        end_x: i32,
        end_y: i32,
        character: Pixel,
    ) {
        let y0 = if start_y < end_y { start_y } else { end_y };
        let y1 = if start_y < end_y {
            end_y + 1
        } else {
            start_y + 1
        };
        for y in y0..y1 {
            self.h_line(start_x, y, end_x, character);
        }
    }

    /// Draws a circle of the provided character at an x and y position with a radius
    /// see: [olcPixelGameEngine Repository](https://github.com/OneLoneCoder/olcPixelGameEngine)
    ///
    /// usage:
    /// ```
    /// use console_engine::pixel;
    /// // ...
    /// screen.circle(10, 10, 4, pixel::pxl('#'));
    /// ```
    pub fn circle(&mut self, x: i32, y: i32, radius: u32, character: Pixel) {
        let mut relative_pos_x = 0;
        let mut relative_pos_y = radius as i32;
        let mut distance: i32 = 3 - 2 * radius as i32;
        if radius == 0 {
            return;
        }

        while relative_pos_y >= relative_pos_x {
            self.set_pxl(x + relative_pos_x, y - relative_pos_y, character);
            self.set_pxl(x + relative_pos_y, y - relative_pos_x, character);
            self.set_pxl(x + relative_pos_y, y + relative_pos_x, character);
            self.set_pxl(x + relative_pos_x, y + relative_pos_y, character);
            self.set_pxl(x - relative_pos_x, y + relative_pos_y, character);
            self.set_pxl(x - relative_pos_y, y + relative_pos_x, character);
            self.set_pxl(x - relative_pos_y, y - relative_pos_x, character);
            self.set_pxl(x - relative_pos_x, y - relative_pos_y, character);
            if distance < 0 {
                distance += 4 * relative_pos_x as i32 + 6;
                relative_pos_x += 1;
            } else {
                distance += 4 * (relative_pos_x as i32 - relative_pos_y as i32) + 10;
                relative_pos_x += 1;
                relative_pos_y -= 1;
            }
        }
    }

    /// Fill a circle of the provided character at an x and y position with a radius
    /// see: [olcPixelGameEngine Repository](https://github.com/OneLoneCoder/olcPixelGameEngine)
    ///
    /// usage:
    /// ```
    /// use console_engine::pixel;
    /// // ...
    /// screen.fill_circle(10, 10, 4, pixel::pxl('#'));
    /// ```
    pub fn fill_circle(&mut self, x: i32, y: i32, radius: u32, character: Pixel) {
        // Taken from wikipedia
        let mut relative_pos_x = 0;
        let mut relative_pos_y = radius as i32;
        let mut distance: i32 = 3 - 2 * radius as i32;
        if radius == 0 {
            return;
        }

        // create a lambda function that draw fast horizontal lines
        let mut drawline = |start_x: i32, end_x: i32, y: i32| {
            for i in start_x..end_x + 1 {
                self.set_pxl(i, y, character);
            }
        };

        while relative_pos_y >= relative_pos_x {
            // Modified to draw scan-lines instead of edges
            drawline(x - relative_pos_x, x + relative_pos_x, y - relative_pos_y);
            drawline(x - relative_pos_y, x + relative_pos_y, y - relative_pos_x);
            drawline(x - relative_pos_x, x + relative_pos_x, y + relative_pos_y);
            drawline(x - relative_pos_y, x + relative_pos_y, y + relative_pos_x);
            if distance < 0 {
                distance += 4 * relative_pos_x + 6;
                relative_pos_x += 1;
            } else {
                distance += 4 * (relative_pos_x - relative_pos_y) + 10;
                relative_pos_x += 1;
                relative_pos_y -= 1;
            }
        }
    }

    /// Draws a triangle of the provided character using three sets of coordinates
    ///
    /// usage:
    /// ```
    /// use console_engine::pixel;
    /// // ...
    /// screen.triangle(8,8, 4,6, 9,2, pixel::pxl('#'));
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn triangle(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        character: Pixel,
    ) {
        self.line(x1, y1, x2, y2, character);
        self.line(x2, y2, x3, y3, character);
        self.line(x3, y3, x1, y1, character);
    }

    /// Fill a triangle of the provided character using three sets of coordinates
    /// see: [rustyPixelGameEngine Repository](https://github.com/mattbettcher/rustyPixelGameEngine)
    ///
    /// usage:
    /// ```
    /// use console_engine::pixel;
    /// // ...
    /// screen.fill_triangle(8,8, 4,6, 9,2, pixel::pxl('#'));
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn fill_triangle(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        character: Pixel,
    ) {
        self.triangle(
            x1 as i32, y1 as i32, x2 as i32, y2 as i32, x3 as i32, y3 as i32, character,
        );
        // we use tuples for this for now
        let v0 = (x1 as i32, y1 as i32);
        let mut v1 = (x2 as i32, y2 as i32);
        let mut v2 = (x3 as i32, y3 as i32);

        // algorithm only fills counter clockwise triangles, so swap as needed
        // For a triangle A B C, you can find the winding by computing the cross product (B - A) x (C - A). For 2d tri's, with z=0, it will only have a z component.
        // To give all the same winding, swap vertices C and B if this z component is negative.
        let cross = (v1.1 - v0.1) * (v2.0 - v1.0) - (v1.0 - v0.0) * (v2.1 - v1.1);
        if cross > 0 {
            std::mem::swap(&mut v1, &mut v2)
        }

        // Compute triangle bounding box and clip to screen bounds
        let min_x = std::cmp::max(std::cmp::min(std::cmp::min(v0.0, v1.0), v2.0), 0);
        let max_x = std::cmp::min(
            std::cmp::max(std::cmp::max(v0.0, v1.0), v2.0),
            self.get_width() as i32 - 1,
        );
        let min_y = std::cmp::max(std::cmp::min(std::cmp::min(v0.1, v1.1), v2.1), 0);
        let max_y = std::cmp::min(
            std::cmp::max(std::cmp::max(v0.1, v1.1), v2.1),
            self.get_height() as i32 - 1,
        );

        // Triangle setup
        let a01 = v0.1 - v1.1;
        let b01 = v1.0 - v0.0;
        let a12 = v1.1 - v2.1;
        let b12 = v2.0 - v1.0;
        let a20 = v2.1 - v0.1;
        let b20 = v0.0 - v2.0;

        // Determine edges
        let is_top_left = |v0: (i32, i32), v1: (i32, i32)| -> bool { v0.1 > v1.1 };

        // We follow fill rules and add a bias
        let bias0 = if is_top_left(v1, v2) { 0 } else { -1 };
        let bias1 = if is_top_left(v2, v0) { 0 } else { -1 };
        let bias2 = if is_top_left(v0, v1) { 0 } else { -1 };

        // Determine barycentric coordinates
        let orient2d = |a: (i32, i32), b: (i32, i32), c: (i32, i32)| -> i32 {
            (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)
        };

        let mut p = (min_x, min_y);
        let mut w0_row = orient2d(v1, v2, p) + bias0;
        let mut w1_row = orient2d(v2, v0, p) + bias1;
        let mut w2_row = orient2d(v0, v1, p) + bias2;

        // Rasterize
        for y in min_y..max_y {
            p.1 = y;
            // Barycentric coordinates at start of row
            let mut w0 = w0_row;
            let mut w1 = w1_row;
            let mut w2 = w2_row;

            for x in min_x..max_x {
                p.0 = x;
                // If p is on or inside all edges, render pixel.
                if (w0 | w1 | w2) >= 0 {
                    self.set_pxl(p.0, p.1, character);
                }

                // One step to the right
                w0 += a12;
                w1 += a20;
                w2 += a01;
            }
            // One row step
            w0_row += b12;
            w1_row += b20;
            w2_row += b01;
        }
    }

    /// Scrolls the screen for a certain amount of characters vertically or horizontally
    /// Scrolling is a destructive process, the outer border will be filled with the background pixel.
    ///
    /// Scrolling a positive value will move the screen characters to the left / top,
    /// freeing space to the right / bottom
    ///
    /// Scrolling a negative value will move the screen characters to the right / bottom,
    /// freeing space to the left / top
    ///
    /// usage :
    /// ```
    /// use console_engine::pixel;
    ///
    /// // fill the screen with characters
    /// screen.fill(pixel::pxl('#'));
    /// // free one space to the bottom
    /// screen.scroll(0,1,pixel::pxl(' '));
    /// // print something at this place
    /// screen.print(0, height-1, "Hello, world!");
    /// ```
    pub fn scroll(&mut self, h_scroll: i32, v_scroll: i32, background: Pixel) {
        let width = self.width as i32;
        let height = self.height as i32;
        if h_scroll != 0 {
            // if the scroll is beyond the size of the screen, simply clear it
            if h_scroll >= width || h_scroll <= -width {
                self.fill(background);
            } else if h_scroll > 0 {
                let step = h_scroll as usize;
                // scroll to the left
                for j in 0..height {
                    // move the pixels
                    for i in h_scroll..width {
                        let index = self.coord_to_index(i, j);
                        self.screen[index - step] = self.screen[index];
                    }
                    // fill the gap with background
                    for i in (width - h_scroll)..width {
                        let index = self.coord_to_index(i, j);
                        self.screen[index] = background;
                    }
                }
            } else {
                let step = h_scroll.abs() as usize;
                // scroll to the right
                for j in 0..height {
                    // move the pixels
                    for i in (0..(width - h_scroll.abs())).rev() {
                        let index = self.coord_to_index(i, j);
                        self.screen[index + step] = self.screen[index];
                    }
                    // fill the gap with background
                    for i in 0..h_scroll.abs() {
                        let index = self.coord_to_index(i, j);
                        self.screen[index] = background;
                    }
                }
            }
        }
        if v_scroll != 0 {
            // if the scroll is beyond the size of the screen, simply clear it
            if v_scroll >= height || v_scroll <= -height {
                self.fill(background);
            } else if v_scroll > 0 {
                let step = (width * v_scroll) as usize;
                // scroll to the top
                for i in 0..width {
                    // move the pixels
                    for j in v_scroll..height {
                        let index = self.coord_to_index(i, j);
                        self.screen[index - step] = self.screen[index];
                    }
                    // fill the gap with background
                    for j in (height - v_scroll)..height {
                        let index = self.coord_to_index(i, j);
                        self.screen[index] = background;
                    }
                }
            } else {
                let step = (width.abs() * v_scroll.abs()) as usize;
                // scroll to the bottom
                for i in 0..width {
                    // move the pixels
                    for j in (0..(height - v_scroll.abs())).rev() {
                        let index = self.coord_to_index(i, j);
                        self.screen[index + step] = self.screen[index];
                    }
                    // fill the gap with background
                    for j in 0..v_scroll.abs() {
                        let index = self.coord_to_index(i, j);
                        self.screen[index] = background;
                    }
                }
            }
        }
    }

    /// sets the provided character in the specified coordinates
    /// out of bounds pixels will be ignored
    ///
    /// usage:
    /// ```
    /// use console_engine::pixel;
    /// // ...
    /// screen.set_pxl(3,8,pixel::pixel('o'));
    /// ```
    pub fn set_pxl(&mut self, x: i32, y: i32, character: Pixel) {
        if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
            let index = self.coord_to_index(x, y);
            self.screen[index] = character;
        }
    }

    /// Get the character stored at provided coordinates
    ///
    /// usage:
    /// ```
    /// if screen.get_pxl(3,8).unwrap().chr == 'o' {
    ///     screen.print(0,0,"Found a 'o'");
    /// }
    /// ```
    pub fn get_pxl(&self, x: i32, y: i32) -> Result<Pixel, String> {
        if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
            return Ok(self.screen[self.coord_to_index(x, y)]);
        }
        Err(format!(
            "Attempted to get_pxl out of bounds (coords: [{}, {}], bounds: [{}, {}])",
            x,
            y,
            self.width - 1,
            self.height - 1
        ))
    }

    /// Resizes the screen to match the given width and height
    /// truncates the bottom and right side of the screen
    ///
    /// usage:
    /// ```
    /// screen.resize()
    /// ```
    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        // create new screens Vec
        let mut new_screen = vec![pixel::pxl(' '); (new_width * new_height) as usize];
        // transfer old screens into new screens
        for j in 0..std::cmp::min(self.height, new_height) {
            for i in 0..std::cmp::min(self.width, new_width) {
                if (i as u32) < self.width && (j as u32) < self.height {
                    new_screen[((j * new_width) + i) as usize] =
                        self.screen[((j * self.width) + i) as usize];
                }
            }
        }
        self.screen = new_screen;
        self.width = new_width;
        self.height = new_height;
    }

    /// Extracts part of the current screen as a separate Screen object
    /// The original screen is not altered
    /// If the coordinates are out of bounds, they'll be replace by the `default` pixel
    ///
    /// usage:
    /// ```
    /// use console_engine::pixel;
    /// // extract a 3x2 screen from the screen variable and print it
    /// let scr_chunk = screen.extract(10, 4, 12, 5, pixel::pxl(' '));
    /// scr_chunk.draw();
    /// ```
    pub fn extract(
        &self,
        start_x: i32,
        start_y: i32,
        end_x: i32,
        end_y: i32,
        default: Pixel,
    ) -> Screen {
        let target_width = (end_x - start_x).abs() as u32 + 1;
        let target_height = (end_y - start_y).abs() as u32 + 1;
        let mut extracted_screen = vec![default; (target_width * target_height) as usize];
        let x_reversed = start_x > end_x;
        let y_reversed = start_y > end_y;
        let mut x = if x_reversed {
            target_width as i32 - 1
        } else {
            0
        };
        let mut y = if y_reversed {
            target_height as i32 - 1
        } else {
            0
        };
        for j in if y_reversed {
            end_y..=start_y
        } else {
            start_y..=end_y
        } {
            for i in if x_reversed {
                end_x..=start_x
            } else {
                start_x..=end_x
            } {
                if i >= 0 && i < self.width as i32 && j >= 0 && j < self.height as i32 {
                    extracted_screen[((y * target_width as i32) + x) as usize] =
                        self.screen[self.coord_to_index(i, j)];
                }
                x += if x_reversed { -1 } else { 1 };
            }
            y += if y_reversed { -1 } else { 1 };
            x = if x_reversed {
                target_width as i32 - 1
            } else {
                0
            };
        }
        Screen::from_vec(extracted_screen, target_width, target_height)
    }

    /// Draws the screen into the terminal
    /// Uses stdout as target
    ///
    /// You should not use this function while a ConsoleEngine is running.
    /// You may want to use ConsoleEngine's `print_screen`, `print_screen_alpha` or `set_screen` instead
    pub fn draw(&self) {
        let mut output = std::io::stdout();
        crossterm::terminal::enable_raw_mode().unwrap();
        let mut skip_next = false;
        for i in 0..self.width * self.height {
            let pixel = &self.screen[i as usize];
            if skip_next {
                skip_next = false;
                continue;
            }
            if unicode_width::UnicodeWidthChar::width(pixel.chr).unwrap() > 1 {
                skip_next = true;
            }
            execute!(
                output,
                style::SetForegroundColor(pixel.fg),
                style::SetBackgroundColor(pixel.bg),
                style::Print(pixel.chr)
            )
            .unwrap();
            if i != self.width * self.height - 1 && i % self.width == self.width - 1 {
                execute!(output, style::Print("\r\n")).unwrap();
            }
        }
        crossterm::terminal::disable_raw_mode().unwrap();
    }

    /// Converts x and y coordinates to screen index
    ///
    /// example : on a 10x10 screen
    /// `coord_to_index(2,1)` will return index 12
    fn coord_to_index(&self, x: i32, y: i32) -> usize {
        ((y * self.width as i32) + x) as usize
    }
}
