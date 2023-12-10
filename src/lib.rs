//! This library provides simple features to draw things and manage user input for terminal applications.
//!
//! Besides these features, this library also provides some tools to build standalone "screens" that can be used outside of the engine itself.
//!
//! It's built on top of [Crossterm](https://crates.io/crates/crossterm) for handling the screen and inputs. You don't have to worry about initalizing anything because this crate will handle this for you.

pub extern crate crossterm;

pub mod pixel;
pub mod rect_style;
pub mod screen;
mod utils;

#[cfg(feature = "event")]
pub mod events;

#[cfg(feature = "form")]
pub mod forms;

pub use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers, MouseButton};
pub use crossterm::style::Color;
use crossterm::terminal::{self, ClearType};
use crossterm::{
    event::{self, Event, KeyEvent, MouseEvent, MouseEventKind},
    ErrorKind,
};
use crossterm::{execute, queue, style};
use pixel::Pixel;
use rect_style::BorderStyle;
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
#[allow(clippy::needless_doctest_main)]
pub struct ConsoleEngine {
    stdout: Stdout,
    time_limit: std::time::Duration,
    /// The current frame count, publicly accessible
    /// Has no purpose internally, use it as you want
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
    resize_events: Vec<(u16, u16)>,
}

impl ConsoleEngine {
    /// Initialize a screen of the provided width and height, and load the target FPS
    pub fn init(width: u32, height: u32, target_fps: u32) -> Result<ConsoleEngine, ErrorKind> {
        assert!(target_fps > 0, "Target FPS needs to be greater than zero.");
        let mut engine = ConsoleEngine {
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
            resize_events: vec![],
        };
        let previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            Self::handle_panic(panic_info);
            previous_panic_hook(panic_info);
            std::process::exit(1);
        }));
        engine.begin()?;
        engine.try_resize(width, height)?;
        Ok(engine)
    }

    /// Initialize a screen filling the entire terminal with the target FPS
    pub fn init_fill(target_fps: u32) -> Result<ConsoleEngine, ErrorKind> {
        let size = crossterm::terminal::size()?;
        ConsoleEngine::init(size.0 as u32, size.1 as u32, target_fps)
    }

    /// Initialize a screen filling the entire terminal with the target FPS
    /// Also check the terminal width and height and assert if the terminal has at least the asked size
    pub fn init_fill_require(
        width: u32,
        height: u32,
        target_fps: u32,
    ) -> Result<ConsoleEngine, ErrorKind> {
        let mut engine = ConsoleEngine::init_fill(target_fps)?;
        engine.try_resize(width, height)?;
        Ok(engine)
    }

    /// Try to resize the terminal to match the asked width and height at minimum
    fn try_resize(&mut self, width: u32, height: u32) -> Result<(), ErrorKind> {
        let size = crossterm::terminal::size()?;
        if (size.0 as u32) < width || (size.1 as u32) < height {
            execute!(
                self.stdout,
                crossterm::terminal::SetSize(width as u16, height as u16),
                crossterm::terminal::SetSize(width as u16, height as u16)
            )?;
            self.resize(width, height);
            // flush events
            #[cfg(feature = "event")]
            use std::time::Duration;
            #[cfg(feature = "event")]
            while let Ok(true) = event::poll(Duration::from_micros(100)) {
                event::read().ok();
            }
        }
        if crossterm::terminal::size()? < (width as u16, height as u16) {
            Err(ErrorKind::new(std::io::ErrorKind::Other, format!("Your terminal must have at least a width and height of {}x{} characters. Currently has {}x{}", width, height, size.0, size.1)))
        } else {
            Ok(())
        }
    }

    /// Initializes the internal components such as hiding the cursor
    fn begin(&mut self) -> Result<(), ErrorKind> {
        terminal::enable_raw_mode().unwrap();
        execute!(
            self.stdout,
            terminal::EnterAlternateScreen,
            terminal::Clear(ClearType::All),
            crossterm::cursor::Hide,
            crossterm::cursor::MoveTo(0, 0),
            crossterm::event::EnableMouseCapture
        )
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

    /// stops the engine when a panic occurs
    /// Similar to the end function, but without the engine instance.
    /// So we assume we used stdout, and free it.
    fn handle_panic(_panic_info: &std::panic::PanicInfo) {
        execute!(
            stdout(),
            crossterm::cursor::Show,
            style::SetBackgroundColor(Color::Reset),
            style::SetForegroundColor(Color::Reset),
            crossterm::event::DisableMouseCapture,
            terminal::LeaveAlternateScreen
        )
        .unwrap();
        terminal::disable_raw_mode().unwrap();
    }

    /// Set the terminal's title
    pub fn set_title(&mut self, title: &str) {
        execute!(self.stdout, crossterm::terminal::SetTitle(title)).ok();
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

    /// Fill the entire screen to the given pixel
    pub fn fill(&mut self, pixel: Pixel) {
        self.screen.fill(pixel);
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

    /// Draws a rectangle with custom borders of the provided between two sets of coordinates. Check the BorderStyle struct to learn how to use built-in or custom styles
    ///
    /// usage:
    /// ```
    /// use console_engine::rect_style::BorderStyle;
    /// // ...
    /// engine.rect_border(0, 0, 9, 9, BorderStyle::new_simple());
    /// ```
    pub fn rect_border(
        &mut self,
        start_x: i32,
        start_y: i32,
        end_x: i32,
        end_y: i32,
        rect_style: BorderStyle,
    ) {
        self.screen
            .rect_border(start_x, start_y, end_x, end_y, rect_style)
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
        self.screen.fill_triangle(x1, y1, x2, y2, x3, y3, character)
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
    /// engine.fill(pixel::pxl('#'));
    /// // free one space to the bottom
    /// engine.scroll(0,1,pixel::pxl(' '));
    /// // print something at this place
    /// engine.print(0, height-1, "Hello, world!");
    /// ```
    pub fn scroll(&mut self, h_scroll: i32, v_scroll: i32, background: Pixel) {
        self.screen.scroll(h_scroll, v_scroll, background);
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

    /// Extracts part of the current screen as a separate Screen object
    /// The original screen is not altered
    /// If the coordinates are out of bounds, they'll be replace by the `default` pixel
    ///
    /// usage:
    /// ```
    /// use console_engine::pixel;
    /// // extract a 3x2 screen from the engine screen
    /// let scr_chunk = engine.extract(10, 4, 12, 5, pixel::pxl(' '));
    /// ```
    pub fn extract(
        &self,
        start_x: i32,
        start_y: i32,
        end_x: i32,
        end_y: i32,
        default: Pixel,
    ) -> Screen {
        self.screen.extract(start_x, start_y, end_x, end_y, default)
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
        self.request_full_draw();
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
    /// If the terminal content is changed outside of the draw call, the draw function won't be aware of it and may leave some artifacts.
    /// If you want to force the draw function to redraw the entire screen, you should call [request_full_draw](#method.request_full_draw) before `draw()`.
    ///
    /// That's because for optimizing the output speed, the draw function only draw the difference between each frames.
    ///
    /// usage:
    /// ```
    /// engine.print(0,0,"Hello, world!"); // <- prints "Hello, world!" in 'screen' memory
    /// engine.draw(); // display 'screen' memory to the user's terminal
    /// ```
    pub fn draw(&mut self) {
        // we use the queue! macro to store in one-shot the screen we'll write.
        // This is an optimization because we write all we need once instead of writing small bit of screen by small bit of screen.
        // Actually, this does not change much for Linux terminals (like 5 fps gained from this)
        // But for windows terminal we can see huge improvements (example lines-fps goes from 35-40 fps to 65-70 for a 100x50 term)
        // reset cursor position
        queue!(self.stdout, crossterm::cursor::MoveTo(0, 0)).unwrap();
        let mut first = true;
        let mut current_colors: (Color, Color) = (Color::Reset, Color::Reset);
        let mut moving = false;
        self.screen_last_frame.check_empty(); // refresh internal "empty" value of the last_frame screen
        let mut skip_next = false;

        // iterates through the screen memory and prints it on the output buffer
        for y in 0..self.height as i32 {
            for x in 0..self.width as i32 {
                let pixel = self.screen.get_pxl(x, y).unwrap();
                // we check if the screen has been modified at this coordinate or if the last_frame screen is empty
                // if so, we write on the terminal normally, else we set a 'moving' flag
                if skip_next {
                    skip_next = false;
                    continue;
                }
                if let Some(char_width) = unicode_width::UnicodeWidthChar::width(pixel.chr) {
                    if char_width > 1 {
                        skip_next = true;
                    }
                }
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
            // at the end of each line, we write a newline character
            // I believe that since we're on raw mode we need CR and LF even on unix terminals
            if y < self.height as i32 - 1 {
                queue!(self.stdout, style::Print("\r\n")).unwrap();
            }
        }
        // flush the buffer into user's terminal
        self.stdout.flush().unwrap();
        // store the frame for the next draw call
        self.screen_last_frame = self.screen.clone();
    }

    /// Ask the engine to redraw the entire screen on the next `draw` call
    /// Useful if the terminal's content got altered outside of the `draw` function.
    ///
    /// See [draw](#method.draw) for more info about the drawing process
    pub fn request_full_draw(&mut self) {
        // reset the last_frame screen to force a full redraw
        self.screen_last_frame = Screen::new_empty(self.width, self.height);
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
        let mut captured_resize: Vec<(u16, u16)> = vec![];

        // if there is time before next frame, poll keyboard and mouse events until next frame
        let mut elapsed_time = self.instant.elapsed();
        while self.time_limit > elapsed_time {
            let remaining_time = self.time_limit - elapsed_time;
            if let Ok(has_event) = event::poll(std::time::Duration::from_millis(
                (remaining_time.as_millis() % self.time_limit.as_millis()) as u64,
            )) {
                if has_event {
                    if let Ok(current_event) = event::read() {
                        match current_event {
                            Event::Key(evt) => {
                                captured_keyboard.push(evt);
                            }
                            Event::Mouse(evt) => {
                                captured_mouse.push(evt);
                            }
                            Event::Resize(w, h) => {
                                captured_resize.push((w, h));
                            }
                            Event::FocusGained => (),
                            Event::FocusLost => (),
                            Event::Paste(_) => (),
                        };
                    }
                }
            }
            elapsed_time = self.instant.elapsed();
        }
        self.instant = std::time::Instant::now();
        self.frame_count = self.frame_count.wrapping_add(1);

        // updates pressed / held / released states
        let held = utils::intersect(
            &utils::union(&self.keys_pressed, &self.keys_held),
            &captured_keyboard,
        );
        self.keys_released = utils::outersect_left(&self.keys_held, &held);
        self.keys_pressed = utils::outersect_left(&captured_keyboard, &held);
        self.keys_held = utils::union(&held, &self.keys_pressed);
        self.mouse_events = captured_mouse;
        self.resize_events = captured_resize;
    }

    /// Poll the next ConsoleEngine Event
    /// This function waits for the next event to occur,
    /// from a user event like key press or mouse click to automatic events like frame change
    ///
    /// usage:
    /// ```
    /// use console_engine::events::Event;
    /// // initializes a screen with a 10x10 screen and targetting 30 fps
    /// let mut engine = console_engine::ConsoleEngine::init(10, 10, 30).unwrap();
    /// loop {
    ///     match engine.poll() {
    ///         Event::Frame => {
    ///             // do things
    ///             engine.draw();
    ///         }
    ///         Event::Key(key_event) => {
    ///             // handle keys
    ///         }
    ///     }
    /// }
    /// ```
    #[cfg(feature = "event")]
    pub fn poll(&mut self) -> events::Event {
        use std::time::Duration;

        let mut elapsed_time = self.instant.elapsed();
        // guarantees that this loop is running at least once
        loop {
            let remaining_time = if self.time_limit > elapsed_time {
                self.time_limit - elapsed_time
            } else {
                Duration::from_millis(0)
            };
            if let Ok(has_event) = event::poll(std::time::Duration::from_millis(
                (remaining_time.as_millis() % self.time_limit.as_millis()) as u64,
            )) {
                if has_event {
                    if let Ok(current_event) = event::read() {
                        match current_event {
                            Event::Key(evt) => return events::Event::Key(evt),
                            Event::Mouse(evt) => return events::Event::Mouse(evt),
                            Event::Resize(w, h) => return events::Event::Resize(w, h),
                            Event::FocusGained => (),
                            Event::FocusLost => (),
                            Event::Paste(_) => (),
                        };
                    }
                }
            }
            elapsed_time = self.instant.elapsed();
            if self.time_limit <= elapsed_time {
                break;
            }
        }
        self.instant = std::time::Instant::now();
        self.frame_count = self.frame_count.wrapping_add(1);
        events::Event::Frame
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
        self.is_key_pressed_with_modifier(key, KeyModifiers::NONE, KeyEventKind::Press)
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
    pub fn is_key_pressed_with_modifier(
        &self,
        key: KeyCode,
        modifier: KeyModifiers,
        kind: KeyEventKind,
    ) -> bool {
        self.keys_pressed
            .contains(&KeyEvent::new_with_kind(key, modifier, kind))
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
        self.is_key_held_with_modifier(key, KeyModifiers::NONE, KeyEventKind::Press)
    }

    /// checks whenever a key + a modifier (ctrl, shift...) is held down
    pub fn is_key_held_with_modifier(
        &self,
        key: KeyCode,
        modifier: KeyModifiers,
        kind: KeyEventKind,
    ) -> bool {
        self.keys_held
            .contains(&KeyEvent::new_with_kind(key, modifier, kind))
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
        self.is_key_released_with_modifier(key, KeyModifiers::NONE, KeyEventKind::Release)
    }

    /// checks whenever a key + a modifier (ctrl, shift...) has been released (first frame released)
    pub fn is_key_released_with_modifier(
        &self,
        key: KeyCode,
        modifier: KeyModifiers,
        kind: KeyEventKind,
    ) -> bool {
        self.keys_released
            .contains(&KeyEvent::new_with_kind(key, modifier, kind))
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
            if let MouseEventKind::Down(mouse) = evt.kind {
                if mouse == button && evt.modifiers == modifier {
                    return Some((evt.column as u32, evt.row as u32));
                }
            };
        }
        None
    }

    /// Give the terminal resize event
    ///
    /// usage:
    /// ```
    /// if let Some((width, height)) = engine.get_resize() {
    ///     // do something
    /// }
    /// ```
    pub fn get_resize(&self) -> Option<(u16, u16)> {
        for evt in self.resize_events.iter() {
            if let Event::Resize(w, h) = Event::Resize(evt.0, evt.1) {
                return Some((w, h));
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
            if let MouseEventKind::Drag(mouse) = evt.kind {
                if mouse == button && evt.modifiers == modifier {
                    return Some((evt.column as u32, evt.row as u32));
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
            if let MouseEventKind::Up(mouse) = evt.kind {
                if mouse == button && evt.modifiers == modifier {
                    return Some((evt.column as u32, evt.row as u32));
                }
            };
        }
        None
    }

    /// checks whenever the mouse's scroll has been turned down, towards the user
    ///
    /// usage:
    /// ```
    /// if engine.is_mouse_scrolled_down() {
    ///     // do some scrolling logic
    /// }
    /// ```
    pub fn is_mouse_scrolled_down(&self) -> bool {
        self.is_mouse_scrolled_down_with_modifier(KeyModifiers::NONE)
    }

    /// checks whenever the mouse's scroll has been turned down, towards the user with a modifier (ctrl, shift, ...)
    pub fn is_mouse_scrolled_down_with_modifier(&self, modifier: KeyModifiers) -> bool {
        for evt in self.mouse_events.iter() {
            if let MouseEventKind::ScrollDown = evt.kind {
                if evt.modifiers == modifier {
                    return true;
                }
            };
        }
        false
    }

    /// checks whenever the mouse's scroll has been turned up, away from the user
    ///
    /// usage:
    /// ```
    /// if engine.is_mouse_scrolled_up() {
    ///     // do some scrolling logic
    /// }
    /// ```
    pub fn is_mouse_scrolled_up(&self) -> bool {
        self.is_mouse_scrolled_up_with_modifier(KeyModifiers::NONE)
    }

    /// checks whenever the mouse's scroll has been turned up, away from the user with a modifier (ctrl, shift, ...)
    pub fn is_mouse_scrolled_up_with_modifier(&self, modifier: KeyModifiers) -> bool {
        for evt in self.mouse_events.iter() {
            if let MouseEventKind::ScrollUp = evt.kind {
                if evt.modifiers == modifier {
                    return true;
                }
            };
        }
        false
    }
}

impl Drop for ConsoleEngine {
    /// gracefully stop the engine when dropping it
    fn drop(&mut self) {
        self.end();
    }
}
