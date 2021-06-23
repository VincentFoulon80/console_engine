use console_engine::{KeyCode, events::Event};

fn main() {
    // initializes the engine
    let mut engine = console_engine::ConsoleEngine::init(60, 3, 3).unwrap();
    let mut message = String::new();
    
    loop {
        // Poll next event
        match engine.poll() {
            // A frame has passed
            Event::Frame => {
                engine.clear_screen();
                engine.print(0,0,&message);
                engine.print(0,1, &format!("Frame: {}", engine.frame_count));
                engine.draw();
            },

            // A Key has been pressed
            Event::Key(keyevent) => {
                if keyevent.code == KeyCode::Char('q') {
                    break;
                }
                message = format!("Key: {:?}", keyevent.code);
            },

            // Mouse has been moved or clicked
            Event::Mouse(mouseevent) => {
                message = format!("Mouse: {:?} ({},{})", mouseevent.kind, mouseevent.column, mouseevent.row);
            },
            
            // Window has been resized
            Event::Resize(w, h) => {
                message = format!("Resize: {:?}, {:?}", w, h);
            },
        }
    }
}