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
const_key_event!(CTRL_O, KeyCode::Char('o'), CONTROL);
const_key_event!(CTRL_P, KeyCode::Char('p'), CONTROL);
const_key_event!(CTRL_Q, KeyCode::Char('q'), CONTROL);
const_key_event!(CTRL_R, KeyCode::Char('r'), CONTROL);
const_key_event!(CTRL_S, KeyCode::Char('s'), CONTROL);
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
