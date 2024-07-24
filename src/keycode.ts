
export enum KeyCode {
    /// The "no" key, a placeholder to express nothing.
    None = 0x00,
    /// Error if too much keys are pressed at the same time.
    ErrorRollOver,
    /// The POST fail error.
    PostFail,
    /// An undefined error occured.
    ErrorUndefined,
    /// `a` and `A`.
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M, // 0x10
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    /// `1` and `!`.
    Kb1,
    /// `2` and `@`.
    Kb2,
    /// `3` and `#`.
    Kb3, // 0x20
    /// `4` and `$`.
    Kb4,
    /// `5` and `%`.
    Kb5,
    /// `6` and `^`.
    Kb6,
    /// `7` and `&`.
    Kb7,
    /// `8` and `*`.
    Kb8,
    /// `9` and `(`.
    Kb9,
    /// `0` and `)`.
    Kb0,
    Enter,
    Esc,
    BSpace,
    Tab,
    Space,
    /// `-` and `_`.
    Minus,
    /// `=` and `+`.
    Equal,
    /// `[` and `{`.
    LBracket,
    /// `]` and `}`.
    RBracket, // 0x30
    /// `\` and `|`.
    Bslash,
    /// Non-US `#` and `~` (Typically near the Enter key).
    NonUsHash,
    /// `;` and `:`.
    SColon,
    /// `'` and `"`.
    Quote,
    // How to have ` as code?
    /// \` and `~`.
    Grave,
    /// `,` and `<`.
    Comma,
    /// `.` and `>`.
    Dot,
    /// `/` and `?`.
    Slash,
    CapsLock,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7, // 0x40
    F8,
    F9,
    F10,
    F11,
    F12,
    PScreen,
    ScrollLock,
    Pause,
    Insert,
    Home,
    PgUp,
    Delete,
    End,
    PgDown,
    Right,
    Left, // 0x50
    Down,
    Up,
    NumLock,
    /// Keypad `/`
    KpSlash,
    /// Keypad `*`
    KpAsterisk,
    /// Keypad `-`.
    KpMinus,
    /// Keypad `+`.
    KpPlus,
    /// Keypad enter.
    KpEnter,
    /// Keypad 1.
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8, // 0x60
    Kp9,
    Kp0,
    KpDot,
    /// Non-US `\` and `|` (Typically near the Left-Shift key)
    NonUsBslash,
    Application, // 0x65
    /// not a key, used for errors
    Power,
    /// Keypad `=`.
    KpEqual,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21, // 0x70
    F22,
    F23,
    F24,
    Execute,
    Help,
    Menu,
    Select,
    Stop,
    Again,
    Undo,
    Cut,
    Copy,
    Paste,
    Find,
    Mute,
    VolUp, // 0x80
    VolDown,
    /// Deprecated.
    LockingCapsLock,
    /// Deprecated.
    LockingNumLock,
    /// Deprecated.
    LockingScrollLock,
    /// Keypad `,`, also used for the brazilian keypad period (.) key.
    KpComma,
    /// Used on AS/400 keyboard
    KpEqualSign,
    Intl1,
    Intl2,
    Intl3,
    Intl4,
    Intl5,
    Intl6,
    Intl7,
    Intl8,
    Intl9,
    Lang1, // 0x90
    Lang2,
    Lang3,
    Lang4,
    Lang5,
    Lang6,
    Lang7,
    Lang8,
    Lang9,
    AltErase,
    SysReq,
    Cancel,
    Clear,
    Prior,
    Return,
    Separator,
    Out, // 0xA0
    Oper,
    ClearAgain,
    CrSel,
    ExSel,

    // According to QMK, 0xA5-0xDF are not usable on modern keyboards

    // Modifiers
    /// Left Control.
    LCtrl = 0xE0,
    /// Left Shift.
    LShift,
    /// Left Alt.
    LAlt,
    /// Left GUI (the Windows key).
    LGui,
    /// Right Control.
    RCtrl,
    /// Right Shift.
    RShift,
    /// Right Alt (or Alt Gr).
    RAlt,
    /// Right GUI (the Windows key).
    RGui, // 0xE7

    // Unofficial
    MediaPlayPause = 0xE8,
    MediaStopCD,
    MediaPreviousSong,
    MediaNextSong,
    MediaEjectCD,
    MediaVolUp,
    MediaVolDown,
    MediaMute,
    MediaWWW, // 0xF0
    MediaBack,
    MediaForward,
    MediaStop,
    MediaFind,
    MediaScrollUp,
    MediaScrollDown,
    MediaEdit,
    MediaSleep,
    MediaCoffee,
    MediaRefresh,
    MediaCalc, // 0xFB
}

