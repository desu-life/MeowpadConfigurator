use meowpad::{keycode::KeyValue, KeyCode};


pub struct KeyMap([u8; 42]);

impl Default for KeyMap {
    fn default() -> Self {
        Self(
            [
                 1, 1, 1, 1, 1, 1, 1,
                 29, 27, 6, 25, 20, 26, 8,
                 0, 0, 0, 0, 0, 0, 0,
                 0, 0, 0, 0, 0, 0, 0,
                 0, 0, 0, 0, 0, 0, 0,
                 0, 0, 0, 0, 0, 0, 0,
            ]
        )
    }
}

impl From<KeyMap> for [u8; 42] {
    fn from(value: KeyMap) -> Self {
        value.0
    }
}

impl From<[u8; 42]> for KeyMap {
    fn from(value: [u8; 42]) -> Self {
        Self(value)
    }
}

impl From<KeyMap> for [[KeyValue; 3]; 7] {
    fn from(value: KeyMap) -> Self {
        let mut result = [[KeyValue::None; 3]; 7];
        let data = &value.0;

        for row in 0..7 {
            for col in 0..3 {
                let type_index = row + (col * 14);
                let value_index = type_index + 7;
                result[row][col] = KeyValue::from_u8(data[type_index], data[value_index]);
            }
        }

        result
    }
}

impl From<[[KeyValue; 3]; 7]> for KeyMap {
    fn from(array: [[KeyValue; 3]; 7]) -> Self {
        let mut result = [0u8; 42];
        for row in 0..7 {
            for col in 0..3 {
                let (type_val, value) = array[row][col].to_u8();
                let type_index = row + (col * 14);
                let value_index = type_index + 7;
                result[type_index] = type_val;
                result[value_index] = value;
            }
        }
        KeyMap(result)
    }
}
