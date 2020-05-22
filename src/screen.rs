//! Standalone screens

use super::pixel;
use super::pixel::Pixel;
use super::termion::color;

/// Screen structure
/// 
/// A standalone structure that provides every drawing function that ConsoleEngine provides.
/// 
/// You can get the full content of the screen via the to_string method.
#[derive(Clone)]
pub struct Screen {
    width: u32,
    height: u32,
    screen: Vec<Pixel>,
    empty: bool
}

/// # Basic Usage :
/// 
/// ```
/// use console_engine::pixel;
/// use console_engine::screen::Screen;
/// use console_engine::termion::color;
/// 
/// fn main() {
///     // create a screen of 20x11 characters
///     let mut scr = screen::Screen::new(20,11);
/// 
///     // draw some shapes and prints some text
///     scr.rect(0,0, 19,10,pixel::pxl('#'));
///     scr.fill_circle(5,5, 3, pixel::pxl('*'));
///     scr.print(11,4, String::from("Hello,"));
///     scr.print(11,5, String::from("World!"));
/// 
///     // print the screen to the terminal
///     println!("{}", scr.to_string());
/// }
/// ```
impl Screen {

    /// Creates a new Screen object with the provided width and height.
    pub fn new(width: u32, height: u32) -> Screen
    {
        Self::new_fill(width, height, pixel::pxl(' '))
    }

    /// Creates a new empty Screen object with the provided widht and height
    /// Makes sure to [`clear`](#method.clear) or [`fill`](#method.fill) it before drawing anything
    pub fn new_empty(width: u32, height: u32) -> Screen
    {
        let mut scr = Self::new_fill(width, height, pixel::pxl('\u{0}'));
        scr.empty = true;
        scr
    }

    /// Creates a new Screen object with the provided width and height filled with a specific Pixel
    pub fn new_fill(width: u32, height: u32, pixel: Pixel) -> Screen
    {
        Screen {
            width,
            height,
            screen: vec![pixel; (width*height) as usize],
            empty: false
        }
    }

    /// Creates a new Screen object with the provided Vec<Pixel> structure fitting the width and height parameters.
    /// The Vec length must correspond to width*height
    pub fn from_vec(vec: Vec<Pixel>, width: u32, height: u32) -> Screen
    {
        assert!(vec.len() == (width*height) as usize, format!("The Vec structure must have the length corresponding to width*height (={}) but the given Vec has a length of {}.", width*height, vec.len()));
        Screen {
            width,
            height,
            screen: vec,
            empty: false
        }
    }

    /// Creates a new Screen object with the provided String and colors fitting the width and height parameters.
    /// The String length must correspond to width*height
    pub fn from_string<C1: color::Color + Clone, C2: color::Color + Clone>(string: String, fg: C1, bg: C2, width: u32, height: u32) -> Screen
    {
        assert!(string.chars().count() == (width*height) as usize, format!("The String must have the length corresponding to width*height (={}) but the given String has a length of {}.", width*height, string.chars().count()));
        let vec: Vec<Pixel> = string.chars()
            .map(|chr| pixel::pxl_fbg(chr, fg.clone(), bg.clone()))
            .collect();
        Screen::from_vec(vec, width, height)
    }

    /// Get the screen width
    pub fn get_width(&self) -> u32
    {
        self.width
    }
    /// Get the screen height
    pub fn get_height(&self) -> u32
    {
        self.height
    }

    /// Reset the screen to a blank state
    pub fn clear(&mut self) 
    {
        self.fill(pixel::pxl(' '));
    }

    // Fill the entire screen to the given pixel
    pub fn fill(&mut self, pixel: Pixel)
    {
        self.empty = pixel.chr == '\u{0}';
        self.screen = vec![pixel; (self.width*self.height) as usize];
    }

    // checks whenever the screen is full of "zero" characters
    // refresh internal "empty" value
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

    pub fn is_empty(&self) -> bool
    {
        self.empty
    }

