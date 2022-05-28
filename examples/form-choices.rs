use std::collections::HashMap;

use console_engine::{
    events::Event,
    forms::{Checkbox, Form, FormField, FormOptions, FormStyle, FormValue, Radio},
    rect_style::BorderStyle,
    ConsoleEngine, KeyCode, KeyModifiers,
};
use crossterm::event::KeyEvent;

fn main() {
    // Initialize the engine
    let mut engine = ConsoleEngine::init(30, 8, 10).unwrap();

    // Define a theme for the form
    let theme = FormStyle {
        border: Some(BorderStyle::new_light()),
        ..Default::default()
    };

    // Create a new Form with two text inputs in it
    let mut form = Form::new(
        28,
        6,
        FormOptions {
            style: theme,
            ..Default::default()
        },
    );

    let check_choices = vec![
        String::from("First"),
        String::from("Second"),
        String::from("Third"),
    ];

    form.build_field::<Checkbox>(
        "checkbox",
        FormOptions {
            style: theme,
            label: Some("Please select something"),
            custom: HashMap::from([(
                String::from("choices"),
                FormValue::List(check_choices.clone()),
            )]),
            ..Default::default()
        },
    );
    form.build_field::<Radio>(
        "radio",
        FormOptions {
            style: theme,
            label: Some("Do you enjoy this demo?"),
            custom: HashMap::from([(
                String::from("choices"),
                FormValue::List(vec![String::from("Yes"), String::from("No")]),
            )]),
            ..Default::default()
        },
    );

    form.set_active(true);

    while !form.is_finished() {
        // Poll next event
        let event = engine.poll();
        // Make the form handle the event
        form.handle_event(&event);
        match event {
            // A frame has passed
            Event::Frame => {
                engine.clear_screen();
                engine.print_screen(1, 1, form.draw((engine.frame_count % 8 > 3) as usize));
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
            }

            _ => {}
        }
    }

    // we don't need the engine anymore, dropping it will close the fullscreen mode and bring us back to our terminal
    drop(engine);

    if form.is_finished() {
        // Get the output of each fields
        if let FormValue::Vec(selection_list) = form.get_result("checkbox").unwrap_or_default() {
            if selection_list.is_empty() {
                println!("You selected nothing!");
            } else {
                println!(
                    "You selected: {:?}",
                    selection_list
                        .iter()
                        .map(|x| {
                            if let FormValue::Index(id) = x {
                                check_choices[*id].clone()
                            } else {
                                check_choices[0].clone()
                            }
                        })
                        .collect::<Vec<String>>()
                );
            }
        }
        if let FormValue::Index(selection) = form.get_result("radio").unwrap_or_default() {
            if selection == 0 {
                println!("Glad you enjoyed this demo!");
            } else {
                println!("Too bad you didn't enjoy this demo...");
            }
        }
    } else {
        println!("See you later!");
    }
}
