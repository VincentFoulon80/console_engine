use console_engine::{
    events::Event,
    forms::{Form, FormField, FormOptions, FormStyle, FormValue, Text},
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
    let mut form = Form::new(
        12,
        6,
        FormOptions {
            style: theme,
            ..Default::default()
        },
    );
    // you either need to create your form entry directly from add_field ...
    // (We don't care about the width of our input, since it'll be resized inside the form)
    form.add_field(
        "first_name",
        Text::new(
            0,
            FormOptions {
                style: theme,
                label: Some("First Name"),
                ..Default::default()
            },
        ),
    );
    // ... or let the form build it for you
    form.build_field::<Text>(
        "last_name",
        FormOptions {
            style: theme,
            label: Some("Last Name"),
            ..Default::default()
        },
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

            // exit with Escape
            Event::Key(KeyEvent {
                code: KeyCode::Esc,
                modifiers: _,
                kind: _,
                state: _,
            }) => {
                break;
            }

            // exit with CTRL+C
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                kind: _,
                state: _,
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
        let mut first_name = String::new();
        let mut last_name = String::new();

        // Get the output of each fields
        if let Ok(FormValue::String(name)) = form.get_validated_field_output("first_name") {
            first_name = name;
        }
        if let Ok(FormValue::String(name)) = form.get_validated_field_output("last_name") {
            last_name = name;
        }

        println!("Hello, {} {}!", first_name, last_name);
    } else {
        println!("Form cancelled");
    }
}
