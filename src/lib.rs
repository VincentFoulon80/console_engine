pub extern crate termion;

mod utils;
pub mod pixel;

use pixel::Pixel;
use termion::color;
use termion::event::{Event, Key};
use std::io::{stdout, Stdout};
use termion::raw::IntoRawMode;
use std::io::Write;
use termion::input::{TermRead, MouseTerminal};

/// # Console Engine Framework
pub struct ConsoleEngine {
    input: termion::input::Events<termion::AsyncReader>,
    output: MouseTerminal<termion::raw::RawTerminal<Stdout>>,
    time_limit: u128,
    /// The current frame count, publicly accessible
    pub frame_count: usize,
    width: u32,
    height: u32,
    screen: Vec<Pixel>,
    screen_last_frame: Vec<Pixel>,
    instant: std::time::Instant,
    keys_pressed: Vec<Event>,
    keys_held: Vec<Event>,
    keys_released: Vec<Event>,
}
/// # Basic Usage :
/// 
/// ```
/// use console_engine::pixel;
/// use console_engine::termion::color;
/// use console_engine::termion::event::Key;
/// 
/// fn main() {
///     // initializes a screen of 20x10 characters with a target of 3 frame per second
///     // coordinates will range from [0,0] to [19,9]
///     let mut engine = console_engine::ConsoleEngine::init(20, 10, 3);
///     let value = 14;
///     // main loop, be aware that you'll have to break it because ctrl+C is captured
///     loop {
///         engine.wait_frame(); // wait for next frame + capture inputs
///         engine.clear_screen(); // reset the screen
///     
///         engine.line(0, 0, 19, 9, pixel::pxl('#')); // draw a line of '#' from [0,0] to [19,9]
///         engine.print(0, 4, format!("Result: {}", value)); // prints some value at [0,4]
///     
///         engine.set_pxl(4, 0, pixel::pxl_fg('O', color::Cyan)); // write a majestic cyan 'O' at [4,0]
/// 
///         if engine.is_key_pressed(Key::Char('q')) { // if the user presses 'q' :
///             break; // exits app
///         }
///     
///         engine.draw(); // draw the screen
///     }
/// }
/// ```
impl ConsoleEngine {
    /// Initialize a screen of the provided width and height, and load the target FPS
    pub fn init(width: u32, height: u32, target_fps: u32) ->  ConsoleEngine {
        assert!(target_fps > 0, "Target FPS needs to be greater than zero.");
        let size = termion::terminal_size().unwrap();
        assert!(size.0 as u32 >= width && size.1 as u32 >= height, "Your terminal must have at least a width and height of {}x{} characters. Currently has {}x{}", width, height, size.0, size.1);
        let mut my = ConsoleEngine {
            output: MouseTerminal::from(stdout().into_raw_mode().unwrap()),
            input: termion::async_stdin().events(),
            time_limit: (1000/target_fps) as u128,
            frame_count: 0,
            width: width,
            height: height,
            screen: vec![pixel::pxl(' '); (width*height) as usize],
            screen_last_frame: vec![],
            instant: std::time::Instant::now(),
            keys_pressed: vec!(),
            keys_held: vec!(),
            keys_released: vec!(),
        };
        my.begin();
        return my;
    }
    
    /// Initialize a screen filling the entire terminal with the target FPS
    pub fn init_fill(target_fps: u32) -> ConsoleEngine {
        let size = termion::terminal_size().unwrap();
        return ConsoleEngine::init(size.0 as u32, size.1 as u32, target_fps);
    }

    /// Initialize a screen filling the entire terminal with the target FPS  
    /// Also check the terminal width and height and assert if the terminal has at least the asked size
    pub fn init_fill_require(width: u32, height: u32, target_fps: u32) -> ConsoleEngine {
        let size = termion::terminal_size().unwrap();
        assert!(size.0 as u32 >= width && size.1 as u32 >= height, "Your terminal must have at least a width and height of {}x{} characters. Currently has {}x{}", width, height, size.0, size.1);
        return ConsoleEngine::init_fill(target_fps);
    }

    #[cfg(windows)]
    /// Initializes the internal components such as input system
    fn begin(&mut self) {
        println!("Please Press Enter to initialize inputs");
        while !self.input.next().is_some() {}
        println!("{}{}{}", termion::cursor::Hide, termion::clear::All, termion::cursor::Goto(1,1));
    }
    #[cfg(not(windows))]
    /// Initializes the internal components such as hiding the cursor
    fn begin(&mut self) {
        println!("{}{}{}", termion::cursor::Hide, termion::clear::All, termion::cursor::Goto(1,1));
    }

