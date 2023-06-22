// export declare type LightingMode = 'Off' | 'Solid' | 'Breath' | 'RainbowBreath' | 'RainbowGradient' | "PressAndLight" | "SpeedPress";

export interface IRgb {
    red: number
    green: number
    blue: number
}

export interface IDevice {
    name: string
    version: string
}

export interface IHsData {
    dyn: number;
    min: number;
    max: number;
    fixed: number;
}

export interface IVersion {
    version: string
    download_url: string
    v1_latest_firmware_version: string
    v1_hs_latest_firmware_version: string
    latest_firmware_download_url: string | undefined
    v1_latest_firmware_download_url: string
    v1_hs_latest_firmware_download_url: string
}

export interface IConfig { 
    key_1: KeyCode[]
    key_2: KeyCode[]
    key_3: KeyCode[]
    key_4: KeyCode[]
    key_5: KeyCode[]
    led_color_l: IRgb
    led_color_r: IRgb
    led_color_btm_l: IRgb
    led_color_btm_r: IRgb
    lighting_mode_key: LightingMode
    lighting_mode_btm: LightingMode
    maximum_brightness: number
    breath_minimum_brightness: number
    breath_maximum_light_duration: number
    breath_minimum_light_duration: number
    breath_interval: number
    press_light_minimum_brightness: number
    press_light_duration: number
    rainbow_light_switching_interval: number
    speed_press_high_color: IRgb
    speed_press_low_color: IRgb
    speed_press_trans_speed: number
    speed_press_minimum_brightness: number
    press_light_step: number
    keyboard_jitters_elimination_time: number
    keyboard_jitters_elimination_mode: JitterEliminationMode,
    force_key_switch: boolean,
    device_sleep_idle_time: number,
    sleep_mode_maximum_brightness: number,
    sleep_lighting_mode_key: LightingMode,
    sleep_lighting_mode_btm: LightingMode,
    key_trigger_degree: number | undefined,       // 0-100
    key_release_degree: number | undefined,       // 0-100
    dead_zone: number | undefined,               // 0-30
    key_scan_rate: number | undefined,               // 0-30
}

export enum JitterEliminationMode {
    Active = 0,
    Lazy = 1,
}

export enum LightingMode {
    Off = 0,
    Solid = 1,
    Breath = 2,
    RainbowBreathSwitch = 3,
    RainbowBreathSync = 4,
    RainbowGradientSwitch = 5,
    RainbowGradientSync = 6,
    PressAndLight = 7,
    SpeedPress = 8,
    根据按压力度决定LED发光程度 = 9,
}

