use console_engine::screen::Screen;
use console_engine::pixel;

fn main() {
    // create a screen of 8x4 characters
    let mut scr = Screen::new(8,4);

    // draw a rectangle and print "hello, world!"
    scr.rect(0,0, 7,3,pixel::pxl('#'));
    scr.print(1,1, "Hello,");
    scr.print(1,2, "World!");

    scr.draw();
    println!();

    // the extract function returns a screen containing the provided section
    // if the section coordinates are reversed the resulting screen will also be reversed

    // extract the "hello, world!" section and print it to the terminal
    scr.extract(1,1,6,2, pixel::pxl(' ')).draw();
    println!();
    // extract the same section but in reverse and print it to the terminal
    scr.extract(6,2,1,1, pixel::pxl(' ')).draw();
    println!();
}