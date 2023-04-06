use console_engine::pixel;
use console_engine::Color;
use console_engine::KeyCode;

fn main() {
    let (mut x, mut y) = (0, 0);
    // initializes a screen of 30x20 characters with a target of 3 frames per second
    // coordinates will range from [0,0] to [29,19]
    let mut engine = console_engine::ConsoleEngine::init(30, 20, 10).unwrap();
    // draw the background
    engine.fill(pixel::pxl_bg(' ', Color::Cyan));
    // draw the window background
    engine.fill_rect(
        5,
        5,
        engine.get_width() as i32 - 5,
        engine.get_height() as i32 - 5,
        pixel::pxl_bg(' ', Color::White),
    );
    // draw the window borders
    engine.rect(
        5,
        5,
        engine.get_width() as i32 - 5,
        engine.get_height() as i32 - 5,
        pixel::pxl_bg(' ', Color::Blue),
    );
    // write something to the window
    engine.print_fbg(7, 7, "push arrows", Color::Black, Color::White);
    engine.print_fbg(7, 8, "to scroll", Color::Black, Color::White);
    engine.print_fbg(7, 9, "q to quit", Color::Black, Color::White);

    // main loop, be aware that you'll have to break it because ctrl+C is captured
    loop {
        engine.wait_frame(); // wait for next frame + capture inputs

        if engine.is_key_pressed(KeyCode::Char('q')) {
            // if the user presses 'q' :
            break; // exits app
        }

        if engine.is_key_held(KeyCode::Up) {
            y = -1;
            x = 0;
        }
        if engine.is_key_held(KeyCode::Down) {
            y = 1;
            x = 0;
        }
        if engine.is_key_held(KeyCode::Left) {
            x = -1;
            y = 0;
        }
        if engine.is_key_held(KeyCode::Right) {
            x = 1;
            y = 0;
        }

        // is_key_released is windows only as of crossterm 0.26.1
        // sometimes going the opposite direction will incorrectly trigger this code
        if engine.is_key_released(KeyCode::Up) && y == -1
            || engine.is_key_released(KeyCode::Down) && y == 1
        {
            y = 0;
        }
        if engine.is_key_released(KeyCode::Left) && x == -1
            || engine.is_key_released(KeyCode::Right) && x == 1
        {
            x = 0;
        }

        engine.scroll(x, y, pixel::pxl_bg(' ', Color::Cyan)); // continually update x and y
        engine.draw(); // draw the screen
    }
}
