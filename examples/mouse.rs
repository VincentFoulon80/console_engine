use console_engine::pixel;
use console_engine::termion::color;
use console_engine::termion::event::Key;

fn main() {
    // initializes a screen filling the terminal with a target of 30 frames per second
    let mut engine = console_engine::ConsoleEngine::init_fill(30);
    
    // main loop, be aware that you'll have to break it because ctrl+C is captured
    loop {
        engine.wait_frame(); // wait for next frame + capture inputs
        if engine.is_key_pressed(Key::Char('q')) { // if the user presses 'q' :
            break; // exits app
        }

        // prints a 'P' where the mouse's left button has been pressed
        let mouse_pos = engine.get_mouse_press(termion::event::MouseButton::Left);
        if mouse_pos.is_some() {
            let mouse_pos = mouse_pos.unwrap();
            engine.set_pxl(mouse_pos.0 as i32, mouse_pos.1 as i32, pixel::pxl('P'));
        }
        
        // prints a 'H' where the mouse is currently held
        let mouse_pos = engine.get_mouse_held();
        if mouse_pos.is_some() {
            let mouse_pos = mouse_pos.unwrap();
            engine.set_pxl(mouse_pos.0 as i32, mouse_pos.1 as i32, pixel::pxl('H'));
        }

        // prints a 'R' where the mouse has been released
        let mouse_pos = engine.get_mouse_released();
        if mouse_pos.is_some() {
            let mouse_pos = mouse_pos.unwrap();
            engine.set_pxl(mouse_pos.0 as i32, mouse_pos.1 as i32, pixel::pxl('R'));
        }


        engine.draw(); // draw the screen
    }
}