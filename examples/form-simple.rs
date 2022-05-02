use console_engine::{
    events::Event,
    form::{ConsoleWindow, ConsoleWindowOutput, Form},
    rect_style::BorderStyle,
    KeyCode, KeyModifiers,
};
use crossterm::event::KeyEvent;

fn main() {
    // Initialize the engine
    let mut engine = console_engine::ConsoleEngine::init(20, 8, 10).unwrap();

    // Create a new Form with two text inputs in it
    // We don't care about the width of our inputs, since they'll be resized inside the form
    let mut form = Form::new(12, 6, BorderStyle::new_light());
    form.add_field(
        "First name",
        console_engine::form::input::TextInput::new(
            0,
            console_engine::Color::Grey,
            console_engine::Color::Black,
        ),
    );
    form.add_field(
        "Last name",
        console_engine::form::input::TextInput::new(
            0,
            console_engine::Color::Grey,
            console_engine::Color::Black,
        ),
    );

    form.set_active(true);

    while !form.is_finished() {
        // Poll next event
        match engine.poll() {
            // A frame has passed
            Event::Frame => {
                engine.clear_screen();
                engine.print_screen(5, 1, form.draw((engine.frame_count % 8 > 3) as usize));
                engine.draw();
            }

            // A Key has been pressed
            Event::Key(keyevent) => {
                let KeyEvent { code, modifiers } = keyevent;
                match code {
                    KeyCode::Esc => {
                        break;
                    }
                    KeyCode::Char(c) => {
                        if modifiers == KeyModifiers::CONTROL && c == 'C' {
                            break;
                        }
                    }
                    _ => {}
                }

                form.handle_event(&Event::Key(keyevent))
            }

            _ => {}
        }
    }

    // we don't need the engine anymore, dropping it will close the fullscreen mode and bring us back to our terminal
    drop(engine);

    if form.is_finished() {
        let mut first_name = String::new();
        let mut last_name = String::new();

        if let ConsoleWindowOutput::String(name) = form.get_result("First name").unwrap_or_default()
        {
            first_name = name;
        }
        if let ConsoleWindowOutput::String(name) = form.get_result("Last name").unwrap_or_default()
        {
            last_name = name;
        }

        println!("Hello, {} {}!", first_name, last_name);
    } else {
        println!("Form cancelled");
    }
}
