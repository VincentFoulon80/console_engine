use console_engine::pixel;
use console_engine::termion::color;
use console_engine::termion::event::Key;

#[derive(Debug, PartialEq, Clone)]
enum Shapes {
    Rect,
    Circle,
    Triangle,
    Polygon
}


fn main() {
    // initializes a screen filling the terminal of at least 50x20 of size with a target of 3 frame per second
    let mut engine = console_engine::ConsoleEngine::init_fill_require(50,20,5);

    let mut coords = vec![(4,4),(18,12)];
    let mut selection = 0;

    let mut shape = Shapes::Rect;
    let mut fill = false;
    // main loop, be aware that you'll have to break it because ctrl+C is captured
    loop {
        engine.wait_frame(); // wait for next frame + capture inputs
        engine.check_resize(); // resize the terminal if its size has changed
        // exit check
        if engine.is_key_pressed(Key::Char('q')) {
            break;
        }
        engine.clear_screen();
        // draw the currently selected shape
        match shape {
            Shapes::Rect => {
                if fill {
                    engine.fill_rect(coords[0].0, coords[0].1, coords[1].0, coords[1].1, pixel::pxl('#'));
                } else {
                    engine.rect(coords[0].0, coords[0].1, coords[1].0, coords[1].1, pixel::pxl('#'));
                }
                engine.print(0,1, String::from("Position: [2] [4] [6] [8] ; Change point: [5]   "));
            }
            Shapes::Circle => {
                if fill {
                    engine.fill_circle(coords[0].0, coords[0].1, coords[1].0 as u32, pixel::pxl('#'));
                } else {
                    engine.circle(coords[0].0, coords[0].1, coords[1].0 as u32, pixel::pxl('#'));
                }
                engine.print(0,1, String::from("Range: [4] [6] ; Range <-> Point : [5]   "));
            }
            Shapes::Triangle => {
                if fill {
                    engine.fill_triangle(coords[0].0, coords[0].1, coords[1].0, coords[1].1, coords[2].0, coords[2].1, pixel::pxl('#'));
                } else {
                    engine.triangle(coords[0].0, coords[0].1, coords[1].0, coords[1].1, coords[2].0, coords[2].1, pixel::pxl('#'));
                }
                engine.print(0,1, String::from("Position: [2] [4] [6] [8] ; Change point: [5]   "));
            },
            Shapes::Polygon => {
                if fill {
                    engine.fill_triangle(coords[0].0, coords[0].1, coords[1].0, coords[1].1, coords[2].0, coords[2].1, pixel::pxl('#'));
                    engine.fill_triangle(coords[1].0, coords[1].1, coords[2].0, coords[2].1, coords[3].0, coords[3].1, pixel::pxl('#'));
                } else {
                    engine.triangle(coords[0].0, coords[0].1, coords[1].0, coords[1].1, coords[2].0, coords[2].1, pixel::pxl('#'));
                    engine.triangle(coords[1].0, coords[1].1, coords[2].0, coords[2].1, coords[3].0, coords[3].1, pixel::pxl('#'));
                }
                engine.print(0,1, String::from("Position: [2] [4] [6] [8] ; Change point: [5]   "));
            },
        }

        engine.print(0,0, format!("[S]hape: {:?}, [F]ill : {}   ", shape.clone(), fill));

        // display the configured coordinates and highlight the current one
        if engine.frame_count % 4 >= 2 {
            for coord in coords.iter() {
                engine.set_pxl(coord.0, coord.1, pixel::pxl_fg('#', color::Cyan));
            }
            engine.set_pxl(coords[selection].0, coords[selection].1, pixel::pxl_fg('#', color::LightYellow));
        }

        // handling coordinate displacement with a particular case for selection 1 of circle
        // because it's the range selection
        if engine.is_key_held(Key::Char('8')) || engine.is_key_pressed(Key::Up) {
            if coords[selection].1 > 0 && (selection == 0 || shape != Shapes::Circle) {
                coords[selection].1 -= 1;
            }
        }
        if engine.is_key_held(Key::Char('6')) || engine.is_key_pressed(Key::Right) {
            if coords[selection].0 < engine.get_width() as i32-1 {
                coords[selection].0 += 1;
            }
        }
        if engine.is_key_held(Key::Char('2')) || engine.is_key_pressed(Key::Down) {
            if coords[selection].1 < engine.get_height() as i32-1 && (selection == 0 || shape != Shapes::Circle) {
                coords[selection].1 += 1;
            }
        }
        if engine.is_key_held(Key::Char('4')) || engine.is_key_pressed(Key::Left) {
            if coords[selection].0 > 0 {
                coords[selection].0 -= 1;
            }
        }
        // switch between configured coordinates
        if engine.is_key_pressed(Key::Char('5')) || engine.is_key_pressed(Key::Char(' ')){
            selection = (selection+1) % coords.len();
        }
        // switch between shapes
        if engine.is_key_pressed(Key::Char('s')){
            selection = 0;
            match shape {
                Shapes::Rect =>  {
                    shape = Shapes::Circle;
                    coords = vec![(8,8),(3,2)];
                    selection = 1;
                }
                Shapes::Circle => {
                    shape = Shapes::Triangle;
                    coords = vec![(3,3),(12,6),(5,14)];
                }
                Shapes::Triangle => {
                    shape = Shapes::Polygon;
                    coords = vec![(3,3),(18,5),(5,12), (16,14)];
                }
                Shapes::Polygon => {
                    shape = Shapes::Rect;
                    coords = vec![(4,4),(18,12)];
                }
            }
        }
        // toggle fill flag
        if engine.is_key_pressed(Key::Char('f')){
            fill = !fill;
        }
    
        engine.draw(); // draw the screen
    }
}