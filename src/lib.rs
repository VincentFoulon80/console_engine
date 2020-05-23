//! This library provides simple features for handling user's input and display for terminal applications.
//!
//! Besides the user input and display, this library also provides some tools to build standalone "screens" that can be used as simply as printing it.
//!
//! It uses [Crossterm](https://crates.io/crates/crossterm) as main tool for handling the screen and inputs. You don't have to worry about initalizing anything because the lib will handle this for you.

pub extern crate crossterm;

pub mod pixel;
pub mod screen;
mod utils;

use crossterm::event::{self, Event, KeyEvent, MouseEvent};
pub use crossterm::event::{KeyCode, KeyModifiers, MouseButton};
pub use crossterm::style::Color;
use crossterm::terminal::{self, ClearType};
use crossterm::{execute, queue, style};
use pixel::Pixel;
use screen::Screen;
use std::io::Write;
use std::io::{stdout, Stdout};

/// Console Engine Framework
///
/// # Features
///
/// *note : each link will redirect you to a bunch of functions*
///
/// - Build custom terminal display using [shapes](#method.line) or [text](#method.print)
/// - Terminal handling with a [target frame per seconds](#method.init)
/// - [Keyboard](#method.is_key_pressed) and [mouse](#method.get_mouse_press) support
/// - [Terminal resizing](#method.check_resize) support
///
/// # Basic Usage:
///
/// ```
/// use console_engine::pixel;
/// use console_engine::Color;
/// use console_engine::KeyCode;
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
///         engine.print(0, 4, format!("Result: {}", value).as_str()); // prints some value at [0,4]
///     
///         engine.set_pxl(4, 0, pixel::pxl_fg('O', Color::Cyan)); // write a majestic cyan 'O' at [4,0]
///
///         if engine.is_key_pressed(KeyCode::Char('q')) { // if the user presses 'q' :
///             break; // exits app
///         }
///     
///         engine.draw(); // draw the screen
///     }
/// }
/// ```
///
/// #
///
pub struct ConsoleEngine {
    stdout: Stdout,
    time_limit: std::time::Duration,
    /// The current frame count, publicly accessible
    pub frame_count: usize,
    width: u32,
    height: u32,
    screen: Screen,
    screen_last_frame: Screen,
    instant: std::time::Instant,
    keys_pressed: Vec<KeyEvent>,
    keys_held: Vec<KeyEvent>,
    keys_released: Vec<KeyEvent>,
    mouse_events: Vec<MouseEvent>,
}

impl ConsoleEngine {
    /// Initialize a screen of the provided width and height, and load the target FPS
    pub fn init(width: u32, height: u32, target_fps: u32) -> ConsoleEngine {
        assert!(target_fps > 0, "Target FPS needs to be greater than zero.");
        let size = crossterm::terminal::size().unwrap();
        assert!(size.0 as u32 >= width && size.1 as u32 >= height, "Your terminal must have at least a width and height of {}x{} characters. Currently has {}x{}", width, height, size.0, size.1);
        let mut my = ConsoleEngine {
            stdout: stdout(),
            time_limit: std::time::Duration::from_millis(1000 / target_fps as u64),
            frame_count: 0,
            width,
            height,
            screen: Screen::new(width, height),
            screen_last_frame: Screen::new_empty(width, height),
            instant: std::time::Instant::now(),
            keys_pressed: vec![],
            keys_held: vec![],
            keys_released: vec![],
            mouse_events: vec![],
        };
        my.begin();
        my
    }

    /// Initialize a screen filling the entire terminal with the target FPS
    pub fn init_fill(target_fps: u32) -> ConsoleEngine {
        let size = crossterm::terminal::size().unwrap();
        ConsoleEngine::init(size.0 as u32, size.1 as u32, target_fps)
    }

    /// Initialize a screen filling the entire terminal with the target FPS  
    /// Also check the terminal width and height and assert if the terminal has at least the asked size
    pub fn init_fill_require(width: u32, height: u32, target_fps: u32) -> ConsoleEngine {
        let size = crossterm::terminal::size().unwrap();
        assert!(size.0 as u32 >= width && size.1 as u32 >= height, "Your terminal must have at least a width and height of {}x{} characters. Currently has {}x{}", width, height, size.0, size.1);
        ConsoleEngine::init_fill(target_fps)
    }

