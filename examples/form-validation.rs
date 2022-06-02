use std::error::Error;

use console_engine::{
    events::Event,
    forms::{constraints, Form, FormError, FormField, FormOptions, FormStyle, FormValue, Text},
    rect_style::BorderStyle,
    ConsoleEngine, KeyCode, KeyModifiers,
};
use crossterm::event::KeyEvent;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the engine
    let mut engine = ConsoleEngine::init(40, 8, 10)?;

    // Define a theme for the form
    let theme = FormStyle {
        border: Some(BorderStyle::new_light()),
        ..Default::default()
    };

    // Create a new Form with two text inputs in it
    let mut form = Form::new(
        30,
        6,
        FormOptions {
            style: theme,
            ..Default::default()
        },
    );

    // Build a TextInput field with a NotBlank and Number constraints
    form.build_field::<Text>(
        "number",
        FormOptions {
            style: theme,
            label: Some("Please input a number"),
            constraints: vec![
                constraints::NotBlank::new("There is nothing here!"),
                constraints::Number::new("This is not a number"),
            ],
            ..Default::default()
        },
    );

    form.set_active(true);

    while !form.is_finished() {
        match engine.poll() {
            // A frame has passed
            Event::Frame => {
                engine.clear_screen();
                engine.print_screen(5, 1, form.draw((engine.frame_count % 8 > 3) as usize));
                engine.draw();
            }

            // exit with Escape
            Event::Key(KeyEvent {
                code: KeyCode::Esc,
                modifiers: _,
            }) => {
                break;
            }
            // exit with CTRL+C
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            }) => {
                break;
            }
            // Let the form handle the unhandled events
            event => form.handle_event(event),
        }
    }

    // we don't need the engine anymore, dropping it will close the fullscreen mode and bring us back to our terminal
    drop(engine);

    if form.is_finished() {
        // Retrieve the output of the Text field
        match form.get_result("number") {
            Ok(FormValue::String(num)) => {
                // num.parse::<f32>() is garanteed to be valid since the field has a Number constraint
                // and at this point the validator has checked the value
                println!("Double of your number is {}", num.parse::<f32>()? * 2f32)
            }
            Ok(_) => unreachable!(), // we know that Text fields always output a String
            Err(FormError::FieldNotFound) => unreachable!(), // we know that "number" exists
            Err(FormError::ValidationFailed(errors)) => println!("{:?}", errors),
        }
        // Alternative version
        // globally check if your form is valid
        if form.is_valid() {
            // Retrieve the output of the Text field
            // note that get_result_unvalidated just outputs an Option and does not validate (obviously) the field
            // Here it's safe to parse the value since we're in the form.is_valid() block,
            // but be careful when using this function
            let number = if let FormValue::String(num) =
                form.get_result_unvalidated("number").unwrap_or_default()
            {
                num.parse::<f32>().unwrap_or(0f32)
            } else {
                0f32
            };
            println!("Triple of your number is {}", number * 3f32);
        }
    } else {
        println!("Form cancelled");
    }
    Ok(())
}
