use console_engine::screen;
use console_engine::pixel;

fn main() {
    // create a screen of 20x11 characters
    let mut scr = screen::Screen::new(20,11);

    // draw some shapes and prints some text
    scr.rect(0,0, 19,10,pixel::pxl('#'));
    scr.fill_circle(5,5, 3, pixel::pxl('*'));
    scr.print(11,4, String::from("Hello,"));
    scr.print(11,5, String::from("World!"));

    // print the screen to the terminal
    println!("{}", scr.to_string());
}