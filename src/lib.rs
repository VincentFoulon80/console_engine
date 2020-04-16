pub extern crate termion;

mod utils;
pub mod pixel;

use pixel::Pixel;
use termion::color;
use termion::event::Key;
use std::io::{stdout, Stdout};
use termion::raw::IntoRawMode;
use std::io::Write;
use crate::termion::input::TermRead;

/// # Console Engine Framework
pub struct ConsoleEngine {
    input: termion::input::Keys<termion::AsyncReader>,
    output: termion::raw::RawTerminal<Stdout>,
    time_limit: u128,
    width: u32,
    height: u32,
    screen: Vec<Pixel>,
    instant: std::time::Instant,
    keys_pressed: Vec<Key>,
    keys_held: Vec<Key>,
    keys_released: Vec<Key>,
    // device: DeviceState
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
///         engine.line(0, 0, 19, 9, pixel::pxl('#')); // draw a line of '#' from [0,0] to [9,9]
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
        let mut my = ConsoleEngine {
            output: stdout().into_raw_mode().unwrap(),
            input: termion::async_stdin().keys(),
            time_limit: (1000/target_fps) as u128,
            width: width,
            height: height,
            screen: vec![pixel::pxl(' '); (width*height) as usize],
            instant: std::time::Instant::now(),
            keys_pressed: vec!(),
            keys_held: vec!(),
            keys_released: vec!(),
            // device: DeviceState::new()
        };
        my.begin();
        return my;
    }
    
    /// Initialize a screen filling the entire terminal with the target FPS
    pub fn init_fill(target_fps: u32) -> ConsoleEngine {
        let size = termion::terminal_size().unwrap();
        let mut my = ConsoleEngine {
            output: stdout().into_raw_mode().unwrap(),
            input: termion::async_stdin().keys(),
            time_limit: (1000/target_fps) as u128,
            width: size.0 as u32,
            height: size.1 as u32,
            screen: vec![pixel::pxl(' '); (size.0*size.1) as usize],
            instant: std::time::Instant::now(),
            keys_pressed: vec!(),
            keys_held: vec!(),
            keys_released: vec!(),
            // device: DeviceState::new()
        };
        my.begin();
        return my;
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
    /// The string will automatically overlaps if it reach the right border
    /// 
    /// usage:
    /// ```
    /// engine.print(0,0,"Hello, world!");
    /// engine.print(0, 4, format!("Score: {}", score));
    /// ```
    pub fn print(&mut self, x: u32, y: u32, string: String)
    {
        if x < self.width && y < self.height {
            let pos = self.coord_to_index(x, y);
            let mut count = 0usize;
            let char_vec: Vec<char> = string.chars().collect();
            for i in pos..std::cmp::min(pos+char_vec.len(), self.screen.capacity()) {
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
    /// engine.print(0,0,"Hello, world!", color::Blue, color::White);
    /// ```
    pub fn print_fbg<C: color::Color + Clone>(&mut self, x: u32, y: u32, string: String, fg: C, bg: C)
    {
        if x < self.width && y < self.height {
            let pos = self.coord_to_index(x, y);
            let mut count = 0usize;
            let char_vec: Vec<char> = string.chars().collect();
            for i in pos..std::cmp::min(pos+char_vec.len(), self.screen.capacity()) {
                self.screen[i] = pixel::pxl_fbg(char_vec[count], fg.clone(), bg.clone());
                count += 1;
            }
        }
    }

    /// draws a line of the provided character between two sets of coordinates  
    /// this code is heavily inspired by the drawLine function of olc::PixelGameEngine  
    /// see: [olcPixelGameEngine Repository](https://github.com/OneLoneCoder/olcPixelGameEngine)
    /// 
    /// usage : 
    /// ```
    /// use console_engine::pixel;
    /// // ...
    /// engine.line(0, 0, 9, 9, pixel::pxl('#'));
    /// ```
    pub fn line(&mut self, start_x: u32, start_y: u32, end_x: u32, end_y: u32, character: Pixel)
    {
        let delta_x = end_x as i64 - start_x as i64;
        let delta_y = end_y as i64 - start_y as i64;
        if delta_y == 0 {
            // horizontal line
            for i in start_x..end_x {
                self.set_pxl_ref(i, start_y, &character);
            }
            return;
        }
        if delta_x == 0 {
            // horizontal line
            for j in start_y..end_y {
                self.set_pxl_ref(start_x, j, &character);
            }
            return;
        }
        let delta_abs_x = delta_x.abs();
        let delta_abs_y = delta_y.abs();
        let mut pos_x = 2 * delta_abs_y - delta_abs_x;
        let	mut pos_y = 2 * delta_abs_x - delta_abs_y;
        let mut x: i32; 
        let mut y: i32; 
        if delta_abs_y <= delta_abs_x {
            let x_end: i32;
            if delta_x >= 0
			    { x = start_x as i32; y = start_y as i32; x_end = end_x as i32; }
			else
                { x = end_x as i32; y = end_y as i32; x_end = start_x as i32; }

            self.set_pxl_ref(x as u32, y as u32, &character);
            for x in x..x_end {
				
				if pos_x<0 {
					pos_x = pos_x + 2 * delta_abs_y;
                } else {
					if (delta_x<0 && delta_y<0) || (delta_x>0 && delta_y>0) {y = y + 1;} else {y = y - 1;}
					pos_x = pos_x + 2 * (delta_abs_y - delta_abs_x);
                }
                self.set_pxl_ref(x as u32, y as u32, &character);
			}
        } else { 
            let y_end: i32;
            if delta_x >= 0
			    { x = start_x as i32; y = start_y as i32; y_end = end_y as i32; }
			else
                { x = end_x as i32; y = end_y as i32; y_end = start_y as i32; }

            self.set_pxl_ref(x as u32, y as u32, &character);
            for y in y..y_end {
				if pos_y<0 {
					pos_y = pos_y + 2 * delta_abs_x;
                } else {
					if (delta_x<0 && delta_y<0) || (delta_x>0 && delta_y>0) {x = x + 1;} else {x = x - 1};
					pos_y = pos_y + 2 * (delta_abs_x - delta_abs_y);
                }
                self.set_pxl_ref(x as u32, y as u32, &character);
			}
        }
        
        self.set_pxl_ref(end_x, end_y, &character);
    }

    /// Referenced version of set_pxl  
    /// see set_pxl for more information on this usage
    /// 
    /// The only differences between the two is that this version takes the Pixel as a reference
    fn set_pxl_ref(&mut self, x: u32, y: u32, character: &Pixel)
    {
        if x < self.width && y < self.height {
            let index = self.coord_to_index(x, y);
            self.screen[index] = character.clone();
        } else {
            panic!("Attempted to set pxl out of bounds (coords: [{}, {}], bounds: [{}, {}]", x,y,self.width-1,self.height-1);
        }
    }

    /// sets the provided character in the specified coordinates
    /// 
    /// usage: 
    /// ```
    /// use console_engine::pixel;
    /// // ...
    /// engine.set_pxl(3,8,pixel::pixel('o'));
    /// ```
    pub fn set_pxl(&mut self, x: u32, y: u32, character: Pixel)
    {
        if x < self.width && y < self.height {
            let index = self.coord_to_index(x, y);
            self.screen[index] = character;
        } else {
            panic!("Attempted to set pxl out of bounds (coords: [{}, {}], bounds: [{}, {}]", x,y,self.width-1,self.height-1);
        }
    }

    /// Get the character stored at provided coordinates
    /// 
    /// usage:
    /// ```
    /// if engine.get_pxl(3,8).chr == 'o' {
    ///     engine.print(0,0,"Found a 'o'");
    /// }
    /// ```
    pub fn get_pxl(&self, x: u32, y: u32) -> Pixel 
    {
        assert!(x < self.width && y < self.height, "provided coordinates are out of screen's bounds");
        return self.screen[self.coord_to_index(x, y)].clone();
    }
    
    /// Draw the screen in the terminal  
    /// For best results, use it once per frame
    /// 
    /// usage:
    /// ```
    /// engine.print(0,0,"Hello, world!"); // <- prints "Hello, world!" in 'screen' memory
    /// engine.draw(); // display 'screen' memory to the user's terminal
    /// ```
    pub fn draw(&self)
    {
        let mut out = self.output.lock();
        write!(out, "{}", termion::cursor::Goto(1,1)).unwrap();
        for y in 0..self.height {
            for x in 0..self.width {
                write!(out, "{}", self.screen[self.coord_to_index(x, y)]).unwrap();
            }
            if y < self.height-1 {
                write!(out, "\r\n").unwrap();
            }
        }
        out.flush().unwrap();
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
        let mut pressed: Vec<Key> = vec!();

        if self.time_limit > self.instant.elapsed().as_millis() {
            std::thread::sleep(std::time::Duration::from_millis(((self.time_limit - self.instant.elapsed().as_millis()) % self.time_limit) as u64));
        }

        self.instant = std::time::Instant::now();
        
        let mut c = self.input.next();
        let mut count = 0;
        while c.is_some() && count < 10 { // cannot support for more than 10 key presses at the same time
            pressed.push(c.unwrap().unwrap()); 
            c = self.input.next();
            count += 1
        }

        //pressed = utils::union(&pressed, &self.device.get_keys());
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
        self.keys_pressed.contains(&key)
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
        self.keys_held.contains(&key)
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
        self.keys_released.contains(&key)
    }

    /// prints key status on screen. For debug purposes only.
    #[allow(dead_code)]
    pub fn debug_keys(&self)
    {
        println!("pressed: {:?}\nheld: {:?}\nreleased: {:?}", self.keys_pressed, self.keys_held, self.keys_released);
    }

    /// Converts x and y coordinates to screen index
    /// 
    /// example : on a 10x10 screen
    /// `coord_to_index(2,1)` will return index 12
    fn coord_to_index(&self, x: u32, y: u32) -> usize
    {
        return ((y*self.width) + x) as usize;
    }
}

impl Drop for ConsoleEngine {
    fn drop(&mut self) {
        self.end();
    }
}