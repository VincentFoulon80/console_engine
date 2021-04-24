use console_engine::pixel;
use console_engine::Color;
use console_engine::KeyCode;

fn main() {
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

        if engine.is_key_held(KeyCode::Left) {
            engine.scroll(-1, 0, pixel::pxl_bg(' ', Color::Cyan)); // scroll to the right
        }
        if engine.is_key_held(KeyCode::Right) {
            engine.scroll(1, 0, pixel::pxl_bg(' ', Color::Cyan)); // scroll to the left
        }
        if engine.is_key_held(KeyCode::Up) {
            engine.scroll(0, -1, pixel::pxl_bg(' ', Color::Cyan)); // scroll to the bottom
        }
        if engine.is_key_held(KeyCode::Down) {
            engine.scroll(0, 1, pixel::pxl_bg(' ', Color::Cyan)); // scroll to the top
        }
        engine.draw(); // draw the screen
    }
}