// export enum KeyCode {
//     NONE = 0x00,
//     ERR_OVF = 0x01,
//     A = 0x04,
//     B = 0x05,
//     C = 0x06,
//     D = 0x07,
//     E = 0x08,
//     F = 0x09,
//     G = 0x0a,
//     H = 0x0b,
//     I = 0x0c,
//     J = 0x0d,
//     K = 0x0e,
//     L = 0x0f,
//     M = 0x10,
//     N = 0x11,
//     O = 0x12,
//     P = 0x13,
//     Q = 0x14,
//     R = 0x15,
//     S = 0x16,
//     T = 0x17,
//     U = 0x18,
//     V = 0x19,
//     W = 0x1a,
//     X = 0x1b,
//     Y = 0x1c,
//     Z = 0x1d,
//     NUM_1 = 0x1e,
//     NUM_2 = 0x1f,
//     NUM_3 = 0x20,
//     NUM_4 = 0x21,
//     NUM_5 = 0x22,
//     NUM_6 = 0x23,
//     NUM_7 = 0x24,
//     NUM_8 = 0x25,
//     NUM_9 = 0x26,
//     NUM_0 = 0x27,
//     ENTER = 0x28,
//     ESC = 0x29,
//     BACKSPACE = 0x2a,
//     TAB = 0x2b,
//     SPACE = 0x2c,
//     MINUS = 0x2d,
//     EQUAL = 0x2e,
//     LEFTBRACE = 0x2f,
//     RIGHTBRACE = 0x30,
//     BACKSLASH = 0x31,
//     HASHTILDE = 0x32,
//     SEMICOLON = 0x33,
//     APOSTROPHE = 0x34,
//     GRAVE = 0x35,
//     COMMA = 0x36,
//     DOT = 0x37,
//     SLASH = 0x38,
//     CAPSLOCK = 0x39,
//     F1 = 0x3a,
//     F2 = 0x3b,
//     F3 = 0x3c,
//     F4 = 0x3d,
//     F5 = 0x3e,
//     F6 = 0x3f,
//     F7 = 0x40,
//     F8 = 0x41,
//     F9 = 0x42,
//     F10 = 0x43,
//     F11 = 0x44,
//     F12 = 0x45,
//     SYSRQ = 0x46,
//     SCROLLLOCK = 0x47,
//     PAUSE = 0x48,
//     INSERT = 0x49,
//     HOME = 0x4a,
//     PAGEUP = 0x4b,
//     DELETE = 0x4c,
//     END = 0x4d,
//     PAGEDOWN = 0x4e,
//     RIGHT = 0x4f,
//     LEFT = 0x50,
//     DOWN = 0x51,
//     UP = 0x52,
//     NUMLOCK = 0x53,
//     KPSLASH = 0x54,
//     KPASTERISK = 0x55,
//     KPMINUS = 0x56,
//     KPPLUS = 0x57,
//     KPENTER = 0x58,
//     KP1 = 0x59,
//     KP2 = 0x5a,
//     KP3 = 0x5b,
//     KP4 = 0x5c,
//     KP5 = 0x5d,
//     KP6 = 0x5e,
//     KP7 = 0x5f,
//     KP8 = 0x60,
//     KP9 = 0x61,
//     KP0 = 0x62,
//     KPDOT = 0x63,
//     LEFT_BACK_SLASH = 0x64,
//     COMPOSE = 0x65,
//     POWER = 0x66,
//     KPEQUAL = 0x67,
//     F13 = 0x68,
//     F14 = 0x69,
//     F15 = 0x6a,
//     F16 = 0x6b,
//     F17 = 0x6c,
//     F18 = 0x6d,
//     F19 = 0x6e,
//     F20 = 0x6f,
//     F21 = 0x70,
//     F22 = 0x71,
//     F23 = 0x72,
//     F24 = 0x73,
//     OPEN = 0x74,
//     HELP = 0x75,
//     PROPS = 0x76,
//     FRONT = 0x77,
//     STOP = 0x78,
//     AGAIN = 0x79,
//     UNDO = 0x7a,
//     CUT = 0x7b,
//     COPY = 0x7c,
//     PASTE = 0x7d,
//     FIND = 0x7e,
//     MUTE = 0x7f,
//     VOLUMEUP = 0x80,
//     VOLUMEDOWN = 0x81,
//     KPCOMMA = 0x85,
//     RO = 0x87,
//     KATAKANAHIRAGANA = 0x88,
//     YEN = 0x89,
//     HENKAN = 0x8a,
//     MUHENKAN = 0x8b,
//     KPJPCOMMA = 0x8c,
//     HANGEUL = 0x90,
//     HANJA = 0x91,
//     KATAKANA = 0x92,
//     HIRAGANA = 0x93,
//     ZENKAKUHANKAKU = 0x94,
//     KPLEFTPAREN = 0xb6,
//     KPRIGHTPAREN = 0xb7,
//     LEFTCTRL = 0xe0,
//     LEFTSHIFT = 0xe1,
//     LEFTALT = 0xe2,
//     LEFTMETA = 0xe3,
//     RIGHTCTRL = 0xe4,
//     RIGHTSHIFT = 0xe5,
//     RIGHTALT = 0xe6,
//     RIGHTMETA = 0xe7,
//     MEDIA_PLAYPAUSE = 0xe8,
//     MEDIA_STOPCD = 0xe9,
//     MEDIA_PREVIOUSSONG = 0xea,
//     MEDIA_NEXTSONG = 0xeb,
//     MEDIA_EJECTCD = 0xec,
//     MEDIA_VOLUMEUP = 0xed,
//     MEDIA_VOLUMEDOWN = 0xee,
//     MEDIA_MUTE = 0xef,
//     MEDIA_WWW = 0xf0,
//     MEDIA_BACK = 0xf1,
//     MEDIA_FORWARD = 0xf2,
//     MEDIA_STOP = 0xf3,
//     MEDIA_FIND = 0xf4,
//     MEDIA_SCROLLUP = 0xf5,
//     MEDIA_SCROLLDOWN = 0xf6,
//     MEDIA_EDIT = 0xf7,
//     MEDIA_SLEEP = 0xf8,
//     MEDIA_COFFEE = 0xf9,
//     MEDIA_REFRESH = 0xfa,
//     MEDIA_CALC = 0xfb,
// }

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
    Escape,
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