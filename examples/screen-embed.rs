use console_engine::pixel;
use console_engine::screen::Screen;
use console_engine::termion::color;

fn main()
{
    // create a screen of 21x12 characters
    let mut screen = Screen::new(21,12);
    screen.rect(0,0, 20,11, pixel::pxl('#'));
    screen.print(5,1, "main screen");

    // create a new Screen struct and draw a square inside it
    let mut my_square = Screen::new(8,8);
    my_square.rect(0,0, 7,7, pixel::pxl_fg('#', color::LightBlue));
    my_square.print(1,1, "square");

    // prints the square in the main window at a specific location
    screen.print_screen(2, 2, &my_square);
    screen.print_screen(11, 2, &my_square);

    // print the main screen on the terminal
    println!("{}", screen.to_string());
}
