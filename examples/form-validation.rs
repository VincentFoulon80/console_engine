use console_engine::{
    events::Event,
    forms::{constraints, Form, FormField, FormOptions, FormOutput, FormStyle, TextInput},
    rect_style::BorderStyle,
    ConsoleEngine, KeyCode, KeyModifiers,
};
use crossterm::event::KeyEvent;

fn main() {
    // Initialize the engine
    let mut engine = ConsoleEngine::init(40, 8, 10).unwrap();

    // Define a theme for the form
    let theme = FormStyle {
        border: Some(BorderStyle::new_light()),
        ..Default::default()
    };

    // Create a new Form with two text inputs in it
    let mut form = Form::new(30, 6, theme, None);

    // Build a TextInput field with a NotBlank and Number constraints
    form.build_field::<TextInput>(
        "number",
        Some(FormOptions {
            label: Some("Please input a number"),
            constraints: vec![
                constraints::NotBlank::new("There is nothing here!"),
                constraints::Number::new("This is not a number"),
            ],
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
        if form.is_valid() {
            let mut number = 0;

            // Retrieve the output of the TextInput
            if let FormOutput::String(num) = form.get_result("number").unwrap_or_default() {
                number = num.parse::<i32>().unwrap_or(0);
            }
            println!("Double of your number is {}", number * 2);
        } else {
            println!("{:?}", form.get_error("number").unwrap())
        }
    } else {
        println!("Form cancelled");
    }
}