    /// prints a string at the specified coordinates.  
    /// The string will be cropped if it reach the right border
    /// 
    /// usage:
    /// ```
    /// screen.print(0,0, String::from("Hello, world!"));
    /// screen.print(0, 4, format!("Score: {}", score));
    /// ```
    /// 
    /// examples :
    /// - [drag-and-drop](https://github.com/VincentFoulon80/console_engine/blob/master/examples/drag-and-drop.rs)
    /// - [graph](https://github.com/VincentFoulon80/console_engine/blob/master/examples/graph.rs)
    /// - [lines-fps](https://github.com/VincentFoulon80/console_engine/blob/master/examples/lines-fps.rs)
    /// - [screen-embed](https://github.com/VincentFoulon80/console_engine/blob/master/examples/screen-embed.rs)
    /// - [screen-simple](https://github.com/VincentFoulon80/console_engine/blob/master/examples/screen-simple.rs)
    /// - [screen-swap](https://github.com/VincentFoulon80/console_engine/blob/master/examples/screen-swap.rs)
    /// - [shapes](https://github.com/VincentFoulon80/console_engine/blob/master/examples/shapes.rs)
    /// - [snake](https://github.com/VincentFoulon80/console_engine/blob/master/examples/snake.rs)
    /// - [tetris](https://github.com/VincentFoulon80/console_engine/blob/master/examples/tetris.rs)
    pub fn print(&mut self, x: i32, y: i32, string: &str)
    {
        self.print_fbg(x, y, string, color::White, color::Black)
    }

