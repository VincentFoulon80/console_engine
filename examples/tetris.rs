use console_engine::{ConsoleEngine, pixel};
use console_engine::screen::Screen;
use console_engine::termion::color;
use console_engine::termion::event::Key;

/// This function returns a random tetromino
fn random_tetromino() -> Tetromino {
    let num = rand::random::<u32>() % 7;
    match num {
        0 => Tetromino::I,
        1 => Tetromino::O,
        2 => Tetromino::T,
        3 => Tetromino::J,
        4 => Tetromino::L,
        5 => Tetromino::S,
        6 => Tetromino::Z,
        _ => Tetromino::O // will never happen
    }
}

// Tetromino's orientations
enum Orientation {
    Normal,
    Quarter,
    Half,
    ThreeQuarters
}
impl Orientation {
    // pub fn turn_left(&mut self) -> Orientation {
    //     match self {
    //         Orientation::Normal => Orientation::ThreeQuarters,
    //         Orientation::Quarter => Orientation::Normal,
    //         Orientation::Half => Orientation::Quarter,
    //         Orientation::ThreeQuarters => Orientation::Half
    //     }
    // }
    pub fn turn_right(&mut self) -> Orientation {
        match self {
            Orientation::Normal => Orientation::Quarter,
            Orientation::Quarter => Orientation::Half,
            Orientation::Half => Orientation::ThreeQuarters,
            Orientation::ThreeQuarters => Orientation::Normal
        }
    }
}

/// Tetrominos enum
/// see wikipedia's page for more informations about tetrominos
/// https://en.wikipedia.org/wiki/Tetromino
enum Tetromino {
    I,
    O,
    T,
    J,
    L,
    S,
    Z
}
impl Tetromino {
    /// Returns a screen of 4x4 containing the tetromino
    /// We use dots to fill the screen for a more easy-to-read code
    /// these dots will be ignored in game because we use print_screen_alpha
    /// Additionally, we consider position 1,1 to be the center of rotation
    pub fn get_scr(&self, orientation: &Orientation) -> Screen
    {
        match self {
            Tetromino::I => {
                match orientation {
                    Orientation::Normal | Orientation::Half
                    => Screen::from_string(String::from(".█..\
                                                         .█..\
                                                         .█..\
                                                         .█.."), color::Cyan, color::Black, 4,4),

                    Orientation::Quarter | Orientation::ThreeQuarters 
                    => Screen::from_string(String::from("....\
                                                         ████\
                                                         ....\
                                                         ...."), color::Cyan, color::Black, 4,4)
                }
            },
            Tetromino::O => Screen::from_string(String::from("....\
                                                              .██.\
                                                              .██.\
                                                              ...."), color::LightYellow, color::Black, 4,4),
            Tetromino::T => {
                match orientation {
                    Orientation::Normal 
                    => Screen::from_string(String::from("....\
                                                         ███.\
                                                         .█..\
                                                         ...."), color::Magenta, color::Black, 4,4),

                    Orientation::Quarter 
                    => Screen::from_string(String::from(".█..\
                                                         ██..\
                                                         .█..\
                                                         ...."), color::Magenta, color::Black, 4,4),
                    
                    Orientation::Half 
                    => Screen::from_string(String::from(".█..\
                                                         ███.\
                                                         ....\
                                                         ...."), color::Magenta, color::Black, 4,4),

                    Orientation::ThreeQuarters 
                    => Screen::from_string(String::from(".█..\
                                                         .██.\
                                                         .█..\
                                                         ...."), color::Magenta, color::Black, 4,4),
                }
            },
            Tetromino::J => {
                match orientation {
                    Orientation::Normal 
                    => Screen::from_string(String::from(".█..\
                                                         .█..\
                                                         ██..\
                                                         ...."), color::Blue, color::Black, 4,4),

                    Orientation::Quarter 
                    => Screen::from_string(String::from("█...\
                                                         ███.\
                                                         ....\
                                                         ...."), color::Blue, color::Black, 4,4),
                    
                    Orientation::Half 
                    => Screen::from_string(String::from(".██.\
                                                         .█..\
                                                         .█..\
                                                         ...."), color::Blue, color::Black, 4,4),

                    Orientation::ThreeQuarters 
                    => Screen::from_string(String::from("....\
                                                         ███.\
                                                         ..█.\
                                                         ...."), color::Blue, color::Black, 4,4),
                }
            },
            Tetromino::L => {
                match orientation {
                    Orientation::Normal 
                    => Screen::from_string(String::from(".█..\
                                                         .█..\
                                                         .██.\
                                                         ...."), color::Yellow, color::Black, 4,4),

                    Orientation::Quarter 
                    => Screen::from_string(String::from("....\
                                                         ███.\
                                                         █...\
                                                         ...."), color::Yellow, color::Black, 4,4),
                    
                    Orientation::Half 
                    => Screen::from_string(String::from("██..\
                                                         .█..\
                                                         .█..\
                                                         ...."), color::Yellow, color::Black, 4,4),

                    Orientation::ThreeQuarters 
                    => Screen::from_string(String::from("..█.\
                                                         ███.\
                                                         ....\
                                                         ...."), color::Yellow, color::Black, 4,4),
                }
            },
            Tetromino::S => {
                match orientation {
                    Orientation::Normal | Orientation::Half
                    => Screen::from_string(String::from("....\
                                                         .██.\
                                                         ██..\
                                                         ...."), color::Green, color::Black, 4,4),

                    Orientation::Quarter | Orientation::ThreeQuarters
                    => Screen::from_string(String::from("█...\
                                                         ██..\
                                                         .█..\
                                                         ...."), color::Green, color::Black, 4,4),
                }
            },
            Tetromino::Z => {
                match orientation {
                    Orientation::Normal | Orientation::Half
                    => Screen::from_string(String::from("....\
                                                         ██..\
                                                         .██.\
                                                         ...."), color::Red, color::Black, 4,4),

                    Orientation::Quarter | Orientation::ThreeQuarters
                    => Screen::from_string(String::from(".█..\
                                                         ██..\
                                                         █...\
                                                         ...."), color::Red, color::Black, 4,4),
                }
            }
        }
    }
}

