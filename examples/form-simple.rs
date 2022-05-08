use console_engine::{
    events::Event,
    forms::{Form, FormField, FormOptions, FormOutput, FormStyle, TextInput},
    rect_style::BorderStyle,
    ConsoleEngine, KeyCode, KeyModifiers,
};
use crossterm::event::KeyEvent;

fn main() {
    // Initialize the engine
    let mut engine = ConsoleEngine::init(20, 8, 10).unwrap();

    // Define a theme for the form
    let theme = FormStyle {
        border: Some(BorderStyle::new_light()),
        ..Default::default()
    };

    // Create a new Form with two text inputs in it
    let mut form = Form::new(12, 6, theme, None);
    // you either need to create your form entry directly from add_field ...
    // (We don't care about the width of our input, since it'll be resized inside the form)
    form.add_field(
        "first_name",
        TextInput::new(
            0,
            Some(FormOptions {
                label: Some("First Name"),
                constraints: vec![],
            }),
            Some(theme),
        ),
    );
    // ... or let the form build it for you
    form.build_field::<TextInput>(
        "last_name",
        Some(FormOptions {
            label: Some("Last Name"),
            constraints: vec![],
        }),
        Some(theme),
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
                        // exit with Escape
                        break;
                    }
                    KeyCode::Char(c) => {
                        if modifiers == KeyModifiers::CONTROL && c == 'c' {
                            // exit with CTRL+C
                            break;
                        }
                    }
                    _ => {}
                }
                // Make the form handle the key event
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

        // Get the output of each fields
        if let FormOutput::String(name) = form.get_result("first_name").unwrap_or_default() {
            first_name = name;
        }
        if let FormOutput::String(name) = form.get_result("last_name").unwrap_or_default() {
            last_name = name;
        }

        println!("Hello, {} {}!", first_name, last_name);
    } else {
        println!("Form cancelled");
    }
}
