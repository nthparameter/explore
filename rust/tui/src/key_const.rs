use crossterm::event::{self, KeyCode};

macro_rules! key_event {
    ($key_code:ident) => {
        event::KeyEvent {
            code: KeyCode::$key_code,
            modifiers: event::KeyModifiers::NONE,
        }
    };
    /*(ctrl, $key_code:ident) => {
          event::KeyEvent {
          code: KeyCode::$key_code,
          modifiers: event::KeyModifiers::CONTROL,
          }

    };
    (ctrl, $key_code:expr) => {
          event::KeyEvent {
          code: $key_code,
          modifiers: event::KeyModifiers::CONTROL,
          }

    };*/
    ($key_code:expr, $key_modifiers:ident) => {
        event::KeyEvent {
            code: $key_code,
            modifiers: event::KeyModifiers::$key_modifiers,
        }
    };
    ($key_code:literal, $key_modifiers:ident) => {
        event::KeyEvent {
            code: KeyCode::$key_code,
            modifiers: event::KeyModifiers::$key_modifiers,
        }
    };
}
macro_rules! const_key_event {
    ($name:ident, $key_code:ident) => {
        pub const $name: event::KeyEvent = event::KeyEvent {
            code: KeyCode::$key_code,
            modifiers: event::KeyModifiers::NONE,
        };
    };
    ($name:ident, $key_code:expr) => {
        pub const $name: event::KeyEvent = event::KeyEvent {
            code: $key_code,
            modifiers: event::KeyModifiers::NONE,
        };
    };
    ($name:ident, $key_code:expr, $key_modifiers:ident) => {
        pub const $name: event::KeyEvent = event::KeyEvent {
            code: $key_code,
            modifiers: event::KeyModifiers::$key_modifiers,
        };
    };
}

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
const_key_event!(CTRL_A, KeyCode::Char('a'), CONTROL);
const_key_event!(CTRL_B, KeyCode::Char('b'), CONTROL);
const_key_event!(CTRL_C, KeyCode::Char('c'), CONTROL);
const_key_event!(CTRL_D, KeyCode::Char('d'), CONTROL);
const_key_event!(CTRL_E, KeyCode::Char('e'), CONTROL);
const_key_event!(CTRL_F, KeyCode::Char('f'), CONTROL);
const_key_event!(CTRL_G, KeyCode::Char('g'), CONTROL);
const_key_event!(CTRL_H, KeyCode::Char('h'), CONTROL);
const_key_event!(CTRL_I, KeyCode::Char('i'), CONTROL);
const_key_event!(CTRL_J, KeyCode::Char('j'), CONTROL);
const_key_event!(CTRL_K, KeyCode::Char('k'), CONTROL);
const_key_event!(CTRL_L, KeyCode::Char('l'), CONTROL);
const_key_event!(CTRL_M, KeyCode::Char('m'), CONTROL);
const_key_event!(CTRL_N, KeyCode::Char('n'), CONTROL);
const_key_event!(CTRL_O, KeyCode::Char('o'), CONTROL);
const_key_event!(CTRL_P, KeyCode::Char('p'), CONTROL);
const_key_event!(CTRL_Q, KeyCode::Char('q'), CONTROL);
const_key_event!(CTRL_R, KeyCode::Char('r'), CONTROL);
const_key_event!(CTRL_S, KeyCode::Char('s'), CONTROL);
const_key_event!(CTRL_T, KeyCode::Char('t'), CONTROL);
const_key_event!(CTRL_U, KeyCode::Char('u'), CONTROL);
const_key_event!(CTRL_V, KeyCode::Char('v'), CONTROL);
const_key_event!(CTRL_W, KeyCode::Char('w'), CONTROL);
const_key_event!(CTRL_X, KeyCode::Char('x'), CONTROL);
const_key_event!(CTRL_Y, KeyCode::Char('y'), CONTROL);
const_key_event!(CTRL_Z, KeyCode::Char('z'), CONTROL);

const_key_event!(CTRL_DOWN, KeyCode::Down, CONTROL);
const_key_event!(CTRL_END, KeyCode::End, CONTROL);
const_key_event!(CTRL_HOME, KeyCode::Home, CONTROL);
const_key_event!(CTRL_UP, KeyCode::Up, CONTROL);

/// Plain keys.
const_key_event!(KEY_BACKSPACE, Backspace);
const_key_event!(KEY_DELETE, Delete);
const_key_event!(KEY_DOWN, Down);
const_key_event!(KEY_END, End);
const_key_event!(KEY_ENTER, Enter);
const_key_event!(KEY_HOME, Home);
const_key_event!(KEY_F1, KeyCode::F(1));
const_key_event!(KEY_F2, KeyCode::F(2));
const_key_event!(KEY_F3, KeyCode::F(3));
const_key_event!(KEY_F4, KeyCode::F(4));
const_key_event!(KEY_F5, KeyCode::F(5));
const_key_event!(KEY_F6, KeyCode::F(6));
const_key_event!(KEY_LEFT, Left);
const_key_event!(KEY_PAGE_DOWN, PageDown);
const_key_event!(KEY_PAGE_UP, PageUp);
const_key_event!(KEY_RIGHT, Right);
const_key_event!(KEY_UP, Up);
