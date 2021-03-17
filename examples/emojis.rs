use console_engine::{KeyCode, pixel};

fn main() {
    let mut engine = console_engine::ConsoleEngine::init(6, 5, 3);
    loop {
        engine.wait_frame();
        engine.clear_screen();

        // draw a rectangle with an emoji inside
        engine.rect(0, 0, 5, 4, pixel::pxl('#'));
        engine.set_pxl(2, 2, pixel::pxl('👍'));

        if engine.is_key_pressed(KeyCode::Char('q')) {
            break;
        }

        engine.draw();
    }
}
