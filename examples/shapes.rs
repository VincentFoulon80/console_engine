use console_engine::pixel;
use console_engine::termion::color;
use console_engine::termion::event::Key;


fn main() {
    // initializes a screen filling the terminal of at least 20x15 of size with a target of 3 frame per second
    let mut engine = console_engine::ConsoleEngine::init_fill_require(20,15,3);
    // main loop, be aware that you'll have to break it because ctrl+C is captured
    loop {
        engine.wait_frame(); // wait for next frame + capture inputs
        // exit check
        if engine.is_key_pressed(Key::Char('q')) {
            break;
        }
        engine.clear_screen();

        // use of some shape functions
        engine.rect(1,1, 18,6, pixel::pxl('#'));

        engine.fill_rect(14,4, 21,8, pixel::pxl_fg('~', color::LightBlue));
        engine.circle(10,10, 5, pixel::pxl_fg('*', color::LightGreen));

        engine.fill_circle(8,8, 3, pixel::pxl_fg('*', color::LightYellow));

    
        engine.draw(); // draw the screen
    }
}