use console_engine::{
    events::Event,
    forms::{ConsoleForm, FormOutput},
    rect_style::BorderStyle,
    KeyCode,
};

fn main() {
    // initializes the engine
    let mut engine = console_engine::ConsoleEngine::init(20, 3, 10).unwrap();
    let mut f_text = console_engine::forms::TextInput::new(9, None, None);

    f_text.set_active(true);

    loop {
        // Poll next event
        match engine.poll() {
            // A frame has passed
            Event::Frame => {
                engine.clear_screen();
                engine.rect_border(4, 0, 14, 2, BorderStyle::new_light());
                engine.print_screen(5, 1, f_text.draw((engine.frame_count % 8 > 3) as usize));
                engine.draw();
            }

            // A Key has been pressed
            Event::Key(keyevent) => {
                if keyevent.code == KeyCode::Char('q') {
                    break;
                }

                if keyevent.code == KeyCode::Enter {
                    break;
                }

                f_text.handle_event(&Event::Key(keyevent))
            }

            _ => {}
        }
    }

    // we don't need the engine anymore, dropping it will close the fullscreen mode and bring us back to our terminal
    drop(engine);

    if let FormOutput::String(output) = f_text.get_output() {
        println!("You wrote: {}", output);
    }
}
