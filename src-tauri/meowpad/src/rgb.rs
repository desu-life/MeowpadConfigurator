use serde::{Deserialize, Serialize};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(
    Serialize, Deserialize, Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
pub struct RGB<T> {
    /// Red
    pub r: T,
    /// Green
    pub g: T,
    /// Blue
    pub b: T,
}

impl<T> FromStr for RGB<T>
where
    ParseIntError: From<<T as FromStr>::Err>,
    T: FromStr,
{
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("rgb").unwrap();
        let coords: Vec<&str> = s
            .trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .collect();

        let r = coords
            .get(0)
            .unwrap_or(&"")
            .trim()
            .parse::<T>()?;
        let g = coords
            .get(1)
            .unwrap_or(&"")
            .trim()
            .parse::<T>()?;
        let b = coords
            .get(2)
            .unwrap_or(&"")
            .trim()
            .parse::<T>()?;

        Ok(RGB { r, g, b })
    }
}

impl<T> RGB<T> {
    /// Convenience function for creating a new pixel
    /// The order of arguments is R,G,B
    #[inline(always)]
    pub const fn new(r: T, g: T, b: T) -> Self {
        Self { r, g, b }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for RGB<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rgb({},{},{})", self.r, self.g, self.b)
    }
}