    /// Gracefully stop the engine, and set back a visible cursor
    fn end(&mut self){
        println!("{}{}{}\r\n", termion::cursor::Show, color::Fg(color::Reset), color::Bg(color::Reset));
    }

    /// Get the screen width
    pub fn scr_w(&self) -> u32
    {
        self.width
    }
    /// Get the screen height
    pub fn scr_h(&self) -> u32
    {
        self.height
    }

    /// Reset the screen to a blank state
    pub fn clear_screen(&mut self) 
    {
        self.screen = vec![pixel::pxl(' '); (self.width*self.height) as usize];
    }

    /// prints a string at the specified coordinates.  
    /// The string will be cropped if it reach the right border
    /// 
    /// usage:
    /// ```
    /// engine.print(0,0, String::from("Hello, world!"));
    /// engine.print(0, 4, format!("Score: {}", score));
    /// ```
    pub fn print(&mut self, x: i32, y: i32, string: String)
    {
        if y >= 0 && x < self.width as i32 && y < self.height as i32 {
            // get screen index, initializes a counter 
            // and get chars of the provided String
            let pos = self.coord_to_index(std::cmp::max(0,x), y);
            let mut delta_x = 0usize;
            if x < 0 {
                delta_x = x.abs() as usize;
            }
            let mut count = delta_x;
            let char_vec: Vec<char> = string.chars().collect();
            let origin_row = pos/self.scr_w() as usize;
            // place each characters one by one. Stops before overflowing
            for i in pos..std::cmp::min(pos+char_vec.len()-delta_x, self.screen.capacity()) {
                // if the row changes, break. 
                // removing this statement will cause a wrapping of the text
                if origin_row != i/self.scr_w() as usize {
                    break;
                }
                // print the character on screen
                self.screen[i] = pixel::pxl(char_vec[count]);
                count += 1;
            }
        }
    }

    /// prints a string at the specified coordinates with the specified foreground and background color  
    /// The string will automatically overlaps if it reach the right border
    /// 
    /// usage
    /// ```
    /// // print "Hello, world" in blue on white background
    /// engine.print(0,0, String::from("Hello, world!"), color::Blue, color::White);
    /// ```
    pub fn print_fbg<C1: color::Color + Clone, C2: color::Color + Clone>(&mut self, x: i32, y: i32, string: String, fg: C1, bg: C2)
    {
        if y >= 0 && x < self.width as i32 && y < self.height as i32 {
            // get screen index, initializes a counter 
            // and get chars of the provided String
            let pos = self.coord_to_index(std::cmp::max(0,x), y);
            let mut delta_x = 0usize;
            if x < 0 {
                delta_x = x.abs() as usize;
            }
            let mut count = delta_x;
            let char_vec: Vec<char> = string.chars().collect();
            let origin_row = pos/self.scr_w() as usize;
            // place each characters one by one. Stops before overflowing
            for i in pos..std::cmp::min(pos+char_vec.len()-delta_x, self.screen.capacity()) {
                // if the row changes, break. 
                // removing this statement will cause a wrapping of the text
                if origin_row != i/self.scr_w() as usize {
                    break;
                }
                // print the character on screen
                self.screen[i] = pixel::pxl_fbg(char_vec[count], fg.clone(), bg.clone());
                count += 1;
            }
        }
    }

    /// draws a line of the provided character between two sets of coordinates  
    /// see: [Bresenham's line algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm)
    /// 
    /// Note : Your line can start or end out of bounds. These pixels won't be drawn 
    /// 
    /// usage : 
    /// ```
    /// use console_engine::pixel;
    /// // ...
    /// engine.line(0, 0, 9, 9, pixel::pxl('#'));
    /// ```
    pub fn line(&mut self, start_x: i32, start_y: i32, end_x: i32, end_y: i32, character: Pixel)
    {
        let delta_x = end_x - start_x;
        let delta_y = end_y - start_y;
        // optimized algorithms for pure horizontal or vertical lines
        if delta_y == 0 {
            let mut start = start_x;
            let mut end = end_x+1;
            if end_x < start_x {
                end = start_x+1;
                start = end_x;
            };
            // horizontal line
            for i in start..end {
                self.set_pxl_ref(i, start_y, &character);
            }
            return;
        }
        if delta_x == 0 {
            let mut start = start_y;
            let mut end = end_y+1;
            if end_y < start_y {
                end = start_y+1;
                start = end_y;
            };
            // horizontal line
            for j in start..end {
                self.set_pxl_ref(start_x, j, &character);
            }
            return;
        }

        // Bresenham's line algorithm
        let line_low = |engine: &mut ConsoleEngine, x0: i32,y0: i32, x1: i32,y1: i32| {
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
                engine.set_pxl_ref(x, y, &character);
                if d > 0 {
                    y = y + yi;
                    d = d - 2*dx;
                }
                d = d + 2*dy;
            } 
        };

