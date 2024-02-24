use nidhogg_derive::Builder;
use serde::{Deserialize, Serialize};

pub type RgbU8 = Rgb<u8>;
pub type RgbF32 = Rgb<f32>;

#[derive(Debug, Default, Clone, Copy, Builder)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Rgb<T> {
    pub red: T,
    pub green: T,
    pub blue: T,
}

impl<T> Rgb<T> {
    #[must_use]
    pub fn new(red: T, green: T, blue: T) -> Rgb<T> {
        Rgb { red, green, blue }
    }
}

impl From<u32> for RgbU8 {
    fn from(color: u32) -> Self {
        RgbU8 {
            red: ((color >> 16) & 0xFF) as u8,
            green: ((color >> 8) & 0xFF) as u8,
            blue: (color & 0xFF) as u8,
        }
    }
}

impl From<RgbU8> for RgbF32 {
    fn from(value: RgbU8) -> Self {
        RgbF32 {
            red: value.red as f32 / 255.0,
            green: value.green as f32 / 255.0,
            blue: value.blue as f32 / 255.0,
        }
    }
}

impl From<RgbF32> for RgbU8 {
    fn from(value: RgbF32) -> Self {
        RgbU8 {
            red: (value.red * 255.0) as u8,
            green: (value.green * 255.0) as u8,
            blue: (value.blue * 255.0) as u8,
        }
    }
}

impl<T> From<Rgb<T>> for [T; 3] {
    fn from(value: Rgb<T>) -> Self {
        [value.red, value.green, value.blue]
    }
}

impl<T: Copy> From<[T; 3]> for Rgb<T> {
    fn from(value: [T; 3]) -> Self {
        Rgb {
            red: value[0],
            green: value[1],
            blue: value[2],
        }
    }
}

macro_rules! impl_consts {
    ($name:ident, $r:expr, $g:expr, $b:expr, f32) => {
        #[doc = concat!("The color ", stringify!($name), " as an f32 RGB value.")]
        pub const $name: RgbF32 = RgbF32 {
            red: $r,
            green: $g,
            blue: $b,
        };
    };
    ($name:ident, $r:expr, $g:expr, $b:expr, u8) => {
        #[doc = concat!("The color ", stringify!($name), " as an 8-bit RGB value.")]
        pub const $name: RgbU8 = RgbU8 {
            red: ($r * 255.0) as u8,
            green: ($g * 255.0) as u8,
            blue: ($b * 255.0) as u8,
        };
    };
    ($($name:ident ($r:expr, $g:expr, $b:expr)),*) => {
        #[doc = "A module containing color constants for f32 RGB values."]
        pub mod f32 {
            use super::RgbF32;
            $(
                impl_consts!{ $name, $r, $g, $b, f32}
            )*
        }
        #[doc = "A module containing color constants for 8-bit RGB values."]
        pub mod u8 {
            use super::RgbU8;
            $(
                impl_consts!{ $name, $r, $g, $b, u8}
            )*
        }
    };
}

impl_consts! {
    BLUE (0.0, 0.0, 1.0),
    CYAN (0.0, 1.0, 1.0),
    EMPTY (0.0, 0.0, 0.0),
    GRAY (0.5, 0.5, 0.5),
    GREEN (0.0, 0.5, 0.0),
    LIME (0.0, 1.0, 0.0),
    MAGENTA (1.0, 0.0, 1.0),
    MAROON (0.5, 0.0, 0.0),
    NAVY (0.0, 0.0, 0.5),
    OLIVE (0.5, 0.5, 0.0),
    PURPLE (0.5, 0.0, 0.5),
    RED (1.0, 0.0, 0.0),
    SILVER (0.75, 0.75, 0.75),
    TEAL (0.0, 0.5, 0.5),
    WHITE (1.0, 1.0, 1.0),
    YELLOW (1.0, 1.0, 0.0),
    ORANGE (1.0, 0.25, 0.0)
}
