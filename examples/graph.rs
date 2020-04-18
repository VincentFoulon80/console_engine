use console_engine::pixel;
use console_engine::termion::color;
use console_engine::termion::event::Key;

const MAX_VALUES: usize = 64;

/// Dummy values to display on the screen
const VALUES: [u8; MAX_VALUES] = [
    4, 13, 64, 62, 60, 160, 67, 71,
    58, 52, 53, 60, 46, 11, 20, 24,
    34, 42, 59, 120, 114, 106, 123, 250,
    255, 243, 170, 176, 150, 155, 164, 138,
    120, 120, 120, 120, 0, 129, 114, 98,
    80, 74, 118, 104, 65, 80, 74, 60,
    38, 49, 70, 120, 39, 185, 170, 157,
    150, 133, 119, 100, 89, 60, 17, 0
];

/// Function that takes a list of values and uses a ConsoleEngine instance to draw it on the screen
fn draw_graph(engine: &mut console_engine::ConsoleEngine, values: [u8; MAX_VALUES]) {
    let ceiling = engine.scr_h()-3;
    let step  = engine.scr_w() as f32/MAX_VALUES as f32;
    let mut last_position = 0;
    // for each values in the dataset
    for i in 0..MAX_VALUES-1 {
        let value = values[i];
        // process the position based on the available space in the terminal
        let position = ((value as f32/255f32)*ceiling as f32) as u32;

        // draw a line using the last position registered (see below) and the current position.
        engine.line((i as f32*step) as u32, 2+ceiling-last_position, ((1+i) as f32*step) as u32, 2+ceiling-position, pixel::pxl('*'));

        // keep the position for the next iteration
        last_position = position;
    }
}

fn main() {

    // initializes a screen filling the terminal of at least MAX_VALUESx10 of size with a target of 10 frame per second
    let mut engine = console_engine::ConsoleEngine::init_fill_require(MAX_VALUES as u32,10,10);
    
    // initalize some variables
    let mut values: [u8; MAX_VALUES] = [0; MAX_VALUES];
    let mut value_position = 0usize;
    let mut sum = 0u32;
    let step  = engine.scr_w() as f32/MAX_VALUES as f32;
    // main loop, be aware that you'll have to break it because ctrl+C is captured
    loop {
        engine.wait_frame(); // wait for next frame + capture inputs
        engine.clear_screen(); // reset the screen

        // progressively add dummy values to the graph at each frame,
        // and display a message when it's finished
        if value_position >= MAX_VALUES {
            if engine.frame_count % 10 > 5 {
                engine.print_fbg(2,3, String::from("Press 'q' to close"), color::LightYellow, color::Black);
            }
        } else {
            let value = VALUES[value_position];
            sum += value as u32;
            values[value_position] = value;
            value_position += 1;
        }

        
        // Display a header with an average and sum calculation
        engine.print(0, 0, format!("Average : {}", (sum/value_position as u32) as f32));
        engine.set_pxl((engine.scr_w()/2)-1, 0, pixel::pxl('#'));
        engine.print((engine.scr_w()/2)+1, 0, format!("Sum : {}", sum));
        engine.line(0, 1, engine.scr_w()-1, 1, pixel::pxl('#'));
        // Draw a line at the position of the last value written
        if value_position < MAX_VALUES {
            engine.line((value_position as f32*step) as u32, 2, (value_position as f32*step) as u32, engine.scr_h()-1, pixel::pxl_bg(' ', color::Blue));
        }
        // draw the graph
        draw_graph(&mut engine, values);


        if engine.is_key_pressed(Key::Char('q')) { // if the user presses 'q' :
            break; // exits app
        }
        engine.draw(); // draw the screen
    }
}