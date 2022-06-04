use console_engine::{
    events::Event,
    forms::{FormField, FormOptions, FormValue},
    rect_style::BorderStyle,
    KeyCode,
};
use crossterm::event::KeyEvent;

fn main() {
    // initializes the engine
    let mut engine = console_engine::ConsoleEngine::init(20, 3, 10).unwrap();

    // Initialize a TextInput, that'll handle an input field into our application
    let mut f_text = console_engine::forms::Text::new(9, FormOptions::default());
    // This field is inactive by default, you need to set it active once created
    f_text.set_active(true);

    loop {
        // Poll next event
        let event = engine.poll();
        match event {
            // A frame has passed
            Event::Frame => {
                // Setup screen and border
                engine.clear_screen();
                engine.rect_border(4, 0, 14, 2, BorderStyle::new_light());

                // Print the TextInput into our screen
                engine.print_screen(5, 1, f_text.draw((engine.frame_count % 8 > 3) as usize));

                // draw the result on screen
                engine.draw();
            }

            // Manually break when the user press enter or escape
            Event::Key(KeyEvent {
                code: KeyCode::Enter | KeyCode::Esc,
                modifiers: _,
            }) => {
                break;
            }

            // Fields needs to handle events by themselves, so we pass our unhandled case to it
            event => f_text.handle_event(event),
        }
    }

    // we don't need the engine anymore, dropping it will close the fullscreen mode and bring us back to our terminal
    drop(engine);

    // Print what the user wrote
    if let FormValue::String(output) = f_text.get_output() {
        println!("You wrote: {}", output);
    }
}
