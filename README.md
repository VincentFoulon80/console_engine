# Console Engine

[Changelog](https://github.com/VincentFoulon80/console_engine/releases)

**Note: Code reviewer needed ! see [this issue](https://github.com/VincentFoulon80/console_engine/issues/1) for more informations**

This library provides simple features for handling user's input and display for terminal applications.  
Besides the user input and display, this library also provides some tools to build standalone "screens" that can be used as simply as printing it.

It uses [Termion](https://crates.io/crates/termion) as main tool for handling the screen and inputs. You don't have to worry about initalizing anything because the lib will handle this for you.

## Features

- Build custom terminal display using shapes or text
- Terminal handling with a target frame per seconds
- Keyboard and mouse support
- Terminal resizing support
- You are not interested by keyboard/mouse handling, even terminal handling ? You can still build "screens" to just draw using `println!()`

## Platforms

Works for Linux and possibly Mac (need confirmation).  

Windows support will be available as soon as termion will support it [See here for more info](https://gitlab.redox-os.org/redox-os/termion/-/merge_requests/151)
For now, you can change the cargo.toml termion dependency by this :
```toml
termion = { git = "https://gitlab.redox-os.org/Jezza/termion", branch = "windows-support", package = "termion"}
```
Note: window's input initialization requires the user to first press enter. ConsoleEngine will ask the user to press Enter while inializing.

# example usage 
```rust
use console_engine::pixel;
use console_engine::termion::color;
use console_engine::termion::event::Key;

fn main() {
    // initializes a screen of 20x10 characters with a target of 3 frames per second
    // coordinates will range from [0,0] to [19,9]
    let mut engine = console_engine::ConsoleEngine::init(20, 10, 3);
    let value = 14;
    // main loop, be aware that you'll have to break it because ctrl+C is captured
    loop {
        engine.wait_frame(); // wait for next frame + capture inputs
        engine.clear_screen(); // reset the screen
    
        engine.line(0, 0, 19, 9, pixel::pxl('#')); // draw a line of '#' from [0,0] to [19,9]
        engine.print(0, 4, format!("Result: {}", value)); // prints some value at [0,4]
    
        engine.set_pxl(4, 0, pixel::pxl_fg('O', color::Cyan)); // write a majestic cyan 'O' at [4,0]

        if engine.is_key_pressed(Key::Char('q')) { // if the user presses 'q' :
            break; // exits app
        }
    
        engine.draw(); // draw the screen
    }
}
```

# Documentation

Take a look at the [generated documentation](https://docs.rs/console_engine/).

# Examples

See [examples](https://github.com/VincentFoulon80/console_engine/tree/master/examples) :
- **graph** : Display a graph being generated with some values.
- **snake** : A simple game of snake.
- **lines** : Draw random lines of random colors on the screen.
- **lines-fps** : same example as lines, but with a FPS counter.
- **shapes** : Shape's functions testing tool
- **mouse** : Simple mouse clicking test
- **drag-and-drop** : Move a rectangle with your mouse
- **screen-simple** : Example usage of Screen struct instead of ConsoleEngine
- **screen-embed** : Example usage of Screen's `print_screen` function to embed one screen into another
- **tetris** : A game of Tetris
