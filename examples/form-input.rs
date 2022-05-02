use console_engine::{events::Event, form::ConsoleWindow, rect_style::BorderStyle, KeyCode};

fn main() {
    // initializes the engine
    let mut engine = console_engine::ConsoleEngine::init(20, 3, 10).unwrap();
    let mut f_input = console_engine::form::input::TextInput::new(
        9,
        console_engine::Color::Grey,
        console_engine::Color::Black,
    );

    f_input.set_active(true);

    loop {
        // Poll next event
        match engine.poll() {
            // A frame has passed
            Event::Frame => {
                engine.clear_screen();
                engine.rect_border(4, 0, 14, 2, BorderStyle::new_light());
                engine.print_screen(5, 1, f_input.draw((engine.frame_count % 8 > 3) as usize));
                engine.draw();
            }

            // A Key has been pressed
            Event::Key(keyevent) => {
                if keyevent.code == KeyCode::Char('q') {
                    break;
                }

                f_input.handle_event(&Event::Key(keyevent))
            }

            _ => {}
        }
    }
}
