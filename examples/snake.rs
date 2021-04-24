use console_engine::pixel;
use console_engine::Color;
use console_engine::ConsoleEngine;
use console_engine::KeyCode;

/// custom function for generating a random u32 bound into [0;max[
fn random(max: u32) -> u32 {
    rand::random::<u32>() % max
}

/// Direction the snake can face
enum Direction {
    North,
    East,
    South,
    West,
}

/// Snake structure :  
/// The game logic fits in it
struct Snake {
    playing: bool,
    bound_w: u32,
    bound_h: u32,
    direction: Direction,
    old_dx: i8,
    old_dy: i8,
    pos_x: u32,
    pos_y: u32,
    apple_x: u32,
    apple_y: u32,
    body: Vec<(u32, u32)>,
}

impl Snake {
    /// Game initialization
    pub fn init(game_width: u32, game_height: u32) -> Snake {
        Snake {
            playing: false,
            bound_w: game_width,
            bound_h: game_height,
            direction: Direction::East,
            old_dx: 1, // start condition should be 1 due to starting direction being East
            old_dy: 0,
            pos_x: 4,
            pos_y: 4,
            apple_x: 0,
            apple_y: 0,
            body: vec![(3, 4), (2, 4)],
        }
    }

    /// Generates an apple in the board
    fn gen_apple(&mut self) {
        let mut count_fallback = 0;
        loop {
            // randomly get coordinates
            let x = random(self.bound_w);
            let y = random(self.bound_h);

            // check if the coordinates aren't colliding with the snake's body
            // sets the position if no collision
            if !self.body.contains(&(x, y)) {
                self.apple_x = x;
                self.apple_y = y;
                return;
            }
            count_fallback += 1;
            // if 50 tries did not succeed
            if count_fallback > 50 {
                // bruteforce the first available position
                for y in 0..self.bound_h {
                    for x in 0..self.bound_w {
                        if !self.body.contains(&(x, y)) {
                            self.apple_x = x;
                            self.apple_y = y;
                            return;
                        }
                    }
                }
                // if bruteforce failed, game has been won
                self.playing = false;
                return;
            }
        }
    }

    pub fn input(&mut self, engine: &ConsoleEngine) {
        if self.playing {
            // Change snake's direction based on a keypad layout
            if engine.is_key_pressed(KeyCode::Char('8')) || engine.is_key_pressed(KeyCode::Up) {
                self.direction = Direction::North;
            }
            if engine.is_key_pressed(KeyCode::Char('6')) || engine.is_key_pressed(KeyCode::Right) {
                self.direction = Direction::East;
            }
            if engine.is_key_pressed(KeyCode::Char('2')) || engine.is_key_pressed(KeyCode::Down) {
                self.direction = Direction::South;
            }
            if engine.is_key_pressed(KeyCode::Char('4')) || engine.is_key_pressed(KeyCode::Left) {
                self.direction = Direction::West;
            }
        } else {
            // check when the player starts the game with space
            if engine.is_key_pressed(KeyCode::Char(' ')) {
                // Initialize game values to a starting state
                self.playing = true;
                self.direction = Direction::East;
                self.pos_x = 4;
                self.pos_y = 4;
                self.body = vec![(3, 4), (2, 4)];
                self.gen_apple();
            }
        }
    }

    pub fn update_position(&mut self) {
        if self.playing {
            // calculates the delta_x and delta_y
            // based on facing direction
            let mut dx = 0;
            let mut dy = 0;
            match self.direction {
                Direction::North => dy = -1,
                Direction::East => dx = 1,
                Direction::South => dy = 1,
                Direction::West => dx = -1,
            }

            // checks to see if old inputed direction overlaps with actual inputed direction
            // such as East then West.. This would cause the game to think that the snake collided
            // with itself causing a gameover >>
            // if dx's or dy's are opposites then continue moving in old direction
            if self.old_dx + dx == 0 || self.old_dy + dy == 0 {
                dx = self.old_dx;
                dy = self.old_dy;
            } else {
                self.old_dx = dx;
                self.old_dy = dy;
            }

            // if the snake collides with top and left boundaries, game over
            // this check need to be made first to bypass an underflowing
            if self.pos_x == 0 && dx == -1 || self.pos_y == 0 && dy == -1 {
                self.playing = false;
                return;
            }

            // calculate new position, can't underflow because of the check above
            let new_pos = (
                (self.pos_x as i32 + dx as i32) as u32,
                (self.pos_y as i32 + dy as i32) as u32,
            );

            // if collide with bottom and right boundaries, game over
            if new_pos.0 >= self.bound_w || new_pos.1 >= self.bound_h {
                self.playing = false;
                return;
            }

            // if collide with own tail, game over
            if self.body.contains(&new_pos) {
                self.playing = false;
                return;
            }

            // if collide with apple, add a new segment in snake's body
            // and generate a new apple
            if new_pos == (self.apple_x, self.apple_y) {
                self.body.insert(0, (self.pos_x, self.pos_y));
                self.gen_apple();
            }

            // if still alive, move the body
            if self.playing {
                self.body.insert(0, (self.pos_x, self.pos_y));
                self.pos_x = new_pos.0;
                self.pos_y = new_pos.1;
                self.body.pop();
            }
        }
    }

    pub fn draw(&self, engine: &mut ConsoleEngine) {
        if self.playing {
            // draw apple
            engine.set_pxl(
                self.apple_x as i32,
                self.apple_y as i32,
                pixel::pxl_fg('O', Color::Red),
            );
            // draw snake's body
            for segment in self.body.iter() {
                engine.set_pxl(
                    segment.0 as i32,
                    segment.1 as i32,
                    pixel::pxl_fg('█', Color::Green),
                );
            }
            // don't forget snake's head !
            engine.set_pxl(
                self.pos_x as i32,
                self.pos_y as i32,
                pixel::pxl_fg('█', Color::DarkGreen),
            )
        } else {
            // blink a message, inviting the player to press space
            // and display controls on the other side
            if engine.frame_count % 8 >= 4 {
                engine.print_fbg(2, 1, "Press", Color::Yellow, Color::Black);
                engine.print_fbg(2, 2, "Space", Color::Yellow, Color::Black);
                engine.print_fbg(3, 3, "To", Color::Yellow, Color::Black);
                engine.print_fbg(2, 4, "Play", Color::Yellow, Color::Black);
            } else {
                engine.print(4, 1, "8");
                engine.print(4, 2, "^");
                engine.print(1, 3, "4 < > 6");
                engine.print(4, 4, "v");
                engine.print(4, 5, "2");
            }
            // score is always displayed
            engine.print(1, 8, format!("Score:{}", self.body.len() - 2).as_str());
        }
    }
}

fn main() {
    // initializes a screen filling the terminal of at least 10x10 of size with a target of 4 frame per second
    let mut engine = console_engine::ConsoleEngine::init_fill_require(10, 10, 4).unwrap();

    // initialize game here, providing term size as boundaries
    let mut snake = Snake::init(engine.get_width(), engine.get_height());

    // main loop, be aware that you'll have to break it because ctrl+C is captured
    loop {
        engine.wait_frame(); // wait for next frame + capture inputs
                             // engine.check_resize(); here we do not want to resize the terminal because it could break the boundaries of the game

        // exit check
        if engine.is_key_pressed(KeyCode::Char('q')) {
            break;
        }
        engine.clear_screen(); // reset the screen

        // run the game
        snake.input(&engine);
        snake.update_position();
        // draw the game in engine's screen
        snake.draw(&mut engine);

        engine.draw(); // draw the screen
    }
}