    /// prints a string at the specified coordinates with the specified foreground and background color  
    /// The string will be cropped if it reach the right border
    /// 
    /// usage:
    /// ```
    /// // print "Hello, world" in blue on white background
    /// screen.print(0,0, String::from("Hello, world!"), color::Blue, color::White);
    /// ```
    /// 
    /// examples :
    /// - [graph](https://github.com/VincentFoulon80/console_engine/blob/master/examples/graph.rs)
    /// - [screen-swap](https://github.com/VincentFoulon80/console_engine/blob/master/examples/screen-swap.rs)
    /// - [snake](https://github.com/VincentFoulon80/console_engine/blob/master/examples/snake.rs)
    /// - [tetris](https://github.com/VincentFoulon80/console_engine/blob/master/examples/tetris.rs)
    pub fn print_fbg<C1: color::Color + Clone, C2: color::Color + Clone>(&mut self, x: i32, y: i32, string: &str, fg: C1, bg: C2)
    {
        if y >= 0 && x < self.width as i32 && y < self.height as i32 {
            // get screen index, initializes a counter 
            // and get chars of the provided String
            let pos = self.coord_to_index(std::cmp::max(0,x), y);
            let delta_x = if x < 0 { x.abs() as usize } else { 0usize };
            let mut count = delta_x;
            let char_vec: Vec<char> = string.chars().collect();
            let origin_row = pos/self.get_width() as usize;
            // place each characters one by one. Stops before overflowing
            for i in pos..std::cmp::min(pos+char_vec.len()-delta_x, self.screen.capacity()) {
                // if the row changes, break. 
                // removing this statement will cause a wrapping of the text
                if origin_row != i/self.get_width() as usize {
                    break;
                }
                // print the character on screen
                self.screen[i] = pixel::pxl_fbg(char_vec[count], fg.clone(), bg.clone());
                count += 1;
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
    /// my_square.print(1,1,String::from("square"));
    /// 
    /// // prints the square in the main window at a specific location
    /// screen.print_screen(5,2, &my_square);
    /// ```
    /// 
    /// examples :
    /// - [screen-embed](https://github.com/VincentFoulon80/console_engine/blob/master/examples/screen-embed.rs)
    /// - [tetris](https://github.com/VincentFoulon80/console_engine/blob/master/examples/tetris.rs)
    pub fn print_screen(&mut self, x : i32, y: i32, source: &Screen)
    {
        for j in 0..source.get_height() as i32 {
            for i in 0..source.get_width() as i32 {
                self.set_pxl_ref(x+i, y+j, &source.get_pxl(i, j).unwrap());
            }
        }
    }

    /// Prints another screen on specified coordinates, ignoring a specific character while printing
    /// Ignoring a character will behave like transparency
    /// 
    /// see [print_screen](#method.print_screen) for usage
    /// 
    /// examples :
    /// - [tetris](https://github.com/VincentFoulon80/console_engine/blob/master/examples/tetris.rs)
    pub fn print_screen_alpha(&mut self, x: i32, y: i32, source: &Screen, alpha_character: char)
    {
        for j in 0..source.get_height() as i32 {
            for i in 0..source.get_width() as i32 {
                if source.get_pxl(i, j).unwrap().chr != alpha_character {
                    self.set_pxl_ref(x+i, y+j, &source.get_pxl(i, j).unwrap());
                }
            }
        }
    }

    /// Optimized horizontal line drawing
    /// Automatically called by line if needed
    pub fn h_line(&mut self, start_x: i32, start_y: i32, end_x: i32, character: Pixel)
    {
        let start = if start_x > end_x {end_x} else {start_x};
        let end = if start_x > end_x {start_x+1} else {end_x+1};
        for i in start..end {
            self.set_pxl_ref(i, start_y, &character);
        }
    }

    /// Optimized vertical line drawing
    /// Automatically called by line if needed
    pub fn v_line(&mut self, start_x: i32, start_y: i32, end_y: i32, character: Pixel)
    {
        let start = if start_y > end_y {end_y} else {start_y};
        let end = if start_y > end_y {start_y+1} else {end_y+1};
        for j in start..end {
            self.set_pxl_ref(start_x, j, &character);
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
    /// 
    /// examples :
    /// - [graph](https://github.com/VincentFoulon80/console_engine/blob/master/examples/graph.rs)
    /// - [lines](https://github.com/VincentFoulon80/console_engine/blob/master/examples/lines.rs)
    /// - [lines-fps](https://github.com/VincentFoulon80/console_engine/blob/master/examples/lines-fps.rs)
    pub fn line(&mut self, start_x: i32, start_y: i32, end_x: i32, end_y: i32, character: Pixel)
    {
        let delta_x = end_x - start_x;
        let delta_y = end_y - start_y;
        // use optimized functions for pure horizontal or vertical lines
        if delta_y == 0 {
            self.h_line(start_x, start_y, end_x,character);
            return;
        }
        if delta_x == 0 {
            self.v_line(start_x, start_y, end_y,character);
            return;
        }

        // Bresenham's line algorithm
        let line_low = |screen: &mut Screen, x0: i32,y0: i32, x1: i32,y1: i32| {
            let dx: i32 = x1 - x0;
            let mut dy: i32 = y1 - y0;
            let mut yi = 1;
            if dy < 0 {
                yi = -1;
                dy = -dy;
            }
            let mut d = 2*dy - dx;
            let mut y = y0;

            for x in x0..x1+1 {
                screen.set_pxl_ref(x, y, &character);
                if d > 0 {
                    y += yi;
                    d -= 2*dx;
                }
                d += 2*dy;
            } 
        };

        let line_high = |screen: &mut Screen, x0: i32,y0: i32, x1: i32,y1: i32| {
            let mut dx = x1 - x0;
            let dy = y1 - y0;
            let mut xi = 1;
            if dx < 0 {
                xi = -1;
                dx = -dx;
            }
            let mut d = 2*dx - dy;
            let mut x = x0;
        
            for y in y0..y1+1 {
                screen.set_pxl_ref(x, y, &character);
                if d > 0 {
                    x += xi;
                    d -= 2*dy;
                }
                d += 2*dx;
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
    /// 
    /// examples :
    /// - [drag-and-drop](https://github.com/VincentFoulon80/console_engine/blob/master/examples/drag-and-drop.rs)
    /// - [screen-embed](https://github.com/VincentFoulon80/console_engine/blob/master/examples/screen-embed.rs)
    /// - [screen-simple](https://github.com/VincentFoulon80/console_engine/blob/master/examples/screen-simple.rs)
    /// - [screen-swap](https://github.com/VincentFoulon80/console_engine/blob/master/examples/screen-swap.rs)
    /// - [shapes](https://github.com/VincentFoulon80/console_engine/blob/master/examples/shapes.rs)
    /// - [tetris](https://github.com/VincentFoulon80/console_engine/blob/master/examples/tetris.rs)
    pub fn rect(&mut self, start_x: i32, start_y: i32, end_x: i32, end_y: i32, character: Pixel)
    {
        self.h_line(start_x, start_y, end_x,   character.clone()); // top
        self.v_line(end_x,   start_y, end_y,   character.clone()); // right
        self.h_line(end_x,   end_y,   start_x, character.clone()); // bottom
        self.v_line(start_x, end_y,   start_y, character);         // left
    }

    /// Fill a rectangle of the provided character between two sets of coordinates  
    /// 
    /// usage: 
    /// ```
    /// use console_engine::pixel;
    /// // ...
    /// screen.fill_rect(0, 0, 9, 9, pixel::pxl('#'));
    /// ```
    /// 
    /// examples :
    /// - [shapes](https://github.com/VincentFoulon80/console_engine/blob/master/examples/shapes.rs)
    /// - [tetris](https://github.com/VincentFoulon80/console_engine/blob/master/examples/tetris.rs)
    pub fn fill_rect(&mut self, start_x: i32, start_y: i32, end_x: i32, end_y: i32, character: Pixel)
    {
        let y0 = if start_y < end_y { start_y } else { end_y };
        let y1 = if start_y < end_y { end_y+1 } else { start_y+1 };
        for y in y0..y1 {
            self.h_line(start_x, y, end_x, character.clone());
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
    /// 
    /// examples :
    /// - [shapes](https://github.com/VincentFoulon80/console_engine/blob/master/examples/shapes.rs)
    pub fn circle(&mut self, x: i32, y: i32, radius: u32, character: Pixel)
    {
        let mut relative_pos_x = 0 as i32;
		let mut relative_pos_y = radius as i32;
		let mut distance: i32 = 3 - 2 * radius as i32;
		if radius == 0 {
            return;
        }

		while relative_pos_y >= relative_pos_x
		{
			self.set_pxl_ref(x + relative_pos_x, y - relative_pos_y, &character);
			self.set_pxl_ref(x + relative_pos_y, y - relative_pos_x, &character);
			self.set_pxl_ref(x + relative_pos_y, y + relative_pos_x, &character);
			self.set_pxl_ref(x + relative_pos_x, y + relative_pos_y, &character);
			self.set_pxl_ref(x - relative_pos_x, y + relative_pos_y, &character);
			self.set_pxl_ref(x - relative_pos_y, y + relative_pos_x, &character);
			self.set_pxl_ref(x - relative_pos_y, y - relative_pos_x, &character);
			self.set_pxl_ref(x - relative_pos_x, y - relative_pos_y, &character);
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
    /// 
    /// examples :
    /// - [screen-simple](https://github.com/VincentFoulon80/console_engine/blob/master/examples/screen-simple.rs)
    /// - [screen-swap](https://github.com/VincentFoulon80/console_engine/blob/master/examples/screen-swap.rs)
    /// - [shapes](https://github.com/VincentFoulon80/console_engine/blob/master/examples/shapes.rs)
    pub fn fill_circle(&mut self, x: i32, y: i32, radius: u32, character: Pixel)
    {
        // Taken from wikipedia
		let mut relative_pos_x = 0 as i32;
		let mut relative_pos_y = radius as i32;
		let mut distance: i32 = 3 - 2 * radius as i32;
		if radius == 0 {
            return;
        }

        // create a lambda function that draw fast horizontal lines
		let mut drawline = |start_x: i32, end_x: i32, y: i32|
		{
			for i in start_x..end_x+1 {
				self.set_pxl_ref(i, y, &character);
            }
		};

		while relative_pos_y >= relative_pos_x
		{
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
    /// 
    /// examples :
    /// - [shapes](https://github.com/VincentFoulon80/console_engine/blob/master/examples/shapes.rs)
    pub fn triangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, character: Pixel)
    {
        self.line(x1, y1, x2, y2, character.clone());
        self.line(x2, y2, x3, y3, character.clone());
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
    /// 
    /// examples :
    /// - [shapes](https://github.com/VincentFoulon80/console_engine/blob/master/examples/shapes.rs)
    /// - [screen-swap](https://github.com/VincentFoulon80/console_engine/blob/master/examples/screen-swap.rs)
    pub fn fill_triangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, character: Pixel)
    {
        self.triangle(x1 as i32, y1 as i32, x2 as i32, y2 as i32, x3 as i32, y3 as i32, character.clone());
        // we use tuples for this for now
        let v0 = (x1 as i32, y1 as i32);
        let mut v1 = (x2 as i32, y2 as i32);
        let mut v2 = (x3 as i32, y3 as i32);

        // algorithm only fills counter clockwise triangles, so swap as needed
        // For a triangle A B C, you can find the winding by computing the cross product (B - A) x (C - A). For 2d tri's, with z=0, it will only have a z component.
        // To give all the same winding, swap vertices C and B if this z component is negative.
        let cross = (v1.1 - v0.1) * (v2.0 - v1.0) - (v1.0 - v0.0) * (v2.1 - v1.1); 
        if cross > 0 { std::mem::swap(&mut v1, &mut v2) }
        
        // Compute triangle bounding box and clip to screen bounds
        let min_x = std::cmp::max(std::cmp::min(std::cmp::min(v0.0, v1.0), v2.0), 0);
        let max_x = std::cmp::min(std::cmp::max(std::cmp::max(v0.0, v1.0), v2.0), self.get_width() as i32 - 1);
        let min_y = std::cmp::max(std::cmp::min(std::cmp::min(v0.1, v1.1), v2.1), 0);
        let max_y = std::cmp::min(std::cmp::max(std::cmp::max(v0.1, v1.1), v2.1), self.get_height() as i32 - 1);

        // Triangle setup
        let a01 = v0.1 - v1.1;
        let b01 = v1.0 - v0.0;
        let a12 = v1.1 - v2.1;
        let b12 = v2.0 - v1.0;
        let a20 = v2.1 - v0.1;
        let b20 = v0.0 - v2.0;

        // Determine edges
        let is_top_left = |v0: (i32, i32), v1: (i32, i32)| -> bool {
            v0.1 > v1.1 
        };

        // We follow fill rules and add a bias
        let bias0 = if is_top_left(v1, v2) { 0 } else { -1 };
        let bias1 = if is_top_left(v2, v0) { 0 } else { -1 };
        let bias2 = if is_top_left(v0, v1) { 0 } else { -1 };

        // Determine barycentric coordinates
        let orient2d = |a: (i32,i32), b: (i32,i32), c: (i32,i32)| -> i32 {
            (b.0-a.0)*(c.1-a.1) - (b.1-a.1)*(c.0-a.0)
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
                        self.set_pxl_ref(p.0, p.1, &character);
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


    /// Referenced version of set_pxl  
    /// see set_pxl for more information on this usage
    /// 
    /// The only differences between the two is that this version takes the Pixel as a reference
    fn set_pxl_ref(&mut self, x: i32, y: i32, character: &Pixel)
    {
        if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
            let index = self.coord_to_index(x, y);
            self.screen[index] = character.clone();
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
    /// 
    /// examples :
    /// - [graph](https://github.com/VincentFoulon80/console_engine/blob/master/examples/graph.rs)
    /// - [mouse](https://github.com/VincentFoulon80/console_engine/blob/master/examples/mouse.rs)
    /// - [shapes](https://github.com/VincentFoulon80/console_engine/blob/master/examples/shapes.rs)
    /// - [snake](https://github.com/VincentFoulon80/console_engine/blob/master/examples/snake.rs)
    /// - [tetris](https://github.com/VincentFoulon80/console_engine/blob/master/examples/tetris.rs)
    pub fn set_pxl(&mut self, x: i32, y: i32, character: Pixel)
    {
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
    /// 
    /// examples :
    /// - [tetris](https://github.com/VincentFoulon80/console_engine/blob/master/examples/tetris.rs)
    pub fn get_pxl(&self, x: i32, y: i32) -> Result<Pixel, String> 
    {
        if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
            return Ok(self.screen[self.coord_to_index(x, y)].clone());
        }
        Err(format!("Attempted to get_pxl out of bounds (coords: [{}, {}], bounds: [{}, {}]", x,y,self.width-1,self.height-1))
    }

    /// Resizes the screen to match the given width and height
    /// truncates the bottom and right side of the screen
    /// 
    /// usage:
    /// ```
    /// screen.resize()
    /// ```
    /// 
    /// examples :
    /// - *no examples*
    pub fn resize(&mut self, new_width: u32, new_height: u32)
    {
        // create new screens Vec
        let mut new_screen = vec![pixel::pxl(' '); (new_width*new_height) as usize];
        // transfer old screens into new screens
        for j in 0..std::cmp::min(self.height, new_height) {
            for i in 0..std::cmp::min(self.width, new_width) {
                if (i as u32) < self.width && (j as u32) < self.height {
                    new_screen[((j*new_width)+i) as usize] = self.screen[((j*self.width)+i) as usize].clone();
                }
            }
        }
        self.screen = new_screen;
        self.width = new_width;
        self.height = new_height;
    }

    /// Converts x and y coordinates to screen index
    /// 
    /// example : on a 10x10 screen
    /// `coord_to_index(2,1)` will return index 12
    fn coord_to_index(&self, x: i32, y: i32) -> usize
    {
        ((y*self.width as i32) + x) as usize
    }
}

impl ToString for Screen {
    fn to_string(&self) -> String
    {
        let mut output = String::new();
        for i in 0..self.width*self.height {
            output.push_str(self.screen[i as usize].to_string().as_str());
            if i != self.width*self.height-1 && i % self.width == self.width-1 {
                output.push_str("\r\n");
            }
        }
        output
    }
}