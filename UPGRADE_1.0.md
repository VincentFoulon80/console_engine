# Removed Deprecations

- `engine.scr_w()` : use `engine.get_width()` instead
- `engine.scr_h()` : use `engine.get_height()` instead

# From 0.7.x

- Replace :
  ```rust
  use console_engine::termion::color;
  use console_engine::termion::event::Key;
  use console_engine::termion::event::MouseButton;
  ```
  by :
  ```rust
  use console_engine::Color;
  use console_engine::KeyCode;
  use console_engine::MouseButton;
  ```
- in your code, `Key::Char('*')` becomes `KeyCode::Char('*')`
- `color` becomes `Color`.
  Be aware that the Color enum has changed so instead of "Yellow" you have "DarkYellow", and "LightYellow" now is "Yellow".  
  Black and White variants doesn't exist anymore because of the new "Grey" and "DarkGrey".
- `engine.get_mouse_held()` and `engine.get_mouse_released()` now require that you provide a `MouseButton` as first parameter
- `screen.to_string()` as well as `pixel.to_string()` no longer exists.
  If you want to print a screen you'll now use `screen.draw()` instead.
- `pixel.colors` has been replaced by `pixel.fg` and `pixel.bg`. These two are now of type Color instead of String.

# From 0.6.x or less

- `print` and `print_fbg` now uses `&str` instead of `String` for text.
  If you want to keep using Strings, you'll just need to add `.as_str()` to get it to work again
- `PubScreen` no longer exists