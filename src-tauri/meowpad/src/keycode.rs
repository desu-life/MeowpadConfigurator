use num_derive::{FromPrimitive, ToPrimitive};
use serde_repr::*;
use strum_macros::EnumIter;

#[allow(non_camel_case_types)]
#[derive(
    Default,
    EnumIter,
    Serialize_repr,
    Deserialize_repr,
    FromPrimitive,
    ToPrimitive,
    Copy,
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
)]
#[repr(u8)]
pub enum KeyCode {
    /// The "no" key, a placeholder to express nothing.
    #[default]
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

use std::{borrow::BorrowMut, fmt};
impl fmt::Display for KeyCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code: i32 = (*self) as i32;
        f.write_fmt(format_args!("{:?} ({})", self, code))?;
        Ok(())
    }
}

use num::FromPrimitive;
impl From<u8> for KeyCode {
    fn from(value: u8) -> Self {
        KeyCode::from_u8(value).unwrap_or_default()
    }
}

impl KeyCode {
    /// Returns `true` if the key code corresponds to a modifier (sent
    /// separately on the USB HID report).
    pub fn is_modifier(self) -> bool {
        KeyCode::LCtrl <= self && self <= KeyCode::RGui
    }

    /// Returns the byte with the bit corresponding to the USB HID
    /// modifier bitfield set.
    pub fn as_modifier_bit(self) -> u8 {
        if self.is_modifier() {
            1 << (self as u8 - KeyCode::LCtrl as u8)
        } else {
            0
        }
    }

    pub fn to_report(self) -> KbReport {
        let mut r = KbReport::default();
        r.pressed(self);
        r
    }
}

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct KbReport([u8; 4]);

impl core::iter::FromIterator<KeyCode> for KbReport {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = KeyCode>,
    {
        let mut res = Self::default();
        for kc in iter {
            res.pressed(kc);
        }
        res
    }
}

impl From<KbReport> for Vec<KeyCode> {
    fn from(value: KbReport) -> Self {
        [
            value.get_modifier_codes(),
            value.0[..3]
                .iter()
                .map(|&k| KeyCode::from(k))
                .filter(|&k| k != KeyCode::None)
                .collect(),
        ]
        .concat()
    }
}

impl From<KbReport> for [u8; 4] {
    fn from(value: KbReport) -> Self {
        value.0
    }
}

impl From<[u8; 4]> for KbReport {
    fn from(value: [u8; 4]) -> Self {
        Self(value)
    }
}

impl KbReport {
    /// Returns the byte slice corresponding to the report.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Add the given key code to the report. If the report is full,
    /// it will be set to `ErrorRollOver`.
    pub fn pressed(&mut self, kc: KeyCode) {
        use KeyCode::*;
        match kc {
            None => (),
            ErrorRollOver | PostFail | ErrorUndefined => (),
            kc if kc.is_modifier() => self.0[3] |= kc.as_modifier_bit(),
            _ => {
                let k = kc as u8;
                let s = self.0[0..3].borrow_mut();
                if !s.contains(&k) {
                    s.iter_mut()
                        .find(|c| **c == 0)
                        .map(|c| *c = kc as u8)
                        .unwrap_or_default()
                }
            }
        }
    }

    pub fn get_modifier_codes(&self) -> Vec<KeyCode> {
        const START: u8 = KeyCode::LCtrl as u8;
        const END: u8 = KeyCode::RGui as u8;
        let i = unsafe { *self.0.get_unchecked(3) };
        (START..END)
            .filter(|&k| (i & (1 << (k - START))) != 0)
            .map(|k| KeyCode::from(k))
            .collect()
    }
}