        let line_high = |engine: &mut ConsoleEngine, x0: i32,y0: i32, x1: i32,y1: i32| {
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
                engine.set_pxl_ref(x, y, &character);
                if d > 0 {
                    x = x + xi;
                    d = d - 2*dy;
                }
                d = d + 2*dx;
            }   
        };

        if (end_y - start_y).abs() < (end_x - start_x).abs() {
            if start_x > end_x {
                line_low(self, end_x, end_y, start_x, start_y);
            } else {
                line_low(self, start_x, start_y, end_x, end_y);
            }
        } else {
            if start_y > end_y {
                line_high(self, end_x, end_y, start_x, start_y);
            } else {
                line_high(self, start_x, start_y, end_x, end_y);
            }
        }
    }

    /// Draws a rectangle of the provided character between two sets of coordinates  
    /// 
    /// usage : 
    /// ```
    /// use console_engine::pixel;
    /// // ...
    /// engine.rect(0, 0, 9, 9, pixel::pxl('#'));
    /// ```
    pub fn rect(&mut self, start_x: i32, start_y: i32, end_x: i32, end_y: i32, character: Pixel)
    {
        self.line(start_x, start_y, end_x, start_y, character.clone()); // top
        self.line(end_x, start_y, end_x, end_y, character.clone());     // right
        self.line(end_x, end_y, start_x, end_y, character.clone());     // bottom
        self.line(start_x, end_y, start_x, start_y, character.clone()); // left
    }

    /// Fill a rectangle of the provided character between two sets of coordinates  
    /// 
    /// usage : 
    /// ```
    /// use console_engine::pixel;
    /// // ...
    /// engine.fill_rect(0, 0, 9, 9, pixel::pxl('#'));
    /// ```
    pub fn fill_rect(&mut self, start_x: i32, start_y: i32, end_x: i32, end_y: i32, character: Pixel)
    {
        let y0 = if start_y < end_y { start_y } else { end_y };
        let y1 = if start_y < end_y { end_y+1 } else { start_y+1 };
        for y in y0..y1 {
            self.line(start_x, y, end_x, y, character.clone());
        }
    }

    /// Draws a circle of the provided character at an x and y position with a radius
    /// see: [olcPixelGameEngine Repository](https://github.com/OneLoneCoder/olcPixelGameEngine)
    /// 
    /// usage : 
    /// ```
    /// use console_engine::pixel;
    /// // ...
    /// engine.circle(10, 10, 4, pixel::pxl('#'));
    /// ```
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
    /// usage : 
    /// ```
    /// use console_engine::pixel;
    /// // ...
    /// engine.circle(10, 10, 4, pixel::pxl('#'));
    /// ```
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
    /// usage : 
    /// ```
    /// use console_engine::pixel;
    /// // ...
    /// engine.triangle(8,8, 4,6, 9,2, pixel::pxl('#'));
    /// ```
    pub fn triangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, character: Pixel)
    {
        self.line(x1, y1, x2, y2, character.clone());
        self.line(x2, y2, x3, y3, character.clone());
        self.line(x3, y3, x1, y1, character.clone());
    }

    /// Fill a triangle of the provided character using three sets of coordinates
    /// see: [rustyPixelGameEngine Repository](https://github.com/mattbettcher/rustyPixelGameEngine)
    /// 
    /// usage : 
    /// ```
    /// use console_engine::pixel;
    /// // ...
    /// engine.fill_triangle(8,8, 4,6, 9,2, pixel::pxl('#'));
    /// ```
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
        let max_x = std::cmp::min(std::cmp::max(std::cmp::max(v0.0, v1.0), v2.0), self.scr_w() as i32 - 1);
        let min_y = std::cmp::max(std::cmp::min(std::cmp::min(v0.1, v1.1), v2.1), 0);
        let max_y = std::cmp::min(std::cmp::max(std::cmp::max(v0.1, v1.1), v2.1), self.scr_h() as i32 - 1);

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
    /// engine.set_pxl(3,8,pixel::pixel('o'));
    /// ```
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
    /// if engine.get_pxl(3,8).unwrap().chr == 'o' {
    ///     engine.print(0,0,"Found a 'o'");
    /// }
    /// ```
    pub fn get_pxl(&self, x: i32, y: i32) -> Result<Pixel, String> 
    {
        if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
            return Ok(self.screen[self.coord_to_index(x, y)].clone());
        }
        Err(format!("Attempted to get_pxl out of bounds (coords: [{}, {}], bounds: [{}, {}]", x,y,self.width-1,self.height-1))
    }
    
    /// Draw the screen in the terminal  
    /// For best results, use it once per frame
    /// 
    /// usage:
    /// ```
    /// engine.print(0,0,String::from("Hello, world!")); // <- prints "Hello, world!" in 'screen' memory
    /// engine.draw(); // display 'screen' memory to the user's terminal
    /// ```
    pub fn draw(&mut self)
    {
        let mut out = self.output.lock();
        // reset cursor position
        write!(out, "{}", termion::cursor::Goto(1,1)).unwrap();
        let mut current_colors = String::from("");
        let mut moving = false;
        // iterates through the screen memory and prints it on the output buffer
        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.coord_to_index(x as i32, y as i32);
                let pixel = &self.screen[index];
                // we check if the screen has been modified at this coordinate
                // if so, we write like normally, else we set a 'moving' flag
                if self.screen_last_frame.is_empty() || *pixel != self.screen_last_frame[index] {
                    if moving {
                        // if the moving flag is set, we need to write a goto instruction first
                        // this optimization minimize useless write on the screen
                        // actually writing to the screen is very slow so it's a good compromise
                        write!(out, "{}", termion::cursor::Goto(1+x as u16,1+y as u16)).unwrap();
                        moving = false;
                    }
                    // we check if the last color is the same as the current one.
                    // if the color is the same, only print the character
                    // the less we write on the output the faster we'll get
                    // and additional characters for colors we already have set is
                    // time consuming
                    if current_colors != pixel.colors {
                        current_colors = pixel.colors.clone();
                        write!(out, "{}", pixel).unwrap();
                    } else {
                        write!(out, "{}", pixel.chr).unwrap();
                    }
                } else {
                    moving = true
                }
            }
            if y < self.height-1 {
                write!(out, "\r\n").unwrap();
            }
        }
        // flush the buffer into user's terminal
        out.flush().unwrap();
        self.screen_last_frame = self.screen.clone();
    }

    /// Pause the execution until the next frame need to be rendered  
    /// Internally gets user's input for the next frame
    /// 
    /// usage:
    /// ```
    /// // initializes a screen with a 10x10 screen and targetting 30 fps
    /// let mut engine = console_engine::ConsoleEngine::init(10, 10, 30);
    /// loop {
    ///     engine.wait_frame(); // wait for next frame
    ///     // do your stuff
    /// }
    /// ```
    pub fn wait_frame(&mut self) {
        let mut pressed: Vec<Event> = vec!();

        // if there is time before next frame, sleep until next frame
        if self.time_limit > self.instant.elapsed().as_millis() {
            std::thread::sleep(std::time::Duration::from_millis(((self.time_limit - self.instant.elapsed().as_millis()) % self.time_limit) as u64));
        }
        self.instant = std::time::Instant::now();
        
        self.frame_count += 1;

        // captures user's input
        let mut c = self.input.next();
        let mut count = 0;
        while c.is_some() && count < 10 { // cannot support for more than 10 key presses at the same time
            pressed.push(c.unwrap().unwrap()); 
            c = self.input.next();
            count += 1
        }
        // updates pressed / held / released states
        let held = utils::intersect(&utils::union(&self.keys_pressed,&self.keys_held), &pressed);
        self.keys_released = utils::outersect_left(&self.keys_held, &held);
        self.keys_pressed = utils::outersect_left(&pressed, &held);
        self.keys_held = utils::union(&held, &self.keys_pressed);

    }

    /// checks whenever a key is pressed (first frame held only)
    /// 
    /// usage:
    /// ```
    /// loop {
    ///     engine.wait_frame(); // wait for next frame + captures input
    ///     
    ///     if engine.is_key_pressed(Key::Char('q')) {
    ///         break; // exits app
    ///     }
    /// }
    /// ```
    pub fn is_key_pressed(&self, key: Key) -> bool
    {
        self.keys_pressed.contains(&Event::Key(key))
    }

    /// checks whenever a key is held down
    /// 
    /// usage:
    /// ```
    /// loop {
    ///     engine.wait_frame(); // wait for next frame + captures input
    ///     
    ///     if engine.is_key_held(Key::Char('8')) && pos_y > 0 {
    ///         pos_y -= 1; // move position upward
    ///     }
    /// }
    /// ```
    pub fn is_key_held(&self, key: Key) -> bool
    {
        self.keys_held.contains(&Event::Key(key))
    }

    /// checks whenever a key has been released (first frame released)
    ///  
    /// usage:
    /// ```
    /// if engine.is_key_held(Key::Char('h')) {
    ///     engine.clear_screen();
    ///     engine.print(0,0,"Please don't hold this button.");
    ///     engine.draw();
    ///     while !engine.is_key_released(Key::Char('h')) {
    ///         engine.wait_frame(); // refresh button's states
    ///     }
    /// }
    /// ```
    pub fn is_key_released(&self, key: Key) -> bool
    {
        self.keys_released.contains(&Event::Key(key))
    }

    /// Give the mouse's terminal coordinates if the provided button has been pressed
    /// 
    /// usage :
    /// ```
    /// // prints a 'P' where the mouse's left button has been pressed
    /// let mouse_pos = engine.get_mouse_press(termion::event::MouseButton::Left);
    /// if mouse_pos.is_some() {
    ///    let mouse_pos = mouse_pos.unwrap();
    ///    engine.set_pxl(mouse_pos.0 as i32, mouse_pos.1 as i32, pixel::pxl('P'));
    /// }
    /// ```
    pub fn get_mouse_press(&self, button: termion::event::MouseButton) -> Option<(u32,u32)> 
    {
        for evt in self.keys_pressed.iter() {
            match evt {
                Event::Mouse(me) => {
                    match me {
                        termion::event::MouseEvent::Press(mouse, x, y) => {
                            if *mouse == button {
                                return Some((x.clone() as u32 -1, y.clone() as u32 -1));
                            }
                        },
                        _ => {}
                    }
                }
                _ => {}
            };
        }
        return None;
    }

    /// Give the mouse's terminal coordinates if a button is held on the mouse
    /// 
    /// usage :
    /// ```
    /// // prints a 'H' where the mouse is currently held
    /// let mouse_pos = engine.get_mouse_held();
    /// if mouse_pos.is_some() {
    ///     let mouse_pos = mouse_pos.unwrap();
    ///     engine.set_pxl(mouse_pos.0 as i32, mouse_pos.1 as i32, pixel::pxl('H'));
    /// }
    /// ```
    pub fn get_mouse_held(&self) -> Option<(u32,u32)> 
    {
        for evt in self.keys_pressed.iter() {
            match evt {
                Event::Mouse(me) => {
                    match me {
                        termion::event::MouseEvent::Hold(x, y) => {
                            return Some((x.clone() as u32 -1, y.clone() as u32 -1));
                        },
                        _ => {}
                    }
                }
                _ => {}
            };
        }
        return None;
    }

    /// Give the mouse's terminal coordinates if a button has been released on the mouse
    /// 
    /// usage :
    /// ```
    /// // prints a 'R' where the mouse has been released
    /// let mouse_pos = engine.get_mouse_held();
    /// if mouse_pos.is_some() {
    ///     let mouse_pos = mouse_pos.unwrap();
    ///     engine.set_pxl(mouse_pos.0 as i32, mouse_pos.1 as i32, pixel::pxl('H'));
    /// }
    /// ```
    pub fn get_mouse_released(&self) -> Option<(u32,u32)> 
    {
        for evt in self.keys_pressed.iter() {
            match evt {
                Event::Mouse(me) => {
                    match me {
                        termion::event::MouseEvent::Release(x, y) => {
                            return Some((x.clone() as u32 -1, y.clone() as u32 -1));
                        },
                        _ => {}
                    }
                }
                _ => {}
            };
        }
        return None;
    }

    /// prints key status on screen. For debug purposes only.
    #[allow(dead_code)]
    pub fn debug_keys(&self)
    {
        println!("pressed: {:?}\r\nheld: {:?}\r\nreleased: {:?}", self.keys_pressed, self.keys_held, self.keys_released);
    }

    /// Converts x and y coordinates to screen index
    /// 
    /// example : on a 10x10 screen
    /// `coord_to_index(2,1)` will return index 12
    fn coord_to_index(&self, x: i32, y: i32) -> usize
    {
        return ((y*self.width as i32) + x) as usize;
    }
}

impl Drop for ConsoleEngine {
    fn drop(&mut self) {
        self.end();
    }
}