    /// Initializes the internal components such as hiding the cursor
    fn begin(&mut self) {
        terminal::enable_raw_mode().unwrap();
        execute!(
            self.stdout,
            terminal::Clear(ClearType::All),
            terminal::EnterAlternateScreen,
            crossterm::cursor::Hide,
            crossterm::cursor::MoveTo(0, 0),
            crossterm::event::EnableMouseCapture
        )
        .unwrap();
    }

    /// Gracefully stop the engine, and set back a visible cursor
    fn end(&mut self) {
        execute!(
            self.stdout,
            crossterm::cursor::Show,
            style::SetBackgroundColor(Color::Reset),
            style::SetForegroundColor(Color::Reset),
            crossterm::event::DisableMouseCapture,
            terminal::LeaveAlternateScreen
        )
        .unwrap();
        terminal::disable_raw_mode().unwrap();
    }

    /// Get the screen width
    pub fn get_width(&self) -> u32 {
        self.screen.get_width()
    }

    /// Get the screen height
    pub fn get_height(&self) -> u32 {
        self.screen.get_height()
    }

    /// Reset the screen to a blank state
    pub fn clear_screen(&mut self) {
        self.screen.clear()
    }

    /// prints a string at the specified coordinates.  
    /// The string will be cropped if it reach the right border
    ///
    /// usage:
    /// ```
    /// engine.print(0,0, "Hello, world!");
    /// engine.print(0, 4, format!("Score: {}", score).as_str());
    /// ```
    pub fn print(&mut self, x: i32, y: i32, string: &str) {
        self.screen.print(x, y, string)
    }

    /// prints a string at the specified coordinates with the specified foreground and background color  
    /// The string will automatically overlaps if it reach the right border
    ///
    /// usage:
    /// ```
    /// use console_engine::Color;
    ///
    /// // print "Hello, world" in blue on white background
    /// engine.print(0,0, "Hello, world!", Color::Blue, Color::White);
    /// ```
    pub fn print_fbg(&mut self, x: i32, y: i32, string: &str, fg: Color, bg: Color) {
        self.screen.print_fbg(x, y, string, fg, bg)
    }

    /// Prints another screen on specified coordinates.
    /// Useful when you want to manage several "subscreen"
    ///
    /// *see example* `screen-embed`
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
    /// // prints the square in the engine's screen at a specific location
    /// engine.print_screen(5,2, &my_square);
    /// ```
    pub fn print_screen(&mut self, x: i32, y: i32, source: &Screen) {
        self.screen.print_screen(x, y, source)
    }

