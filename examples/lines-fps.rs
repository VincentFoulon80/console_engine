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
    // initializes a screen filling the terminal with a target of 120 frame per second
    let mut engine = console_engine::ConsoleEngine::init_fill(120);
    // We initalize a stopwatch for our FPS counter
    let mut stopwatch = std::time::Instant::now();
    let mut last_fps = 0;
    // main loop, be aware that you'll have to break it because ctrl+C is captured
    loop {
        engine.wait_frame(); // wait for next frame + capture inputs
        if engine.is_key_pressed(Key::Char('q')) { // if the user presses 'q' :
            break; // exits app
        }
    
        // generate two random positions and a color
        let pos_1 = random_pos(engine.scr_w(), engine.scr_h());
        let pos_2 = random_pos(engine.scr_w(), engine.scr_h());
        let pxl_c = random_color();

        // draw a line using the three variables above
        engine.line(pos_1.0, pos_1.1, pos_2.0, pos_2.1, pixel::pxl_fg('#', color::Rgb(pxl_c.0, pxl_c.1, pxl_c.2)));

        // we keep a small space to display FPS at the top-left corner
        engine.print(0,0,String::from("         "));
        engine.print(0,0,format!("FPS: {}",last_fps));

        engine.draw(); // draw the screen

        // if our stopwatch has reached 1 second,
        // get the frame count and reset it for the next measure
        // also reset the stopwatch for the next measure
        if stopwatch.elapsed().as_millis() >= 1000 {
            last_fps = engine.frame_count;
            engine.frame_count = 0;
            stopwatch = std::time::Instant::now();
        }
    }
}