use meowpad::KeyCode;

pub struct KeyMap([u8; 128]);

impl Default for KeyMap {
    fn default() -> Self {
        Self(
            [
                // normal layer
                41, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 45, 46, 42, // ESC, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, -, =, Backspace
                43, 20, 26, 8, 21, 23, 28, 24, 12, 18, 19, 47, 48, 49,  // TAB, Q, W, E, R, T, Y, U, I, O, P, [, ], '\'
                57, 4, 22, 7, 9, 10, 11, 13, 14, 15, 51, 52, 40,        // CapsLock, A, S, D, F, G, H, J, K, L, ;, ', Enter
                225, 29, 27, 6, 25, 5, 17, 16, 54, 55, 56, 229, 82, 76, // LShift, Z, X, C, V, B, N, M, ,, ., /, RShift, Up, Delete
                224, 227, 226, 44, 230, 0, 80, 81, 79,                  // LCtrl, Win, LAlt, Space, RAlt, FN, Left, Down, Right
                // fn layer
                53, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 0, // ~, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, NONE
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // NONE
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // NONE
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // NONE
                0, 0, 0, 0, 0, 0, 0, 0, 0, // NONE
            ]
        )
    }
}

impl From<KeyMap> for [u8; 128] {
    fn from(value: KeyMap) -> Self {
        value.0
    }
}

impl From<[u8; 128]> for KeyMap {
    fn from(value: [u8; 128]) -> Self {
        Self(value)
    }
}

impl From<KeyMap> for [[KeyCode; 64]; 2] {
    fn from(value: KeyMap) -> Self {
        let mut normal_layer = [KeyCode::default(); 64];  // 用于初始化的默认值
        let mut fn_layer = [KeyCode::default(); 64]; // 用于初始化的默认值

        for (i, &k) in value.0.iter().enumerate() {
            let keycode = KeyCode::from(k);
            if i < 64 {
                normal_layer[i] = keycode;
            } else {
                fn_layer[i - 64] = keycode;
            }
        }

        [
            normal_layer, fn_layer
        ]
    }
}

impl From<[[KeyCode; 64]; 2]> for KeyMap {
    fn from(value: [[KeyCode; 64]; 2]) -> Self {
        let mut combined_array = [0u8; 128];
        
        for (i, &keycode) in value[0].iter().enumerate() {
            combined_array[i] = keycode as u8;
        }
        
        for (i, &keycode) in value[1].iter().enumerate() {
            combined_array[i + 64] = keycode as u8;
        }
        
        KeyMap(combined_array)
    }
}