    /// Prints another screen on specified coordinates, ignoring a specific character while printing
    /// Ignoring a character will behave like transparency
    ///
    /// see [print_screen](#method.print_screen) for usage
    pub fn print_screen_alpha(&mut self, x: i32, y: i32, source: &Screen, alpha_character: char) {
        self.screen
            .print_screen_alpha(x, y, source, alpha_character)
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
    /// engine.line(0, 0, 9, 9, pixel::pxl('#'));
    /// ```
    pub fn line(&mut self, start_x: i32, start_y: i32, end_x: i32, end_y: i32, character: Pixel) {
        self.screen.line(start_x, start_y, end_x, end_y, character)
    }

    /// Draws a rectangle of the provided character between two sets of coordinates  
    ///
    /// usage:
    /// ```
    /// use console_engine::pixel;
    /// // ...
    /// engine.rect(0, 0, 9, 9, pixel::pxl('#'));
    /// ```
    pub fn rect(&mut self, start_x: i32, start_y: i32, end_x: i32, end_y: i32, character: Pixel) {
        self.screen.rect(start_x, start_y, end_x, end_y, character)
    }

    /// Fill a rectangle of the provided character between two sets of coordinates  
    ///
    /// usage:
    /// ```
    /// use console_engine::pixel;
    /// // ...
    /// engine.fill_rect(0, 0, 9, 9, pixel::pxl('#'));
    /// ```
    pub fn fill_rect(
        &mut self,
        start_x: i32,
        start_y: i32,
        end_x: i32,
        end_y: i32,
        character: Pixel,
    ) {
        self.screen
            .fill_rect(start_x, start_y, end_x, end_y, character)
    }

    /// Draws a circle of the provided character at an x and y position with a radius
    /// see: [olcPixelGameEngine Repository](https://github.com/OneLoneCoder/olcPixelGameEngine)
    ///
    /// usage:
    /// ```
    /// use console_engine::pixel;
    /// // ...
    /// engine.circle(10, 10, 4, pixel::pxl('#'));
    /// ```
    pub fn circle(&mut self, x: i32, y: i32, radius: u32, character: Pixel) {
        self.screen.circle(x, y, radius, character)
    }

    /// Fill a circle of the provided character at an x and y position with a radius
    /// see: [olcPixelGameEngine Repository](https://github.com/OneLoneCoder/olcPixelGameEngine)
    ///
    /// usage:
    /// ```
    /// use console_engine::pixel;
    /// // ...
    /// engine.fill_circle(10, 10, 4, pixel::pxl('#'));
    /// ```
    pub fn fill_circle(&mut self, x: i32, y: i32, radius: u32, character: Pixel) {
        self.screen.fill_circle(x, y, radius, character)
    }

    /// Draws a triangle of the provided character using three sets of coordinates
    ///
    /// usage:
    /// ```
    /// use console_engine::pixel;
    /// // ...
    /// engine.triangle(8,8, 4,6, 9,2, pixel::pxl('#'));
    /// ```
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
        self.screen.triangle(x1, y1, x2, y2, x3, y3, character)
    }

    /// Fill a triangle of the provided character using three sets of coordinates
    /// see: [rustyPixelGameEngine Repository](https://github.com/mattbettcher/rustyPixelGameEngine)
    ///
    /// usage:
    /// ```
    /// use console_engine::pixel;
    /// // ...
    /// engine.fill_triangle(8,8, 4,6, 9,2, pixel::pxl('#'));
    /// ```
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
        self.screen.fill_triangle(x1, y1, x2, y2, x3, y3, character)
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
    pub fn set_pxl(&mut self, x: i32, y: i32, character: Pixel) {
        self.screen.set_pxl(x, y, character)
    }

    /// Get the character stored at provided coordinates
    ///
    /// usage:
    /// ```
    /// if engine.get_pxl(3,8).unwrap().chr == 'o' {
    ///     engine.print(0,0,"Found a 'o'");
    /// }
    /// ```
    pub fn get_pxl(&self, x: i32, y: i32) -> Result<Pixel, String> {
        self.screen.get_pxl(x, y)
    }

