use crate::{
    ComplexFunctions, ComplexFunctionsMut, ComplexTrait, Constant, FloatFunctions,
    FloatFunctionsMut, FloatTrait, NegAssign, Pow, RealTrait,
};
use lexical::{NumberFormatBuilder, ParseFloatOptions, WriteFloatOptions};
use std::cmp::Ordering;
#[cfg(feature = "f16")]
use std::f16::consts;
#[cfg(feature = "f32")]
use std::f32::consts;
#[cfg(feature = "f64")]
use std::f64::consts;
#[cfg(feature = "f128")]
use std::f128::consts;
use std::fmt::{Debug, Display, Formatter};
use std::iter::{Product, Sum};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};
use std::{fmt, mem};
#[cfg(feature = "f16")]
pub type F = f16;
#[cfg(feature = "f32")]
pub type F = f32;
#[cfg(feature = "f64")]
pub type F = f64;
#[cfg(feature = "f128")]
pub type F = f128;
#[cfg(feature = "fastnum")]
pub type F = fastnum::D1024;
#[derive(Clone, Default, PartialEq, PartialOrd)]
pub struct Integer(pub isize);
#[derive(Clone, Default, PartialEq, PartialOrd)]
pub struct Float(pub F);
#[derive(Clone, Default, PartialEq)]
pub struct Complex {
    pub real: Float,
    pub imag: Float,
}
impl Debug for Float {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl Debug for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}{}{:?}i",
            self.real,
            if self.imag.is_sign_positive() {
                "+"
            } else {
                ""
            },
            self.imag
        )
    }
}
impl Debug for Integer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match (
            self.real.is_zero(),
            self.imag.is_zero(),
            self.imag.is_sign_positive(),
        ) {
            (false, false, true) => write!(f, "{}+{}i", self.real, self.imag),
            (false, false, false) => write!(f, "{}{}i", self.real, self.imag),
            (false, true, _) => write!(f, "{}", self.real),
            (true, false, _) => write!(f, "{}i", self.imag),
            (true, true, _) => write!(f, "0"),
        }
    }
}
impl Display for Float {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Sum for Complex {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |sum, s| sum + s)
    }
}
impl Product for Complex {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::from(1.0), |sum, s| sum * s)
    }
}
impl Sum for Float {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |sum, s| sum + s)
    }
}
impl Product for Float {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::from(1.0), |sum, s| sum * s)
    }
}
impl RealTrait<Float> for Float {
    fn is_sign_negative(&self) -> bool {
        self.0.is_sign_negative()
    }
    fn is_sign_positive(&self) -> bool {
        self.0.is_sign_positive()
    }
    fn into_isize(self) -> isize {
        self.0 as isize
    }
    fn into_usize(self) -> usize {
        self.0 as usize
    }
    fn closest_fraction(&self) -> Option<(bool, usize, usize)> {
        if !self.0.is_finite() {
            return None;
        }
        let is_positive = self.is_sign_positive();
        let orig = self.0.abs();
        let mut num = orig;
        let mut mult = 1.0;
        for _ in 0..64 {
            let recip = num.recip();
            let fract = recip.fract();
            mult *= recip;
            num = fract;
            if fract < 1e-8 {
                let numerator = (orig * mult) as usize;
                let denominator = mult as usize;
                return Some((is_positive, numerator, denominator));
            }
        }
        None
    }
}
impl FloatFunctionsMut<Float> for Float {
    fn sin_mut(&mut self) {
        self.0 = self.0.sin();
    }
    fn cos_mut(&mut self) {
        self.0 = self.0.cos();
    }
    fn asin_mut(&mut self) {
        self.0 = self.0.asin();
    }
    fn acos_mut(&mut self) {
        self.0 = self.0.acos();
    }
    fn sinh_mut(&mut self) {
        self.0 = self.0.sinh();
    }
    fn cosh_mut(&mut self) {
        self.0 = self.0.cosh();
    }
    fn asinh_mut(&mut self) {
        self.0 = self.0.asinh();
    }
    fn acosh_mut(&mut self) {
        self.0 = self.0.acosh();
    }
    fn tan_mut(&mut self) {
        self.0 = self.0.tan()
    }
    fn tanh_mut(&mut self) {
        self.0 = self.0.tanh()
    }
    fn atan_mut(&mut self) {
        self.0 = self.0.atan()
    }
    fn atanh_mut(&mut self) {
        self.0 = self.0.atanh()
    }
    fn ln_mut(&mut self) {
        self.0 = self.0.ln();
    }
    fn exp_mut(&mut self) {
        self.0 = self.0.exp();
    }
    fn hypot_mut(&mut self, other: &Self) {
        self.0 = self.0.hypot(other.0);
    }
    fn atan2_mut(&mut self, other: &Self) {
        self.0 = self.0.atan2(other.0);
    }
    fn min_mut(&mut self, other: &Self) {
        self.0 = self.0.min(other.0);
    }
    fn max_mut(&mut self, other: &Self) {
        self.0 = self.0.max(other.0);
    }
    fn recip_mut(&mut self) {
        self.0 = self.0.recip();
    }
    fn sqrt_mut(&mut self) {
        self.0 = self.0.sqrt();
    }
    fn cbrt_mut(&mut self) {
        self.0 = self.0.cbrt();
    }
    fn abs_mut(&mut self) {
        self.0 = self.0.abs();
    }
    fn gamma_mut(&mut self) {
        self.0 = self.0.gamma();
    }
    fn erf_mut(&mut self) {
        self.0 = self.0.erf();
    }
    fn erfc_mut(&mut self) {
        self.0 = self.0.erfc();
    }
    fn round_mut(&mut self) {
        self.0 = self.0.round();
    }
    fn ceil_mut(&mut self) {
        self.0 = self.0.ceil();
    }
    fn floor_mut(&mut self) {
        self.0 = self.0.floor();
    }
    fn trunc_mut(&mut self) {
        self.0 = self.0.trunc();
    }
    fn fract_mut(&mut self) {
        self.0 = self.0.fract();
    }
    fn tetration_mut(&mut self, other: &Self) {
        fn tetration(a: Float, other: &Float) -> Float {
            let other = other.clone().round();
            if other.0 <= 0.0 {
                Float::from(1)
            } else {
                a.clone().pow(a.tetration(&(other - Float::from(1))))
            }
        }
        //TODO
        *self = tetration(self.clone(), other)
    }
    fn subfactorial_mut(&mut self) {
        //TODO
        if self.is_zero() {
            *self = Self::from(1);
        } else {
            *self = ((self.clone() + Float::from(1)).gamma() / Float::from(Constant::E)).round()
        }
    }
}
impl FloatTrait<Float> for Float {
    fn to_real(self) -> Self {
        self
    }
    fn real(&self) -> &Self {
        self
    }
    fn real_mut(&mut self) -> &mut Self {
        self
    }
    fn is_zero(&self) -> bool {
        self.0 == 0.0
    }
    fn parse_radix(src: &str, base: u8) -> Option<Self> {
        let options = ParseFloatOptions::from_radix(base);
        macro_rules! parses {
            ($($n:ident = $nu:expr),*) => {
                match base {
                    $(
                        $nu => lexical::parse_with_options::<F, _, $n>(src, &options).map(Self).ok(),
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
    fn to_string_radix(&self, base: u8) -> String {
        let options = WriteFloatOptions::from_radix(base);
        macro_rules! strings {
            ($($n:ident = $nu:expr),*) => {
                match base {
                    $(
                        $nu => lexical::to_string_with_options::<F, $n>(self.0, &options),
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
    fn get_closest_fraction(&self) -> impl Display {
        fmt::from_fn(move |fmt| {
            if let Some((sign, num, den)) = self.closest_fraction() {
                if !sign {
                    write!(fmt, "-")?;
                }
                write!(fmt, "{num}/{den}")
            } else {
                write!(fmt, "")
            }
        })
    }
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
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
impl ComplexFunctionsMut<Float> for Complex {
    fn arg_mut(&mut self) {
        self.imag.atan2_mut(&self.real);
        mem::swap(&mut self.real, &mut self.imag);
        self.zero_imag();
    }
    fn mul_i_mut(&mut self, negative: bool) {
        mem::swap(&mut self.real, &mut self.imag);
        if negative {
            self.imag.neg_assign();
        } else {
            self.real.neg_assign();
        }
    }
    fn conj_mut(&mut self) {
        self.imag.neg_assign();
    }
    fn norm_mut(&mut self) {
        self.real.0 = self.real.0.powi(2) + self.imag.0.powi(2);
        self.zero_imag();
    }
}
impl ComplexTrait<Float> for Complex {
    fn to_imag(self) -> Float {
        self.imag
    }
    fn imag(&self) -> &Float {
        &self.imag
    }
    fn imag_mut(&mut self) -> &mut Float {
        &mut self.imag
    }
    fn zero_real(&mut self) {
        self.real = Float::default();
        mem::swap(&mut self.real, &mut self.imag);
    }
    fn zero_imag(&mut self) {
        self.imag = Float::default()
    }
}
impl FloatFunctionsMut<Float> for Complex {
    fn sin_mut(&mut self) {
        let (a, b) = self.real.0.sin_cos();
        let (c, d) = (self.imag.0.sinh(), self.imag.0.cosh());
        *self = Self {
            real: Float(a * d),
            imag: Float(b * c),
        }
    }
    fn cos_mut(&mut self) {
        let (a, b) = self.real.0.sin_cos();
        let (c, d) = (self.imag.0.sinh(), self.imag.0.cosh());
        *self = Self {
            real: Float(b * d),
            imag: Float(-a * c),
        }
    }
    fn asin_mut(&mut self) {
        *self = self.clone().mul_i(false) + (Float::from(1) - self.clone() * self.clone()).sqrt();
        self.ln_mut();
        self.mul_i_mut(true);
    }
    fn acos_mut(&mut self) {
        self.asin_mut();
        *self -= Float::from(consts::FRAC_PI_2);
        self.neg_assign();
    }
    fn sinh_mut(&mut self) {
        self.exp_mut();
        *self -= self.clone().recip();
        *self /= Float::from(2);
    }
    fn cosh_mut(&mut self) {
        self.exp_mut();
        *self += self.clone().recip();
        *self /= Float::from(2);
    }
    fn asinh_mut(&mut self) {
        *self += (self.clone() * self.clone() + Float::from(1)).sqrt();
        self.ln_mut();
    }
    fn acosh_mut(&mut self) {
        *self += (self.clone() * self.clone() - Float::from(1)).sqrt();
        self.ln_mut();
    }
    fn tan_mut(&mut self) {
        let cos = self.clone().cos();
        self.sin_mut();
        *self /= cos;
    }
    fn tanh_mut(&mut self) {
        let cosh = self.clone().cosh();
        self.sinh_mut();
        *self /= cosh;
    }
    fn atan_mut(&mut self) {
        self.mul_i_mut(false);
        *self = (Float::from(1) - self.clone()) / (Float::from(1) + self.clone());
        self.ln_mut();
        self.mul_i_mut(false);
        *self /= Float::from(2);
    }
    fn atanh_mut(&mut self) {
        *self = (Float::from(1) + self.clone()) / (Float::from(1) - self.clone());
        self.ln_mut();
        *self /= Float::from(2);
    }
    fn ln_mut(&mut self) {
        *self = Self {
            real: self.clone().abs().ln(),
            imag: self.clone().arg(),
        }
    }
    fn exp_mut(&mut self) {
        let (imag, real) = self.imag.clone().0.sin_cos();
        let e = self.real.0.exp();
        self.real = Float(real * e);
        self.imag = Float(imag * e);
    }
    fn hypot_mut(&mut self, other: &Self) {
        if self.imag.is_zero() && other.imag.is_zero() {
            self.real.hypot_mut(&other.real)
        } else {
            *self = self.clone() * self.clone() + other.clone() * other.clone();
            self.sqrt_mut();
        }
    }
    fn atan2_mut(&mut self, other: &Self) {
        if self.imag.is_zero() && other.imag.is_zero() {
            self.real.atan2_mut(&other.real);
        } else {
            let d = self.clone().hypot(other);
            *self = self.clone().mul_i(false) + other;
            *self /= d;
            self.ln_mut();
            self.mul_i_mut(true)
        }
    }
    fn min_mut(&mut self, other: &Self) {
        self.real.min_mut(&other.real);
        self.imag.min_mut(&other.imag);
    }
    fn max_mut(&mut self, other: &Self) {
        self.real.max_mut(&other.real);
        self.imag.max_mut(&other.imag);
    }
    fn recip_mut(&mut self) {
        let d = self.clone().norm();
        self.conj_mut();
        *self /= d;
    }
    fn sqrt_mut(&mut self) {
        if self.imag.is_zero() {
            if self.real.is_sign_positive() {
                self.real.sqrt_mut()
            } else {
                self.real.abs_mut();
                self.real.sqrt_mut();
                self.mul_i_mut(false);
            }
        } else if self.real.is_zero() {
            self.imag /= Float::from(2);
            if self.imag.is_sign_positive() {
                self.imag.sqrt_mut();
                self.real = self.imag.clone();
            } else {
                self.imag.abs_mut();
                self.imag.sqrt_mut();
                mem::swap(&mut self.real, &mut self.imag);
                self.imag = -self.real.clone();
            }
        } else {
            let abs = self.clone().abs();
            let r = ((self.real.clone() + &abs) / Float::from(2)).sqrt();
            let i = ((abs - &self.real) / Float::from(2)).sqrt();
            let i = if self.imag.is_sign_positive() { i } else { -i };
            self.real = r;
            self.imag = i;
        }
    }
    fn cbrt_mut(&mut self) {
        if self.imag.is_zero() {
            self.real.cbrt_mut()
        } else if self.real.is_zero() {
            self.imag.cbrt_mut();
            self.imag.neg_assign();
        } else {
            let r = self.clone().abs().cbrt();
            let theta = self.clone().arg() / Float::from(3);
            let (sin, cos) = theta.0.sin_cos();
            self.real = Float(cos) * &r;
            self.imag = Float(sin) * r;
        }
    }
    fn abs_mut(&mut self) {
        self.real.hypot_mut(&self.imag);
        self.imag = Float(0.0);
    }
    fn gamma_mut(&mut self) {
        //TODO
        self.real.gamma_mut();
    }
    fn erf_mut(&mut self) {
        //TODO
        self.real.erf_mut();
    }
    fn erfc_mut(&mut self) {
        //TODO
        self.real.erfc_mut();
    }
    fn round_mut(&mut self) {
        self.real.round_mut();
        self.imag.round_mut();
    }
    fn ceil_mut(&mut self) {
        self.real.ceil_mut();
        self.imag.ceil_mut();
    }
    fn floor_mut(&mut self) {
        self.real.floor_mut();
        self.imag.floor_mut();
    }
    fn trunc_mut(&mut self) {
        self.real.trunc_mut();
        self.imag.trunc_mut();
    }
    fn fract_mut(&mut self) {
        self.real.fract_mut();
        self.imag.fract_mut();
    }
    fn tetration_mut(&mut self, other: &Self) {
        fn tetration(a: Complex, other: &Complex) -> Complex {
            let other = Complex::from(other.real.clone().round());
            if other.real.0 <= 0.0 {
                Complex::from(1)
            } else {
                a.clone().pow(a.tetration(&(other - Float::from(1))))
            }
        }
        //TODO
        *self = tetration(self.clone(), other)
    }
    fn subfactorial_mut(&mut self) {
        //TODO
        if self.is_zero() {
            *self = Self::from(1);
        } else {
            *self = ((self.clone() + Float::from(1)).gamma() / Float::from(Constant::E)).round()
        }
    }
}
impl FloatTrait<Float> for Complex {
    fn to_real(self) -> Float {
        self.real
    }
    fn real(&self) -> &Float {
        &self.real
    }
    fn real_mut(&mut self) -> &mut Float {
        &mut self.real
    }
    fn is_zero(&self) -> bool {
        self.real.is_zero() && self.imag.is_zero()
    }
    fn parse_radix(src: &str, base: u8) -> Option<Self> {
        Some(Self {
            real: Float::parse_radix(src, base)?,
            imag: Float(0.0),
        })
    }
    fn to_string_radix(&self, base: u8) -> String {
        format!(
            "{}{}{}i",
            self.real.to_string_radix(base),
            if self.imag.is_sign_positive() {
                "+"
            } else {
                ""
            },
            self.imag.to_string_radix(base)
        )
    }
    fn get_closest_fraction(&self) -> impl Display {
        fmt::from_fn(move |fmt| {
            if let Some((sign, num, den)) = self.real.closest_fraction() {
                if !sign {
                    write!(fmt, "-")?;
                }
                write!(fmt, "{num}/{den}")?;
            } else {
                write!(fmt, "")?;
            }
            if let Some((sign, num, den)) = self.imag.closest_fraction() {
                if sign {
                    write!(fmt, "+")?;
                } else {
                    write!(fmt, "-")?;
                }
                write!(fmt, "{num}i/{den}")
            } else {
                write!(fmt, "")
            }
        })
    }
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.real
            .total_cmp(&other.real)
            .then(self.imag.total_cmp(&other.imag))
    }
}
macro_rules! ops_assign {
    ($ty:ty, $assign:ident, $orig:ident, $assign_fun:ident, $orig_fun:ident) => {
        impl $assign<$ty> for $ty {
            default fn $assign_fun(&mut self, rhs: $ty) {
                $assign::$assign_fun(&mut self.0, rhs.0);
            }
        }
        impl $orig<$ty> for $ty {
            type Output = $ty;
            default fn $orig_fun(self, rhs: $ty) -> $ty {
                Self($orig::$orig_fun(self.0, rhs.0))
            }
        }
        impl<'a> $assign<&'a $ty> for $ty {
            default fn $assign_fun(&mut self, rhs: &'a $ty) {
                $assign::$assign_fun(&mut self.0, rhs.0);
            }
        }
        impl<'a> $orig<&'a $ty> for $ty {
            type Output = $ty;
            default fn $orig_fun(self, rhs: &'a $ty) -> $ty {
                Self($orig::$orig_fun(self.0, rhs.0))
            }
        }
    };
}
macro_rules! ops_assign_for {
    ($($ty:ty),*) => {
        $(
            ops_assign!($ty, MulAssign,Mul,mul_assign,mul);
            ops_assign!($ty, DivAssign,Div,div_assign,div);
            ops_assign!($ty, SubAssign,Sub,sub_assign,sub);
            ops_assign!($ty, AddAssign,Add,add_assign,add);
            ops_assign!($ty, RemAssign,Rem,rem_assign,rem);
        )*
    }
}
ops_assign_for!(Float, Integer);
impl From<Constant> for Float {
    fn from(value: Constant) -> Self {
        match value {
            Constant::Pi => Self(consts::PI),
            Constant::Tau => Self(consts::TAU),
            Constant::E => Self(consts::E),
            Constant::Infinity => Self(F::INFINITY),
            Constant::NegInfinity => Self(F::NEG_INFINITY),
            Constant::Nan => Self(F::NAN),
        }
    }
}
impl<T> From<T> for Complex
where
    Float: From<T>,
{
    fn from(value: T) -> Self {
        Complex {
            real: value.into(),
            imag: Float(0.0),
        }
    }
}
impl<T, K> From<(T, K)> for Complex
where
    Float: From<T> + From<K>,
{
    fn from(value: (T, K)) -> Self {
        Complex {
            real: value.0.into(),
            imag: value.1.into(),
        }
    }
}
macro_rules! with_val {
    ($($ty:ty),*) => {
        $(
            impl From<$ty> for Float
            {
                fn from(value: $ty) -> Self {
                    #[cfg(feature = "f16")]
                    {Self(value as f16)}
                    #[cfg(feature = "f32")]
                    {Self(value as f32)}
                    #[cfg(feature = "f64")]
                    {Self(value as f64)}
                    #[cfg(feature = "f128")]
                    {Self(value as f128)}
                }
            }
        )*
    };
}
with_val!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f16, f32, f64, f128
);
impl Pow<Float> for Float {
    type Output = Float;
    default fn pow(self, rhs: Float) -> Self {
        self.pow(&rhs)
    }
}
impl Pow<&Float> for Float {
    type Output = Float;
    default fn pow(self, rhs: &Float) -> Self {
        if rhs.0.fract() == 0.0
            && let Ok(rhs) = (rhs.0 as i64).try_into()
        {
            Self(self.0.powi(rhs))
        } else {
            Self(self.0.powf(rhs.0))
        }
    }
}
impl Neg for Float {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(self.0.neg())
    }
}
impl Neg for Integer {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(self.0.neg())
    }
}
impl Pow<&Complex> for Float {
    type Output = Complex;
    fn pow(self, rhs: &Complex) -> Complex {
        self.pow(rhs.clone())
    }
}
impl Pow<Complex> for Float {
    type Output = Complex;
    fn pow(self, rhs: Complex) -> Complex {
        if rhs.imag.is_zero() {
            self.pow(rhs.real).into()
        } else {
            (rhs * self.ln()).exp()
        }
    }
}
impl Pow<&Self> for Complex {
    type Output = Complex;
    fn pow(self, rhs: &Self) -> Self {
        self.pow(rhs.clone())
    }
}
impl Pow<Self> for Complex {
    type Output = Complex;
    fn pow(self, rhs: Self) -> Self {
        if rhs.imag.is_zero() {
            self.pow(rhs.real)
        } else if self.imag.is_zero() {
            (rhs * self.real.ln()).exp()
        } else {
            (self.ln() * rhs).exp()
        }
    }
}
impl Pow<&Float> for Complex {
    type Output = Complex;
    fn pow(self, rhs: &Float) -> Self {
        self.pow(rhs.clone())
    }
}
impl Pow<Float> for Complex {
    type Output = Complex;
    fn pow(self, rhs: Float) -> Self {
        if self.imag.is_zero() {
            if self.real.is_sign_negative() {
                let fract = rhs.clone().fract();
                if fract.is_zero() {
                    self.real.pow(rhs).into()
                } else if fract.0 == 0.5 {
                    Complex::from(self.real.abs().pow(rhs)).mul_i(false)
                } else {
                    (self.ln() * rhs).exp()
                }
            } else {
                self.real.pow(rhs).into()
            }
        } else if self.real.is_zero()
            && rhs.0.fract() == 0.0
            && let Ok(rhs) = (rhs.0 as i64).try_into()
        {
            let f = self.imag.0.powi(rhs);
            match rhs.rem_euclid(4) {
                0 => Self {
                    real: Float(f),
                    imag: Float::default(),
                },
                1 => Self {
                    real: Float::default(),
                    imag: Float(f),
                },
                2 => Self {
                    real: Float(-f),
                    imag: Float::default(),
                },
                3 => Self {
                    real: Float::default(),
                    imag: Float(-f),
                },
                _ => unreachable!(),
            }
        } else {
            (self.ln() * rhs).exp()
        }
    }
}
impl Mul<Self> for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real.clone() * rhs.real.clone() - self.imag.clone() * rhs.imag.clone(),
            imag: self.real * rhs.imag + self.imag * rhs.real,
        }
    }
}
impl MulAssign<Self> for Complex {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}
impl Div<Self> for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let norm = rhs.clone().norm();
        Self {
            real: self.real.clone() * rhs.real.clone() + self.imag.clone() * rhs.imag.clone(),
            imag: self.imag * rhs.real - self.real * rhs.imag,
        } / norm
    }
}
impl DivAssign<Self> for Complex {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.clone() / rhs;
    }
}
impl Add<Self> for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}
impl AddAssign<Float> for Complex {
    fn add_assign(&mut self, rhs: Float) {
        *self = self.clone() + rhs;
    }
}
impl AddAssign<Self> for Complex {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}
impl Sub<Self> for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag,
        }
    }
}
impl SubAssign<Self> for Complex {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone() - rhs;
    }
}
impl Rem<Self> for Complex {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real % rhs.real.clone(),
            imag: self.imag % rhs.real,
        }
    }
}
impl RemAssign<Self> for Complex {
    fn rem_assign(&mut self, rhs: Self) {
        *self = self.clone() % rhs;
    }
}
impl Rem<Float> for Complex {
    type Output = Self;
    fn rem(self, rhs: Float) -> Self::Output {
        Self {
            real: self.real % rhs.clone(),
            imag: self.imag % rhs,
        }
    }
}
impl Neg for Complex {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            real: -self.real,
            imag: -self.imag,
        }
    }
}
impl From<bool> for Float {
    fn from(value: bool) -> Self {
        Self::from(value as u8)
    }
}
impl Mul<Float> for Complex {
    type Output = Self;
    default fn mul(self, rhs: Float) -> Self::Output {
        Self {
            real: self.real * rhs.clone(),
            imag: self.imag * rhs,
        }
    }
}
impl MulAssign<Float> for Complex {
    fn mul_assign(&mut self, rhs: Float) {
        *self = self.clone() * rhs;
    }
}
impl Div<Float> for Complex {
    type Output = Self;
    fn div(self, rhs: Float) -> Self::Output {
        Self {
            real: self.real / rhs.clone(),
            imag: self.imag / rhs,
        }
    }
}
impl DivAssign<Float> for Complex {
    fn div_assign(&mut self, rhs: Float) {
        *self = self.clone() / rhs;
    }
}
impl Add<Float> for Complex {
    type Output = Self;
    fn add(self, rhs: Float) -> Self::Output {
        Self {
            real: self.real + rhs,
            imag: self.imag,
        }
    }
}
impl Sub<Float> for Complex {
    type Output = Self;
    fn sub(self, rhs: Float) -> Self::Output {
        Self {
            real: self.real - rhs,
            imag: self.imag,
        }
    }
}
impl SubAssign<Float> for Complex {
    fn sub_assign(&mut self, rhs: Float) {
        *self = self.clone() - rhs;
    }
}
impl Rem<Complex> for Float {
    type Output = Complex;
    fn rem(self, rhs: Complex) -> Self::Output {
        Complex {
            real: self.clone() % rhs.real,
            imag: self % rhs.imag,
        }
    }
}
impl RemAssign<Float> for Complex {
    fn rem_assign(&mut self, rhs: Float) {
        *self = self.clone() % rhs;
    }
}
impl Mul<Complex> for Float {
    type Output = Complex;
    default fn mul(self, rhs: Complex) -> Self::Output {
        Complex {
            real: self.clone() * rhs.real,
            imag: self * rhs.imag,
        }
    }
}
impl Div<Complex> for Float {
    type Output = Complex;
    fn div(self, rhs: Complex) -> Self::Output {
        let norm = rhs.clone().norm();
        Complex {
            real: self.clone() * rhs.real,
            imag: -self * rhs.imag,
        } / norm
    }
}
impl Add<Complex> for Float {
    type Output = Complex;
    fn add(self, rhs: Complex) -> Self::Output {
        Complex {
            real: self + rhs.real,
            imag: rhs.imag,
        }
    }
}
impl Sub<Complex> for Float {
    type Output = Complex;
    fn sub(self, rhs: Complex) -> Self::Output {
        Complex {
            real: self - rhs.real,
            imag: -rhs.imag,
        }
    }
}
impl Mul<&Self> for Complex {
    type Output = Self;
    fn mul(self, rhs: &Self) -> Self::Output {
        Self {
            real: self.real.clone() * rhs.real.clone() - self.imag.clone() * rhs.imag.clone(),
            imag: self.real * &rhs.imag + self.imag * &rhs.real,
        }
    }
}
impl MulAssign<&Self> for Complex {
    fn mul_assign(&mut self, rhs: &Self) {
        *self = self.clone() * rhs;
    }
}
impl Div<&Self> for Complex {
    type Output = Self;
    fn div(self, rhs: &Self) -> Self::Output {
        let norm = rhs.clone().norm();
        Self {
            real: self.real.clone() * rhs.real.clone() + self.imag.clone() * rhs.imag.clone(),
            imag: self.imag * &rhs.real - self.real * &rhs.imag,
        } / norm
    }
}
impl DivAssign<&Self> for Complex {
    fn div_assign(&mut self, rhs: &Self) {
        *self = self.clone() / rhs;
    }
}
impl Add<&Self> for Complex {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self::Output {
        Self {
            real: self.real + &rhs.real,
            imag: self.imag + &rhs.imag,
        }
    }
}
impl AddAssign<&Float> for Complex {
    fn add_assign(&mut self, rhs: &Float) {
        *self = self.clone() + rhs;
    }
}
impl AddAssign<&Self> for Complex {
    fn add_assign(&mut self, rhs: &Self) {
        *self = self.clone() + rhs;
    }
}
impl Sub<&Self> for Complex {
    type Output = Self;
    fn sub(self, rhs: &Self) -> Self::Output {
        Self {
            real: self.real - &rhs.real,
            imag: self.imag - &rhs.imag,
        }
    }
}
impl SubAssign<&Self> for Complex {
    fn sub_assign(&mut self, rhs: &Self) {
        *self = self.clone() - rhs;
    }
}
impl Rem<&Self> for Complex {
    type Output = Self;
    fn rem(self, rhs: &Self) -> Self::Output {
        Self {
            real: self.real % rhs.real.clone(),
            imag: self.imag % &rhs.real,
        }
    }
}
impl RemAssign<&Self> for Complex {
    fn rem_assign(&mut self, rhs: &Self) {
        *self = self.clone() % rhs;
    }
}
impl Mul<&Float> for Complex {
    type Output = Self;
    default fn mul(self, rhs: &Float) -> Self::Output {
        Self {
            real: self.real * rhs.clone(),
            imag: self.imag * rhs,
        }
    }
}
impl MulAssign<&Float> for Complex {
    fn mul_assign(&mut self, rhs: &Float) {
        *self = self.clone() * rhs;
    }
}
impl Div<&Float> for Complex {
    type Output = Self;
    fn div(self, rhs: &Float) -> Self::Output {
        Self {
            real: self.real / rhs.clone(),
            imag: self.imag / rhs,
        }
    }
}
impl DivAssign<&Float> for Complex {
    fn div_assign(&mut self, rhs: &Float) {
        *self = self.clone() / rhs;
    }
}
impl Add<&Float> for Complex {
    type Output = Self;
    fn add(self, rhs: &Float) -> Self::Output {
        Self {
            real: self.real + rhs,
            imag: self.imag,
        }
    }
}
impl Sub<&Float> for Complex {
    type Output = Self;
    fn sub(self, rhs: &Float) -> Self::Output {
        Self {
            real: self.real - rhs,
            imag: self.imag,
        }
    }
}
impl SubAssign<&Float> for Complex {
    fn sub_assign(&mut self, rhs: &Float) {
        *self = self.clone() - rhs;
    }
}
impl Rem<&Float> for Complex {
    type Output = Self;
    fn rem(self, rhs: &Float) -> Self::Output {
        Self {
            real: self.real % rhs.clone(),
            imag: self.imag % rhs,
        }
    }
}
impl RemAssign<&Float> for Complex {
    fn rem_assign(&mut self, rhs: &Float) {
        *self = self.clone() % rhs;
    }
}
impl Mul<&Complex> for Float {
    type Output = Complex;
    default fn mul(self, rhs: &Complex) -> Self::Output {
        Complex {
            real: self.clone() * &rhs.real,
            imag: self * &rhs.imag,
        }
    }
}
impl Div<&Complex> for Float {
    type Output = Complex;
    fn div(self, rhs: &Complex) -> Self::Output {
        let norm = rhs.clone().norm();
        Complex {
            real: self.clone() * &rhs.real,
            imag: -self * &rhs.imag,
        } / norm
    }
}
impl Add<&Complex> for Float {
    type Output = Complex;
    fn add(self, rhs: &Complex) -> Self::Output {
        Complex {
            real: self + &rhs.real,
            imag: rhs.imag.clone(),
        }
    }
}
impl Sub<&Complex> for Float {
    type Output = Complex;
    fn sub(self, rhs: &Complex) -> Self::Output {
        Complex {
            real: self - &rhs.real,
            imag: -rhs.imag.clone(),
        }
    }
}
