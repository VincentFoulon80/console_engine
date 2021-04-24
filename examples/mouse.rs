use console_engine::pixel;
use console_engine::KeyCode;
use console_engine::MouseButton;

fn main() {
    // initializes a screen filling the terminal with a target of 30 frames per second
    let mut engine = console_engine::ConsoleEngine::init_fill(30).unwrap();

    // main loop, be aware that you'll have to break it because ctrl+C is captured
    loop {
        engine.wait_frame(); // wait for next frame + capture inputs
        engine.check_resize(); // resize the terminal if its size has changed
        if engine.is_key_pressed(KeyCode::Char('q')) {
            // if the user presses 'q' :
            break; // exits app
        }

        // prints a 'P' where the mouse's left button has been pressed
        let mouse_pos = engine.get_mouse_press(MouseButton::Left);
        if let Some(mouse_pos) = mouse_pos {
            engine.set_pxl(mouse_pos.0 as i32, mouse_pos.1 as i32, pixel::pxl('P'));
        }

        // prints a 'H' where the mouse is currently held
        let mouse_pos = engine.get_mouse_held(MouseButton::Left);
        if let Some(mouse_pos) = mouse_pos {
            engine.set_pxl(mouse_pos.0 as i32, mouse_pos.1 as i32, pixel::pxl('H'));
        }

        // prints a 'R' where the mouse has been released
        let mouse_pos = engine.get_mouse_released(MouseButton::Left);
        if let Some(mouse_pos) = mouse_pos {
            engine.set_pxl(mouse_pos.0 as i32, mouse_pos.1 as i32, pixel::pxl('R'));
        }

        engine.draw(); // draw the screen
    }
}
