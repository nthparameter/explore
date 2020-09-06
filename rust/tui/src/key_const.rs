use crossterm::event::{self, KeyCode};

/// Control key modified keys.
pub const CTRL_DOWN: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Down,
    modifiers: event::KeyModifiers::CONTROL,
};
pub const CTRL_O: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Char('o'),
    modifiers: event::KeyModifiers::CONTROL,
};
pub const CTRL_Q: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Char('q'),
    modifiers: event::KeyModifiers::CONTROL,
};
pub const CTRL_UP: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Up,
    modifiers: event::KeyModifiers::CONTROL,
};

/// Plain keys.
pub const KEY_DOWN: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Down,
    modifiers: event::KeyModifiers::NONE,
};
pub const KEY_F2: event::KeyEvent = event::KeyEvent {
    code: KeyCode::F(2),
    modifiers: event::KeyModifiers::NONE,
};
pub const KEY_F3: event::KeyEvent = event::KeyEvent {
    code: KeyCode::F(3),
    modifiers: event::KeyModifiers::NONE,
};
pub const KEY_LEFT: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Left,
    modifiers: event::KeyModifiers::NONE,
};
pub const KEY_PAGE_DOWN: event::KeyEvent = event::KeyEvent {
    code: KeyCode::PageDown,
    modifiers: event::KeyModifiers::NONE,
};
pub const KEY_PAGE_UP: event::KeyEvent = event::KeyEvent {
    code: KeyCode::PageUp,
    modifiers: event::KeyModifiers::NONE,
};
pub const KEY_RIGHT: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Right,
    modifiers: event::KeyModifiers::NONE,
};
pub const KEY_UP: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Up,
    modifiers: event::KeyModifiers::NONE,
};