export let mapping = {
    0x04: "A",
    0x05: "B",
    0x06: "C",
    0x07: "D",
    0x08: "E",
    0x09: "F",
    0x0a: "G",
    0x0b: "H",
    0x0c: "I",
    0x0d: "J",
    0x0e: "K",
    0x0f: "L",
    0x10: "M",
    0x11: "N",
    0x12: "O",
    0x13: "P",
    0x14: "Q",
    0x15: "R",
    0x16: "S",
    0x17: "T",
    0x18: "U",
    0x19: "V",
    0x1a: "W",
    0x1b: "X",
    0x1c: "Y",
    0x1d: "Z",
    0x1e: "1",
    0x1f: "2",
    0x20: "3",
    0x21: "4",
    0x22: "5",
    0x23: "6",
    0x24: "7",
    0x25: "8",
    0x26: "9",
    0x27: "0",
    0x2a: "Backspace",
    0x2b: "Tab",
    0x2c: "Space",
    0x2d: "-",
    0x2e: "=",
    0x2f: "[",
    0x30: "]",
    0x31: "\\",
    0x32: "#",
    0x33: ";",
    0x34: "'",
    0x35: "`",
    0x36: ",",
    0x37: ".",
    0x38: "/",
    0x39: "Caps",
    0x3a: "F1",
    0x3b: "F2",
    0x3c: "F3",
    0x3d: "F4",
    0x3e: "F5",
    0x3f: "F6",
    0x40: "F7",
    0x41: "F8",
    0x42: "F9",
    0x43: "F10",
    0x44: "F11",
    0x45: "F12",
    0x49: "Ins",
    0x4a: "Home",
    0x4b: "PgUp",
    0x4c: "Del",
    0x4d: "End",
    0x4e: "PgDn",
    0x50: "←",
    0x51: "↓",
    0x52: "↑",
    0x4f: "→",
    0x55: "*",
    0x56: "-",
    0x57: "+",
    0x58: "Enter",
    0x59: "1",
    0x5a: "2",
    0x5b: "3",
    0x5c: "4",
    0x5d: "5",
    0x5e: "6",
    0x5f: "7",
    0x60: "8",
    0x61: "9",
    0x62: "0",
    0x63: ".",
    0x67: "=",
    0x85: ",",
    0xb6: "(",
    0xb7: ")",
    0xe0: "Ctrl",
    0xe1: "Shift",
    0xe2: "Alt",
    0xe3: "Meta",
    0xe4: "Ctrl",
    0xe5: "Shift",
    0xe6: "Alt",
    0xe7: "Meta",
    0x87: "IntlRo",
    0x7d: "IntlYen",
    0x28: "Enter",
    0x29: "Esc",
};

export let jsToHid = {
    "AltLeft": 0xe2,
    "AltRight": 0xe6,
    "ArrowDown": 0x51,
    "ArrowLeft": 0x50,
    "ArrowRight": 0x4f,
    "ArrowUp": 0x52,
    "Backquote": 0x35,
    "Backslash": 0x31,
    "Backspace": 0x2a,
    "BracketLeft": 0x2f,
    "BracketRight": 0x30,
    "CapsLock": 0x39,
    "Comma": 0x36,
    "ControlLeft": 0xe0,
    "ControlRight": 0xe4,
    "Insert": 0x49,
    "Delete": 0x4c,
    "Digit0": 0x27,
    "Digit1": 0x1e,
    "Digit2": 0x1f,
    "Digit3": 0x20,
    "Digit4": 0x21,
    "Digit5": 0x22,
    "Digit6": 0x23,
    "Digit7": 0x24,
    "Digit8": 0x25,
    "Digit9": 0x26,
    "End": 0x4d,
    "Enter": 0x28,
    "Equal": 0x2e,
    "Escape": 0x29,
    "F1": 0x3a,
    "F2": 0x3b,
    "F3": 0x3c,
    "F4": 0x3d,
    "F5": 0x3e,
    "F6": 0x3f,
    "F7": 0x40,
    "F8": 0x41,
    "F9": 0x42,
    "F10": 0x43,
    "F11": 0x44,
    "F12": 0x45,
    "Home": 0x4a,
    "IntlBackslash": 0x31,
    "KeyA": 0x04,
    "KeyB": 0x05,
    "KeyC": 0x06,
    "KeyD": 0x07,
    "KeyE": 0x08,
    "KeyF": 0x09,
    "KeyG": 0x0a,
    "KeyH": 0x0b,
    "KeyI": 0x0c,
    "KeyJ": 0x0d,
    "KeyK": 0x0e,
    "KeyL": 0x0f,
    "KeyM": 0x10,
    "KeyN": 0x11,
    "KeyO": 0x12,
    "KeyP": 0x13,
    "KeyQ": 0x14,
    "KeyR": 0x15,
    "KeyS": 0x16,
    "KeyT": 0x17,
    "KeyU": 0x18,
    "KeyV": 0x19,
    "KeyW": 0x1a,
    "KeyX": 0x1b,
    "KeyY": 0x1c,
    "KeyZ": 0x1d,
    "MetaLeft": 0xe3,
    "MetaRight": 0xe7,
    "Minus": 0x2d,
    "NumpadEnter": 0x58,
    "PageDown": 0x4e,
    "PageUp": 0x4b,
    "Period": 0x37,
    "Quote": 0x34,
    "Semicolon": 0x33,
    "ShiftLeft": 0xe1,
    "ShiftRight": 0xe5,
    "Slash": 0x38,
    "Space": 0x2c,
    "Tab": 0x2b,
    "Numpad1": 0x59,
    "Numpad2": 0x5a,
    "Numpad3": 0x5b,
    "Numpad4": 0x5c,
    "Numpad5": 0x5d,
    "Numpad6": 0x5e,
    "Numpad7": 0x5f,
    "Numpad8": 0x60,
    "Numpad9": 0x61,
    "Numpad0": 0x62,
    "NumpadDecimal": 0x63,
    "NumpadAdd": 0x57,
    "NumpadSubtract": 0x56,
    "NumpadMultiply": 0x55,
    "NumpadDivide": 0x54,
    "NumpadEqual": 0x67,
    "NumpadComma": 0x85,
    "NumpadParenLeft": 0xb6,
    "NumpadParenRight": 0xb7,
    "IntlRo": 0x87,
    "IntlYen": 0x7d,
    "IntlHash": 0x32,
};