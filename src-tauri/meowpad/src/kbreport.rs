
use crate::KeyCode;
use std::borrow::BorrowMut;

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct KbReport(u8, [u8; 6]);

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
            value.1
                .iter()
                .map(|&k| KeyCode::from(k))
                .filter(|&k| k != KeyCode::None)
                .collect(),
        ]
        .concat()
    }
}

impl From<KbReport> for [KeyCode; 6] {
    fn from(value: KbReport) -> Self {
        Vec::from(value).iter().map(|k| *k).chain(std::iter::repeat(KeyCode::None).take(6))
        .take(6).collect::<Vec<_>>().try_into().unwrap()
    }
}

impl From<KbReport> for [u8; 6] {
    fn from(value: KbReport) -> Self {
        Vec::from(value).iter().map(|k| *k as u8).chain(std::iter::repeat(0).take(6))
        .take(6).collect::<Vec<u8>>().try_into().unwrap()
    }
}

impl From<[u8; 6]> for KbReport {
    fn from(value: [u8; 6]) -> Self {
        value.iter().map(|&k| KeyCode::from(k)).collect()
    }
}

impl KbReport {
    /// Add the given key code to the report. If the report is full,
    /// it will be set to `ErrorRollOver`.
    pub fn pressed(&mut self, kc: KeyCode) {
        use KeyCode::*;
        match kc {
            None => (),
            ErrorRollOver | PostFail | ErrorUndefined => (),
            kc if kc.is_modifier() => self.0 |= kc.as_modifier_bit(),
            _ => {
                let k = kc as u8;
                let s: &mut [u8] = self.1.borrow_mut();
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
        let i = self.0;
        (START..END)
            .filter(|&k| (i & (1 << (k - START))) != 0)
            .map(|k| KeyCode::from(k))
            .collect()
    }
}
