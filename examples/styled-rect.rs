use console_engine::rect_style::BorderStyle;
use console_engine::screen;

fn main() {
    let mut scr = screen::Screen::new(9, 10);

    scr.rect_border(0, 0, 3, 2, BorderStyle::new_simple());
    scr.rect_border(0, 3, 3, 5, BorderStyle::new_light());
    scr.rect_border(4, 0, 7, 2, BorderStyle::new_heavy());
    scr.rect_border(4, 3, 7, 5, BorderStyle::new_double());

    // print the screen to the terminal
    scr.draw();
}
