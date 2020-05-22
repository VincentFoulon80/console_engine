use console_engine::pixel;
use console_engine::termion::event::Key;
use console_engine::termion::event::MouseButton;

fn main() {
    // initializes a screen filling the terminal with a target of 30 frames per second
    let mut engine = console_engine::ConsoleEngine::init_fill_require(30, 20, 30);
    
    let mut rect_x = 8;
    let mut rect_y = 3;
    let rect_w = 16;
    let rect_h = 4;
    let mut dragging = false;
    let mut relative_x = 0;
    let mut relative_y = 0;

    // main loop, be aware that you'll have to break it because ctrl+C is captured
    loop {
        engine.wait_frame(); // wait for next frame + capture inputs
        engine.check_resize(); // resize the terminal if its size has changed
        if engine.is_key_pressed(Key::Char('q')) { // if the user presses 'q' :
            break; // exits app
        }
        engine.clear_screen();

        // check if the mouse's left button has been pressed
        let mouse_pos = engine.get_mouse_press(MouseButton::Left);
        if mouse_pos.is_some() {
            // if the mouse position is within the boundaries of the rectangle,
            // enables dragging mode and register relative position of the mouse
            let mouse_pos = mouse_pos.unwrap();
            if mouse_pos.0 as i32 >= rect_x && mouse_pos.0 as i32 <= rect_x+rect_w
            && mouse_pos.1 as i32 >= rect_y && mouse_pos.1 as i32 <= rect_y+rect_h {
                dragging = true;
                relative_x = mouse_pos.0 as i32 - rect_x;
                relative_y = mouse_pos.1 as i32 - rect_y;
            }
        }
        
        // check if a mouse button is currently held
        let mouse_pos = engine.get_mouse_held();
        if mouse_pos.is_some() {
            // if dragging mode is enabled, move the rectangle according to mouse's position
            let mouse_pos = mouse_pos.unwrap();
            if dragging {
                rect_x = mouse_pos.0 as i32 - relative_x;
                rect_y = mouse_pos.1 as i32 - relative_y;
            }
        }

        // check if the mouse has been released
        let mouse_pos = engine.get_mouse_released();
        if mouse_pos.is_some() {
            // disable dragging mode
            dragging = false;
        }

        // print the recrangle
        engine.rect(rect_x, rect_y, rect_x+rect_w, rect_y+rect_h, pixel::pxl('#'));
        engine.print(rect_x+4, rect_y+2, "Drag me!");

        engine.draw(); // draw the screen
    }
}