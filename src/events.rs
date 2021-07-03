use crossterm::event::{KeyEvent, MouseEvent};

/// # ConsoleEngine events
///
/// You can poll events with the `engine.poll` function.
/// You may want to match the event to act according to which one has been returned
///
/// See example `events`
pub enum Event {
    /// A frame has passed. You can either call `engine.draw()` or do nothing.
    Frame,
    /// A Key has been pressed.
    /// You can check which one and if a modifier has been pressed as well.
    Key(KeyEvent),
    /// The Mouse has been moved, or clicked.
    /// You can check which event occured and if a modifier has been pressed as well.
    Mouse(MouseEvent),
    /// The window has been resized.
    Resize(u16, u16),
}