/// This function checks if a given piece fits into the game screen.
/// By "fits" we mean that the piece does not overlap already fixed blocks on the screen
/// return true if the piece fits
fn piece_fits(game_scr: &Screen, piece: &Tetromino, piece_r: &Orientation, x: i32, y: i32) -> bool
{
    // get the piece's screen (4x4)
    let piece_scr = piece.get_scr(piece_r);
    // look trough each "pixel" of the screen
    for j in 0..4 {
        for i in 0..4 {
            // if the piece's character is not a dot (see Tetromino's `get_scr` rustdoc)
            if piece_scr.get_pxl(i,j).unwrap().chr != '.' {
                // we check if the pixel's position overlaps a block in the game screen
                if game_scr.get_pxl(x+i-1,y+j-1).is_err() || game_scr.get_pxl(x+i-1, y+j-1).unwrap().chr != ' ' {
                    return false;
                }
            }
        }
    }
    true
}

/// This function will check every lines in the game screen to see
/// if there are filled ones.
/// When a line is full, every blocks above will fall down
/// Returns the number of full lines encountered
fn check_and_remove_lines(game_scr: &mut Screen) -> u32
{
    let mut lines_found = 0;
    // look through each lines of the game screen
    for j in 0..game_scr.get_height() as i32 {
        let mut line_count = 0;
        // for each lines, count the number of pixels that are not empty
        for i in 0..game_scr.get_width() as i32 {
            if game_scr.get_pxl(i,j).unwrap().chr != ' ' {
                line_count += 1;
            }
        }
        // if the line is full
        if line_count == game_scr.get_width() {
            // count up the found counter and make the above block fall
            lines_found += 1;
            for rev_j in (0..j).rev() {
                for x in 0..game_scr.get_width() as i32 {
                    game_scr.set_pxl(x, rev_j+1, game_scr.get_pxl(x,rev_j).unwrap());
                }
            }
        }
    }
    lines_found
}

/// Draw the game
/// Be aware that we drawed the interface (walls, borders, ...) once at the start of the game, 
/// and since they will never change, we don't need to draw them on every frame
fn draw_game(engine: &mut ConsoleEngine, game_scr: &Screen, piece: &Tetromino, piece_x : i32, piece_y : i32, piece_r: &Orientation, next_piece: &Tetromino, score: u32)
{
    // print the game screen
    engine.print_screen(1,0, &game_scr);
    // print the current piece, coordinates are corrected to match the game_screen and the piece center
    engine.print_screen_alpha(piece_x,piece_y-1,&piece.get_scr(&piece_r), '.');
    // print next piece in its frame
    engine.print_screen_alpha(game_scr.get_width() as i32+4, 5, &next_piece.get_scr(&Orientation::Normal), '.');
    // print the score
    engine.print(game_scr.get_width() as i32+3, 1, format!("{}", score).as_str());
}


