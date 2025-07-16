use meowpad::{keycode::KeyValue, KeyCode};

pub const TOTAL_KEYS: usize = 8;
pub const ROW_SIZE: usize = 3;
pub const KEYMAP_SIZE: usize = TOTAL_KEYS * ROW_SIZE * 2;
pub type Row = [KeyValue; ROW_SIZE];
pub type KeyMatrix = [Row; TOTAL_KEYS];

pub struct KeyMap([u8; KEYMAP_SIZE]);

impl Default for KeyMap {
    fn default() -> Self {
        Self(
            [
                 1, 1, 1, 1, 1, 1, 1, 4,
                 29, 27, 6, 25, 20, 26, 8, 0xcd,
                 0, 0, 0, 0, 0, 0, 0, 0,
                 0, 0, 0, 0, 0, 0, 0, 0,
                 0, 0, 0, 0, 0, 0, 0, 0,
                 0, 0, 0, 0, 0, 0, 0, 0,
            ]
        )
    }
}

impl From<KeyMap> for [u8; KEYMAP_SIZE] {
    fn from(value: KeyMap) -> Self {
        value.0
    }
}

impl From<[u8; KEYMAP_SIZE]> for KeyMap {
    fn from(value: [u8; KEYMAP_SIZE]) -> Self {
        Self(value)
    }
}

impl From<KeyMap> for KeyMatrix {
    fn from(value: KeyMap) -> Self {
        let mut result = [[KeyValue::None; ROW_SIZE]; TOTAL_KEYS];
        let data = &value.0;

        for row in 0..TOTAL_KEYS {
            for col in 0..ROW_SIZE {
                let type_index = row + (col * (TOTAL_KEYS * 2));
                let value_index = type_index + TOTAL_KEYS;
                result[row][col] = KeyValue::from_u8(data[type_index], data[value_index]);
            }
        }

        result
    }
}

impl From<KeyMatrix> for KeyMap {
    fn from(array: KeyMatrix) -> Self {
        let mut result = [0u8; KEYMAP_SIZE];
        for row in 0..TOTAL_KEYS {
            for col in 0..ROW_SIZE {
                let (type_val, value) = array[row][col].to_u8();
                let type_index = row + (col * (TOTAL_KEYS * 2));
                let value_index = type_index + TOTAL_KEYS;
                result[type_index] = type_val;
                result[value_index] = value;
            }
        }
        KeyMap(result)
    }
}
