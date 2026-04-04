use lexical::{
    NumberFormatBuilder, ParseFloatOptions, ParseIntegerOptions, WriteFloatOptions,
    WriteIntegerOptions,
};
use std::fmt::Display;
#[cfg(feature = "f16")]
pub type F = f16;
#[cfg(feature = "f32")]
pub type F = f32;
#[cfg(feature = "f64")]
pub type F = f64;
#[cfg(feature = "f128")]
pub type F = f128;
pub fn parse_radix(src: &str, base: u8) -> Option<F> {
    let options = ParseFloatOptions::from_radix(base);
    macro_rules! parses {
            ($($n:ident = $nu:expr),*) => {
                match base {
                    $(
                        $nu => lexical::parse_with_options::<F, _, $n>(src, &options).ok(),
                    )*
                    _ => unreachable!()
                }
            };
        }
    parses!(
        F2 = 2,
        F3 = 3,
        F4 = 4,
        F5 = 5,
        F6 = 6,
        F7 = 7,
        F8 = 8,
        F9 = 9,
        F10 = 10,
        F11 = 11,
        F12 = 12,
        F13 = 13,
        F14 = 14,
        F15 = 15,
        F16 = 16,
        F17 = 17,
        F18 = 18,
        F19 = 19,
        F20 = 20,
        F21 = 21,
        F22 = 22,
        F23 = 23,
        F24 = 24,
        F25 = 25,
        F26 = 26,
        F27 = 27,
        F28 = 28,
        F29 = 29,
        F30 = 30,
        F31 = 31,
        F32 = 32,
        F33 = 33,
        F34 = 34,
        F35 = 35,
        F36 = 36
    )
}
pub fn to_string_radix(value: f64, base: u8) -> impl Display {
    let options = WriteFloatOptions::from_radix(base);
    macro_rules! strings {
            ($($n:ident = $nu:expr),*) => {
                match base {
                    $(
                        $nu => lexical::to_string_with_options::<F, $n>(value, &options),
                    )*
                    _ => unreachable!()
                }
            };
        }
    strings!(
        F2 = 2,
        F3 = 3,
        F4 = 4,
        F5 = 5,
        F6 = 6,
        F7 = 7,
        F8 = 8,
        F9 = 9,
        F10 = 10,
        F11 = 11,
        F12 = 12,
        F13 = 13,
        F14 = 14,
        F15 = 15,
        F16 = 16,
        F17 = 17,
        F18 = 18,
        F19 = 19,
        F20 = 20,
        F21 = 21,
        F22 = 22,
        F23 = 23,
        F24 = 24,
        F25 = 25,
        F26 = 26,
        F27 = 27,
        F28 = 28,
        F29 = 29,
        F30 = 30,
        F31 = 31,
        F32 = 32,
        F33 = 33,
        F34 = 34,
        F35 = 35,
        F36 = 36
    )
}
pub fn parse_radix_usize(src: &str, base: u8) -> Option<usize> {
    let options = ParseIntegerOptions::from_radix(base);
    macro_rules! parses {
            ($($n:ident = $nu:expr),*) => {
                match base {
                    $(
                        $nu => lexical::parse_with_options::<usize, _, $n>(src, &options).ok(),
                    )*
                    _ => unreachable!()
                }
            };
        }
    parses!(
        F2 = 2,
        F3 = 3,
        F4 = 4,
        F5 = 5,
        F6 = 6,
        F7 = 7,
        F8 = 8,
        F9 = 9,
        F10 = 10,
        F11 = 11,
        F12 = 12,
        F13 = 13,
        F14 = 14,
        F15 = 15,
        F16 = 16,
        F17 = 17,
        F18 = 18,
        F19 = 19,
        F20 = 20,
        F21 = 21,
        F22 = 22,
        F23 = 23,
        F24 = 24,
        F25 = 25,
        F26 = 26,
        F27 = 27,
        F28 = 28,
        F29 = 29,
        F30 = 30,
        F31 = 31,
        F32 = 32,
        F33 = 33,
        F34 = 34,
        F35 = 35,
        F36 = 36
    )
}
pub fn to_string_radix_usize(value: usize, base: u8) -> impl Display {
    let options = WriteIntegerOptions::from_radix(base);
    macro_rules! strings {
            ($($n:ident = $nu:expr),*) => {
                match base {
                    $(
                        $nu => lexical::to_string_with_options::<usize, $n>(value, &options),
                    )*
                    _ => unreachable!()
                }
            };
        }
    strings!(
        F2 = 2,
        F3 = 3,
        F4 = 4,
        F5 = 5,
        F6 = 6,
        F7 = 7,
        F8 = 8,
        F9 = 9,
        F10 = 10,
        F11 = 11,
        F12 = 12,
        F13 = 13,
        F14 = 14,
        F15 = 15,
        F16 = 16,
        F17 = 17,
        F18 = 18,
        F19 = 19,
        F20 = 20,
        F21 = 21,
        F22 = 22,
        F23 = 23,
        F24 = 24,
        F25 = 25,
        F26 = 26,
        F27 = 27,
        F28 = 28,
        F29 = 29,
        F30 = 30,
        F31 = 31,
        F32 = 32,
        F33 = 33,
        F34 = 34,
        F35 = 35,
        F36 = 36
    )
}
macro_rules! formats {
    ($($n:ident = $nu:expr),*) => {
        $(const $n: u128 = NumberFormatBuilder::new().radix($nu).build_strict();)*
    };
}
formats!(
    F2 = 2,
    F3 = 3,
    F4 = 4,
    F5 = 5,
    F6 = 6,
    F7 = 7,
    F8 = 8,
    F9 = 9,
    F10 = 10,
    F11 = 11,
    F12 = 12,
    F13 = 13,
    F14 = 14,
    F15 = 15,
    F16 = 16,
    F17 = 17,
    F18 = 18,
    F19 = 19,
    F20 = 20,
    F21 = 21,
    F22 = 22,
    F23 = 23,
    F24 = 24,
    F25 = 25,
    F26 = 26,
    F27 = 27,
    F28 = 28,
    F29 = 29,
    F30 = 30,
    F31 = 31,
    F32 = 32,
    F33 = 33,
    F34 = 34,
    F35 = 35,
    F36 = 36
);