fn main() {
    // initializes engine
    let mut engine = ConsoleEngine::init(40, 22, 10);

    // initalizes game screen + game interface
    let mut game_scr = Screen::new(10,20);
    let game_w = game_scr.get_width() as i32;
    engine.rect(0,0,game_w+1,game_scr.get_height() as i32, pixel::pxl('█')); // walls
    engine.rect(game_w+1,0,engine.get_width() as i32-1, 2, pixel::pxl('█')); // score's border
    engine.print_fbg(game_w +3, 0, "Score:", color::Black, color::White);
    engine.rect(game_w+3, 4, game_w+8, 9, pixel::pxl('█')); // next piece's border
    engine.print_fbg(game_w + 4, 4, "Next", color::Black, color::White);

    // constant values
    let start_pos_x = game_scr.get_width() as i32/2-1;
    let start_pos_y = 1;
    let start_fall_counter = 10;

    // initializes game
    let mut piece = random_tetromino();
    let mut next_piece = random_tetromino();
    let mut piece_x = start_pos_x;
    let mut piece_y = start_pos_y;
    let mut piece_r = Orientation::Normal;
    let mut fall_counter = start_fall_counter;
    let mut score: u32 = 0;

    
    // main loop, be aware that you'll have to break it because ctrl+C is captured
    loop {
        engine.wait_frame(); // wait for next frame + capture inputs
        if engine.is_key_pressed(Key::Char('q')) { // if the user presses 'q' :
            break; // exits app
        }

        // rotate the piece
        if (engine.is_key_held(Key::Char('8')) || engine.is_key_held(Key::Up)) && piece_fits(&game_scr, &piece, &piece_r.turn_right(), piece_x, piece_y) {
            piece_r = piece_r.turn_right();
        }
        // move the piece left or right
        if (engine.is_key_held(Key::Char('4')) || engine.is_key_held(Key::Left)) && piece_x > 0 && piece_fits(&game_scr, &piece, &piece_r, piece_x-1, piece_y) {
            piece_x -= 1;
        }
        if (engine.is_key_held(Key::Char('6')) || engine.is_key_held(Key::Right)) && piece_x < game_w && piece_fits(&game_scr, &piece, &piece_r, piece_x+1, piece_y) {
            piece_x += 1;
        }
        // force the piece to drop
        if engine.is_key_held(Key::Char('2')) || engine.is_key_held(Key::Down) {
            fall_counter = 1;
        }

        fall_counter -= 1;
        // drop the piece if the fall_counter hits zero
        if fall_counter == 0 {
            // check if the piece can fall
            if piece_fits(&game_scr, &piece, &piece_r, piece_x, piece_y+1) {
                piece_y += 1
            } else {
                // the piece can't fall.
                // he is now fixed in place
                // we print the piece in the game screen
                game_scr.print_screen_alpha(piece_x-1, piece_y-1, &piece.get_scr(&piece_r), '.');
                // check if a line has been completed
                let lines = check_and_remove_lines(&mut game_scr);
                if lines > 0 {
                    // update the score
                    score += 100*(lines*lines);
                }
                // update the score, create a new piece and reset the position
                score += 4;
                piece_x = start_pos_x;
                piece_y = start_pos_y;
                piece_r = Orientation::Normal;
                piece = next_piece;
                next_piece = random_tetromino();
                // refresh the next piece's border
                engine.fill_rect(game_w+4, 5, game_w+7, 8, pixel::pxl(' '));
                // check if the starting position is not empty
                if !piece_fits(&game_scr, &piece, &piece_r, piece_x, piece_y) {
                    // game over, we draw the game one last time
                    draw_game(&mut engine, &game_scr, &piece, piece_x, piece_y, &piece_r, &next_piece, score);

                    engine.rect(game_w+3, 11, game_w+13, 13, pixel::pxl('█'));
                    engine.print_fbg(game_w+4, 12, "GAME OVER", color::Black, color::White);

                    // wait 20 frames (=2 seconds) while still drawing the game
                    for _ in 0..20 {
                        engine.wait_frame();
                        engine.draw();
                    }
                    break; // exit the game
                }
            }
            // reset the fall_counter to its start value.
            // the more score the player have, the less we set the frame_counter
            // that makes the pieces drop faster
            fall_counter = start_fall_counter-std::cmp::min(start_fall_counter-3, score/1000);
        }

        // draw the game
        draw_game(&mut engine, &game_scr, &piece, piece_x, piece_y, &piece_r, &next_piece, score);
        engine.draw(); // draw the screen
    }
}