    /// Resizes the screen to match the given width and height
    /// truncates the bottom and right side of the screen
    ///
    /// usage:
    /// ```
    /// engine.resize(40,10)
    /// ```
    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        self.screen.resize(new_width, new_height);
        self.width = new_width;
        self.height = new_height;
        self.screen_last_frame = Screen::new_empty(self.width, self.height);
    }

    /// Changes the screen instance used by the engine and updates internal informations
    ///
    /// Useful if you want to manage multiple screens independently.
    ///
    /// usage
    /// ```
    /// // create a new screen of 40x10 and draw some things on it
    /// let mut scr = Screen::new(40,10)
    /// scr.rect(0,0,39,9, pixel::pxl("#"));
    /// // ...
    ///
    /// // keep a backup of the old screen before replacing it
    /// let old_scr = engine.get_screen();
    /// // change the engine's current screen to the newly created one
    /// engine.set_screen(&scr);
    ///
    /// // ... later
    /// // set back the old screen
    /// engine.set_screen(&old_scr);
    /// ```
    pub fn set_screen(&mut self, screen: &Screen) {
        self.width = screen.get_width();
        self.height = screen.get_height();
        self.screen = screen.clone();
        self.screen_last_frame = Screen::new_empty(self.width, self.height);
    }

    /// Returns a clone of the current screen
    ///
    /// You can keep it into a variable to restore the screen later, via `set_screen`.
    /// You can then use the to_string method to write the screen in a file for example
    ///
    /// see [set_screen](#method.set_screen) for a more complete example
    ///
    /// usage :
    /// ```
    /// let scr = engine.get_screen();
    /// ```
    pub fn get_screen(&self) -> Screen {
        self.screen.clone()
    }

    /// Draw the screen in the terminal  
    /// For best results, use it once per frame
    ///
    /// usage:
    /// ```
    /// engine.print(0,0,"Hello, world!"); // <- prints "Hello, world!" in 'screen' memory
    /// engine.draw(); // display 'screen' memory to the user's terminal
    /// ```
    pub fn draw(&mut self) {
        // we prepare an "output_screen" String variable to store in one-shot the screen we'll write.
        // This is an optimization because we write all we need once instead of writing small bit of screen by small bit of screen.
        // Actually, this does not change much for Linux terminals (like 5 fps gained from this)
        // But for windows terminal we can see huge improvements (example lines-fps goes from 35-40 fps to 65-70 for a 100x50 term)
        // reset cursor position
        queue!(self.stdout, crossterm::cursor::MoveTo(0, 0)).unwrap();
        let mut first = true;
        let mut current_colors: (Color, Color) = (Color::Reset, Color::Reset);
        let mut moving = false;
        self.screen_last_frame.check_empty(); // refresh internal "empty" value
                                              // iterates through the screen memory and prints it on the output buffer
        for y in 0..self.height as i32 {
            for x in 0..self.width as i32 {
                let pixel = self.screen.get_pxl(x, y).unwrap();
                // we check if the screen has been modified at this coordinate
                // if so, we write like normally, else we set a 'moving' flag
                if self.screen_last_frame.is_empty()
                    || pixel != self.screen_last_frame.get_pxl(x, y).unwrap()
                {
                    if moving {
                        // if the moving flag is set, we need to write a goto instruction first
                        // this optimization minimize useless write on the screen
                        // actually writing to the screen is very slow so it's a good compromise
                        queue!(self.stdout, crossterm::cursor::MoveTo(x as u16, y as u16)).unwrap();
                        moving = false;
                    }
                    // we check if the last color is the same as the current one.
                    // if the color is the same, only print the character
                    // the less we write on the output the faster we'll get
                    // and additional characters for colors we already have set is
                    // time consuming
                    if current_colors != pixel.get_colors() || first {
                        current_colors = pixel.get_colors();
                        queue!(
                            self.stdout,
                            style::SetForegroundColor(pixel.fg),
                            style::SetBackgroundColor(pixel.bg),
                            style::Print(pixel.chr)
                        )
                        .unwrap();
                        first = false;
                    } else {
                        queue!(self.stdout, style::Print(pixel.chr)).unwrap();
                    }
                } else {
                    moving = true
                }
            }
            if y < self.height as i32 - 1 {
                queue!(self.stdout, crossterm::cursor::MoveToNextLine(1)).unwrap();
            }
        }
        // flush the buffer into user's terminal
        self.stdout.flush().unwrap();
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
        let mut captured_keyboard: Vec<KeyEvent> = vec![];
        let mut captured_mouse: Vec<MouseEvent> = vec![];

        // if there is time before next frame, poll keyboard and mouse events until next frame
        let mut elapsed_time = self.instant.elapsed();
        while self.time_limit > elapsed_time {
            let remaining_time = self.time_limit - elapsed_time;
            if event::poll(std::time::Duration::from_millis(
                (remaining_time.as_millis() % self.time_limit.as_millis()) as u64,
            ))
            .unwrap()
            {
                match event::read().unwrap() {
                    Event::Key(evt) => {
                        captured_keyboard.push(evt);
                    }
                    Event::Mouse(evt) => {
                        captured_mouse.push(evt);
                    }
                    // we ignore resize events for now
                    Event::Resize(_w, _h) => {}
                };
            }
            elapsed_time = self.instant.elapsed();
        }
        self.instant = std::time::Instant::now();
        self.frame_count += 1;

        // updates pressed / held / released states
        let held = utils::intersect(
            &utils::union(&self.keys_pressed, &self.keys_held),
            &captured_keyboard,
        );
        self.keys_released = utils::outersect_left(&self.keys_held, &held);
        self.keys_pressed = utils::outersect_left(&captured_keyboard, &held);
        self.keys_held = utils::union(&held, &self.keys_pressed);
        self.mouse_events = captured_mouse;
    }

    /// Check and resize the terminal if needed.
    /// Note that the resize will occur but there is no check yet if the terminal
    /// is smaller than the required size provided in the init() function.
    ///
    /// usage:
    /// ```
    /// // initializes a screen filling the terminal
    /// let mut engine = console_engine::ConsoleEngine::init_fill(30);
    /// loop {
    ///     engine.wait_frame(); // wait for next frame
    ///     engine.check_resize(); // resize the terminal if its size has changed
    ///     // do your stuff
    /// }
    /// ```
    pub fn check_resize(&mut self) {
        if crossterm::terminal::size().unwrap() != (self.width as u16, self.height as u16) {
            // resize terminal
            let size = crossterm::terminal::size().unwrap();
            let new_width = size.0 as u32;
            let new_height = size.1 as u32;

            self.resize(new_width, new_height);
        }
    }

    /// checks whenever a key is pressed (first frame held only)
    ///
    /// usage:
    /// ```
    /// use console_engine::KeyCode;
    ///
    /// loop {
    ///     engine.wait_frame(); // wait for next frame + captures input
    ///     
    ///     if engine.is_key_pressed(KeyCode::Char('q')) {
    ///         break; // exits app
    ///     }
    /// }
    /// ```
    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.is_key_pressed_with_modifier(key, KeyModifiers::NONE)
    }

    /// checks whenever a key + a modifier (ctrl, shift...) is pressed (first frame held only)
    ///
    /// usage:
    /// ```
    /// use console_engine::{KeyCode, KeyModifiers}
    ///
    /// loop {
    ///     engine.wait_frame(); // wait for next frame + captures input
    ///     
    ///     if engine.is_key_pressed_with_modifier(KeyCode::Char('c'), KeyModifiers::CONTROL) {
    ///         break; // exits app
    ///     }
    /// }
    /// ```
    pub fn is_key_pressed_with_modifier(&self, key: KeyCode, modifier: KeyModifiers) -> bool {
        self.keys_pressed.contains(&KeyEvent::new(key, modifier))
    }

    /// checks whenever a key is held down
    ///
    /// usage:
    /// ```
    /// use console_engine::KeyCode;
    ///
    /// loop {
    ///     engine.wait_frame(); // wait for next frame + captures input
    ///     
    ///     if engine.is_key_held(KeyCode::Char('8')) && pos_y > 0 {
    ///         pos_y -= 1; // move position upward
    ///     }
    /// }
    /// ```
    pub fn is_key_held(&self, key: KeyCode) -> bool {
        self.is_key_held_with_modifier(key, KeyModifiers::NONE)
    }

    /// checks whenever a key + a modifier (ctrl, shift...) is held down
    pub fn is_key_held_with_modifier(&self, key: KeyCode, modifier: KeyModifiers) -> bool {
        self.keys_held.contains(&KeyEvent::new(key, modifier))
    }

    /// checks whenever a key has been released (first frame released)
    ///  
    /// usage:
    /// ```
    /// use console_engine::KeyCode;
    ///
    /// if engine.is_key_held(KeyCode::Char('h')) {
    ///     engine.clear_screen();
    ///     engine.print(0,0,"Please don't hold this button.");
    ///     engine.draw();
    ///     while !engine.is_key_released(KeyCode::Char('h')) {
    ///         engine.wait_frame(); // refresh button's states
    ///     }
    /// }
    /// ```
    pub fn is_key_released(&self, key: KeyCode) -> bool {
        self.is_key_released_with_modifier(key, KeyModifiers::NONE)
    }

    /// checks whenever a key + a modifier (ctrl, shift...) has been released (first frame released)
    pub fn is_key_released_with_modifier(&self, key: KeyCode, modifier: KeyModifiers) -> bool {
        self.keys_released.contains(&KeyEvent::new(key, modifier))
    }

    /// Give the mouse's terminal coordinates if the provided button has been pressed
    ///
    /// usage:
    /// ```
    /// use console_engine::MouseButton;
    ///
    /// // prints a 'P' where the mouse's left button has been pressed
    /// let mouse_pos = engine.get_mouse_press(MouseButton::Left);
    /// if let Some(mouse_pos) = mouse_pos {
    ///     engine.set_pxl(mouse_pos.0 as i32, mouse_pos.1 as i32, pixel::pxl('P'));
    /// }
    /// ```
    ///
    pub fn get_mouse_press(&self, button: MouseButton) -> Option<(u32, u32)> {
        self.get_mouse_press_with_modifier(button, KeyModifiers::NONE)
    }

    /// Give the mouse's terminal coordinates if the provided button + modifier (ctrl, shift, ...) has been pressed
    pub fn get_mouse_press_with_modifier(
        &self,
        button: MouseButton,
        modifier: KeyModifiers,
    ) -> Option<(u32, u32)> {
        for evt in self.mouse_events.iter() {
            if let MouseEvent::Down(mouse, x, y, key) = evt {
                if *mouse == button && *key == modifier {
                    return Some((*x as u32, *y as u32));
                }
            };
        }
        None
    }

    /// Give the mouse's terminal coordinates if a button is held on the mouse
    ///
    /// usage:
    /// ```
    /// use console_engine::MouseButton;
    ///
    /// // prints a 'H' where the mouse is currently held
    /// let mouse_pos = engine.get_mouse_held(MouseButton::Left);
    /// if let Some(mouse_pos) = mouse_pos {
    ///     engine.set_pxl(mouse_pos.0 as i32, mouse_pos.1 as i32, pixel::pxl('H'));
    /// }
    /// ```
    pub fn get_mouse_held(&self, button: MouseButton) -> Option<(u32, u32)> {
        self.get_mouse_held_with_modifier(button, KeyModifiers::NONE)
    }

    /// Give the mouse's terminal coordinates if a button + modifier (ctrl, shift, ...) is held on the mouse
    pub fn get_mouse_held_with_modifier(
        &self,
        button: MouseButton,
        modifier: KeyModifiers,
    ) -> Option<(u32, u32)> {
        for evt in self.mouse_events.iter() {
            if let MouseEvent::Drag(mouse, x, y, key) = evt {
                if *mouse == button && *key == modifier {
                    return Some((*x as u32, *y as u32));
                }
            };
        }
        None
    }

    /// Give the mouse's terminal coordinates if a button has been released on the mouse
    ///
    /// usage:
    /// ```
    /// use console_engine::MouseButton;
    ///
    /// // prints a 'R' where the mouse has been released
    /// let mouse_pos = engine.get_mouse_released(MouseButton::Left);
    /// if let Some(mouse_pos) = mouse_pos {
    ///     engine.set_pxl(mouse_pos.0 as i32, mouse_pos.1 as i32, pixel::pxl('R'));
    /// }
    /// ```
    pub fn get_mouse_released(&self, button: MouseButton) -> Option<(u32, u32)> {
        self.get_mouse_released_with_modifier(button, KeyModifiers::NONE)
    }

    /// Give the mouse's terminal coordinates if a button + modifier (ctrl, shift, ...) has been released on the mouse
    pub fn get_mouse_released_with_modifier(
        &self,
        button: MouseButton,
        modifier: KeyModifiers,
    ) -> Option<(u32, u32)> {
        for evt in self.mouse_events.iter() {
            if let MouseEvent::Up(mouse, x, y, key) = evt {
                if *mouse == button && *key == modifier {
                    return Some((*x as u32, *y as u32));
                }
            };
        }
        None
    }
}

impl Drop for ConsoleEngine {
    fn drop(&mut self) {
        self.end();
    }
}
