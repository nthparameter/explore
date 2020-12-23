use crossterm::event::{self, KeyCode};

/// Alt key modified keys.
// Not working in i term2.
pub const ALT_END: event::KeyEvent = event::KeyEvent {
    code: KeyCode::End,
    modifiers: event::KeyModifiers::ALT,
};
// Not working in i term2.
pub const ALT_HOME: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Home,
    modifiers: event::KeyModifiers::ALT,
};

/// Control key modified keys.
pub const CTRL_DOWN: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Down,
    modifiers: event::KeyModifiers::CONTROL,
};
pub const CTRL_END: event::KeyEvent = event::KeyEvent {
    code: KeyCode::End,
    modifiers: event::KeyModifiers::CONTROL,
};
pub const CTRL_HOME: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Home,
    modifiers: event::KeyModifiers::CONTROL,
};
pub const CTRL_A: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Char('a'),
    modifiers: event::KeyModifiers::CONTROL,
};
pub const CTRL_B: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Char('b'),
    modifiers: event::KeyModifiers::CONTROL,
};
pub const CTRL_C: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Char('c'),
    modifiers: event::KeyModifiers::CONTROL,
};
pub const CTRL_D: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Char('d'),
    modifiers: event::KeyModifiers::CONTROL,
};
pub const CTRL_E: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Char('e'),
    modifiers: event::KeyModifiers::CONTROL,
};
pub const CTRL_F: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Char('f'),
    modifiers: event::KeyModifiers::CONTROL,
};
pub const CTRL_G: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Char('g'),
    modifiers: event::KeyModifiers::CONTROL,
};
pub const CTRL_H: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Char('h'),
    modifiers: event::KeyModifiers::CONTROL,
};
pub const CTRL_I: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Char('i'),
    modifiers: event::KeyModifiers::CONTROL,
};
pub const CTRL_J: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Char('j'),
    modifiers: event::KeyModifiers::CONTROL,
};
pub const CTRL_K: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Char('k'),
    modifiers: event::KeyModifiers::CONTROL,
};
pub const CTRL_L: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Char('l'),
    modifiers: event::KeyModifiers::CONTROL,
};
pub const CTRL_M: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Char('m'),
    modifiers: event::KeyModifiers::CONTROL,
};
pub const CTRL_N: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Char('n'),
    modifiers: event::KeyModifiers::CONTROL,
};
pub const CTRL_O: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Char('o'),
    modifiers: event::KeyModifiers::CONTROL,
};
pub const CTRL_P: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Char('p'),
    modifiers: event::KeyModifiers::CONTROL,
};
pub const CTRL_Q: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Char('q'),
    modifiers: event::KeyModifiers::CONTROL,
};
pub const CTRL_S: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Char('s'),
    modifiers: event::KeyModifiers::CONTROL,
};
pub const CTRL_T: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Char('t'),
    modifiers: event::KeyModifiers::CONTROL,
};
pub const CTRL_U: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Char('u'),
    modifiers: event::KeyModifiers::CONTROL,
};
pub const CTRL_V: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Char('v'),
    modifiers: event::KeyModifiers::CONTROL,
};
pub const CTRL_W: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Char('w'),
    modifiers: event::KeyModifiers::CONTROL,
};
pub const CTRL_X: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Char('x'),
    modifiers: event::KeyModifiers::CONTROL,
};
pub const CTRL_Y: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Char('y'),
    modifiers: event::KeyModifiers::CONTROL,
};
pub const CTRL_Z: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Char('z'),
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
pub const KEY_END: event::KeyEvent = event::KeyEvent {
    code: KeyCode::End,
    modifiers: event::KeyModifiers::NONE,
};
pub const KEY_ENTER: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Enter,
    modifiers: event::KeyModifiers::NONE,
};
pub const KEY_HOME: event::KeyEvent = event::KeyEvent {
    code: KeyCode::Home,
    modifiers: event::KeyModifiers::NONE,
};
pub const KEY_F1: event::KeyEvent = event::KeyEvent {
    code: KeyCode::F(1),
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
pub const KEY_F4: event::KeyEvent = event::KeyEvent {
    code: KeyCode::F(4),
    modifiers: event::KeyModifiers::NONE,
};
pub const KEY_F5: event::KeyEvent = event::KeyEvent {
    code: KeyCode::F(5),
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
