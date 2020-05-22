use console_engine::pixel;
use console_engine::screen::Screen;
use console_engine::termion::color;
use console_engine::termion::event::Key;

fn main() {
    // initializes a screen of 30x10 characters with a target of 10 frames per second
    let mut engine = console_engine::ConsoleEngine::init(30, 10, 10);

    // create three screen and fill them with some data
    let mut screen_help = Screen::new(30, 10);
    let mut screen_shapes = Screen::new(30, 10);
    let mut screen_empty = Screen::new(30, 10);

    // initializes screen_help
    screen_help.print(1, 1, "*help*");
    screen_help.print(1, 2, "Press any of these keys");
    screen_help.print(2, 4, "1. help (this screen)");
    screen_help.print(2, 5, "2. some shapes");
    screen_help.print(2, 6, "3. empty screen");

    // initializes screen_shapes
    screen_shapes.rect(0, 0, 29, 9, pixel::pxl('+'));
    screen_shapes.fill_circle(4, 4, 2, pixel::pxl_fg('0', color::LightBlue));
    screen_shapes.fill_triangle(27, 2, 27, 7, 17, 7, pixel::pxl_fg('#', color::Green));

    // initializes screen_empty
    screen_empty.print_fbg(
        11,
        9,
        "It's empty, right ?",
        color::LightBlack,
        color::Black,
    );

    // set the engine's screen to help on startup
    engine.set_screen(&screen_help);

    // main loop, be aware that you'll have to break it because ctrl+C is captured
    loop {
        engine.wait_frame(); // wait for next frame + capture inputs

        // exit check
        if engine.is_key_pressed(Key::Char('q')) {
            break;
        }

        // when the corresponding key is pressed (1,2 or 3), swap to the corresponding screen
        if engine.is_key_pressed(Key::Char('1')) {
            engine.set_screen(&screen_help);
        } else if engine.is_key_pressed(Key::Char('2')) {
            engine.set_screen(&screen_shapes);
        } else if engine.is_key_pressed(Key::Char('3')) {
            engine.set_screen(&screen_empty);
        }

        engine.draw(); // draw the screen
    }
}
