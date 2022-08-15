pub mod keycode;
pub mod rgb;
mod meowpad;
mod config;

use rgb::*;
use keycode::*;
pub use config::*;
pub use meowpad::*;

use std::io::Cursor;
use byteorder::WriteBytesExt;
fn str_to_cur(str: &str) -> Cursor<[u8; 65]> {
    let mut cur = Cursor::new([0u8; 65]);
    cur.set_position(1);
    str.as_bytes()
        .iter()
        .for_each(|b| cur.write_u8(*b).unwrap());
    cur
}

// use strum::IntoEnumIterator;
// pub fn itertest() {
//     for k in KeyCode::iter() {
//         println!("{k}")
//     }
// }