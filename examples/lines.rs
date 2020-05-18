use console_engine::pixel;
use console_engine::termion::color;
use console_engine::termion::event::Key;

// generate a random pair of u32
fn random_pos(max_x: u32, max_y: u32) -> (u32,u32) {
    (rand::random::<u32>() % max_x, rand::random::<u32>() % max_y)
}
// generate a random tuple of three numbers for R, G and B
fn random_color() ->(u8, u8, u8) {
    (rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>())
}

fn main() {
    // initializes a screen filling the terminal with a target of 60 frame per second
    let mut engine = console_engine::ConsoleEngine::init_fill(60);
    // main loop, be aware that you'll have to break it because ctrl+C is captured
    loop {
        engine.wait_frame(); // wait for next frame + capture inputs
        engine.check_resize(); // resize the terminal if its size has changed
        // exit check
        if engine.is_key_pressed(Key::Char('q')) {
            break;
        }

        // Note that we don't clear the screen here so that old lines drawed are not cleared

        // generate two random positions and a color
        let pos_1 = random_pos(engine.get_width(), engine.get_height());
        let pos_2 = random_pos(engine.get_width(), engine.get_height());
        let pxl_c = random_color();
        
        // draw a line using the three variables above
        engine.line(pos_1.0 as i32, pos_1.1 as i32, pos_2.0 as i32, pos_2.1 as i32, pixel::pxl_fg('#', color::Rgb(pxl_c.0, pxl_c.1, pxl_c.2)));
    
        engine.draw(); // draw the screen
    }
}