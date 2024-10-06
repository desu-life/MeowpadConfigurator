use meowpad::{keycode::KeyValue, KeyCode};


pub struct KeyMap([u8; 256]);

impl Default for KeyMap {
    fn default() -> Self {
        Self(
            [
                // normal layer
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 2, 1, 1, 1,
                41, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 45, 46, 42, // ESC, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, -, =, Backspace
                43, 20, 26, 8, 21, 23, 28, 24, 12, 18, 19, 47, 48, 49,  // TAB, Q, W, E, R, T, Y, U, I, O, P, [, ], '\'
                57, 4, 22, 7, 9, 10, 11, 13, 14, 15, 51, 52, 40,        // CapsLock, A, S, D, F, G, H, J, K, L, ;, ', Enter
                225, 29, 27, 6, 25, 5, 17, 16, 54, 55, 56, 229, 82, 76, // LShift, Z, X, C, V, B, N, M, ,, ., /, RShift, Up, Delete
                224, 227, 226, 44, 230, 1, 80, 81, 79,                  // LCtrl, Win, LAlt, Space, RAlt, FN, Left, Down, Right
                // fn layer
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0,
                0, 0, 0, 0, 0, 0, 0, 4, 4, 4, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 4, 4, 0, 0, 1, 4,
                0, 2, 0, 0, 0, 0, 3, 1, 3,
                // 不想注释了：加了3是鼠标键，4是功能键，2是自定义功能，这里是禁用win
                53, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 0, // ~, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, NONE
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 70, 74, 77, 0, // NONE....PScreen, Home, End
                0, 0, 0, 0, 0, 0, 0, 0xb6, 0xcd, 0xb5, 0, 0, 0, // NONE
                0, 0, 0, 0, 0, 0, 0, 0, 0xea, 0xe9, 0, 0, 75, 0xe2, // NONE....PgUp 
                0, 2, 0, 0, 0, 0, 1, 78, 2, // NONE....PgDn
            ]
        )
    }
}

impl From<KeyMap> for [u8; 256] {
    fn from(value: KeyMap) -> Self {
        value.0
    }
}

impl From<[u8; 256]> for KeyMap {
    fn from(value: [u8; 256]) -> Self {
        Self(value)
    }
}

impl From<KeyMap> for [[KeyValue; 64]; 2] {
    fn from(value: KeyMap) -> Self {
        let mut normal_layer = [KeyValue::default(); 64];  // 用于初始化的默认值
        let mut fn_layer = [KeyValue::default(); 64]; // 用于初始化的默认值

        for i in 0..64 {
            normal_layer[i] = match value.0[i] {
                0 => KeyValue::None,
                1 => KeyValue::Keyboard(KeyCode::from(value.0[i + 64])),
                2 => KeyValue::Custom(value.0[i + 64]),
                3 => KeyValue::Mouse(value.0[i + 64]),
                4 => KeyValue::Media(value.0[i + 64]),
                _ => KeyValue::None,
            };
        }

        for i in 0..64 {
            fn_layer[i] = match value.0[i + 128] {
                0 => KeyValue::None,
                1 => KeyValue::Keyboard(KeyCode::from(value.0[i + 192])),
                2 => KeyValue::Custom(value.0[i + 192]),
                3 => KeyValue::Mouse(value.0[i + 192]),
                4 => KeyValue::Media(value.0[i + 192]),
                _ => KeyValue::None,
            };
        }

        [
            normal_layer, fn_layer
        ]
    }
}

impl From<[[KeyValue; 64]; 2]> for KeyMap {
    fn from(value: [[KeyValue; 64]; 2]) -> Self {
        let mut combined_array = [0u8; 256];
        
        for (i, &keycode) in value[0].iter().enumerate() {
            match keycode {
                KeyValue::None => (),
                KeyValue::Keyboard(k) => {
                    combined_array[i] = 1;
                    combined_array[i + 64] = k as u8;
                },
                KeyValue::Custom(m) => {
                    combined_array[i] = 2;
                    combined_array[i + 64] = m
                },
                KeyValue::Mouse(m) => {
                    combined_array[i] = 3;
                    combined_array[i + 64] = m
                },
                KeyValue::Media(m) => {
                    combined_array[i] = 4;
                    combined_array[i + 64] = m
                },
            }            
        }
        
        for (i, &keycode) in value[1].iter().enumerate() {
            match keycode {
                KeyValue::None => (),
                KeyValue::Keyboard(k) => {
                    combined_array[i + 128] = 1;
                    combined_array[i + 192] = k as u8;
                },
                KeyValue::Custom(m) => {
                    combined_array[i + 128] = 2;
                    combined_array[i + 192] = m
                },
                KeyValue::Mouse(m) => {
                    combined_array[i + 128] = 3;
                    combined_array[i + 192] = m
                },
                KeyValue::Media(m) => {
                    combined_array[i + 128] = 4;
                    combined_array[i + 192] = m
                },
            }
        }
        
        KeyMap(combined_array)
    }
}
