use crate::{Constant, Pow, PowAssign};
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl Debug for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}i",
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
impl Display for Float {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
impl Float {
    pub fn real(&self) -> &Self {
        self
    }
    pub fn real_mut(&mut self) -> &mut Self {
        self
    }
    pub fn into_isize(self) -> isize {
        self.0 as isize
    }
    pub fn sin_cos(self) -> (Self, Self) {
        let (sin, cos) = self.0.sin_cos();
        (Self(sin), Self(cos))
    }
    pub fn is_sign_negative(&self) -> bool {
        self.0.is_sign_negative()
    }
    pub fn is_zero(&self) -> bool {
        self.0 == 0.0
    }
    pub fn is_sign_positive(&self) -> bool {
        self.0.is_sign_positive()
    }
    pub fn sin_mut(&mut self) {
        *self = self.clone().sin();
    }
    pub fn sin(self) -> Self {
        Self(self.0.sin())
    }
    pub fn cos_mut(&mut self) {
        *self = self.clone().cos();
    }
    pub fn cos(self) -> Self {
        Self(self.0.cos())
    }
    pub fn asin_mut(&mut self) {
        *self = self.clone().asin();
    }
    pub fn asin(self) -> Self {
        Self(self.0.asin())
    }
    pub fn acos_mut(&mut self) {
        *self = self.clone().acos();
    }
    pub fn acos(self) -> Self {
        Self(self.0.acos())
    }
    pub fn sinh_mut(&mut self) {
        *self = self.clone().sinh();
    }
    pub fn sinh(self) -> Self {
        Self(self.0.sinh())
    }
    pub fn cosh_mut(&mut self) {
        *self = self.clone().cosh();
    }
    pub fn cosh(self) -> Self {
        Self(self.0.cosh())
    }
    pub fn asinh_mut(&mut self) {
        *self = self.clone().asinh();
    }
    pub fn asinh(self) -> Self {
        Self(self.0.asinh())
    }
    pub fn acosh_mut(&mut self) {
        *self = self.clone().acosh();
    }
    pub fn acosh(self) -> Self {
        Self(self.0.acosh())
    }
    pub fn tan_mut(&mut self) {
        self.0 = self.0.tan()
    }
    pub fn tan(mut self) -> Self {
        self.tan_mut();
        self
    }
    pub fn tanh_mut(&mut self) {
        self.0 = self.0.tanh()
    }
    pub fn tanh(mut self) -> Self {
        self.tanh_mut();
        self
    }
    pub fn atan_mut(&mut self) {
        self.0 = self.0.atan()
    }
    pub fn atan(mut self) -> Self {
        self.atan_mut();
        self
    }
    pub fn atanh_mut(&mut self) {
        self.0 = self.0.atanh()
    }
    pub fn atanh(mut self) -> Self {
        self.atanh_mut();
        self
    }
    pub fn ln_mut(&mut self) {
        *self = self.clone().ln();
    }
    pub fn ln(self) -> Self {
        Self(self.0.ln())
    }
    pub fn exp_mut(&mut self) {
        *self = self.clone().exp();
    }
    pub fn exp(self) -> Self {
        Self(self.0.exp())
    }
    pub fn atan2_mut(&mut self, other: &Self) {
        *self = self.clone().atan2(other);
    }
    pub fn atan2(self, other: &Self) -> Self {
        Self(self.0.atan2(other.0))
    }
    pub fn min_mut(&mut self, other: &Self) {
        *self = self.clone().min(other);
    }
    pub fn min(self, other: &Self) -> Self {
        Self(self.0.min(other.0))
    }
    pub fn max_mut(&mut self, other: &Self) {
        *self = self.clone().max(other);
    }
    pub fn max(self, other: &Self) -> Self {
        Self(self.0.max(other.0))
    }
    pub fn recip_mut(&mut self) {
        *self = self.clone().recip();
    }
    pub fn recip(self) -> Self {
        Self(self.0.recip())
    }
    pub fn sqrt_mut(&mut self) {
        *self = self.clone().sqrt();
    }
    pub fn sqrt(self) -> Self {
        Self(self.0.sqrt())
    }
    pub fn hypot_mut(&mut self, other: &Self) {
        self.0 = self.0.hypot(other.0);
    }
    pub fn abs_mut(&mut self) {
        self.0 = self.0.abs();
    }
    pub fn abs(mut self) -> Self {
        self.abs_mut();
        self
    }
    pub fn gamma_mut(&mut self) {
        self.0 = self.0.gamma();
    }
    pub fn gamma(mut self) -> Self {
        self.gamma_mut();
        self
    }
    pub fn erf_mut(&mut self) {
        self.0 = self.0.erf();
    }
    pub fn erf(mut self) -> Self {
        self.erf_mut();
        self
    }
    pub fn erfc_mut(&mut self) {
        self.0 = self.0.erfc();
    }
    pub fn erfc(mut self) -> Self {
        self.erfc_mut();
        self
    }
    pub fn total_cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
    pub fn round_mut(&mut self) {
        self.0 = self.0.round();
    }
    pub fn round(mut self) -> Self {
        self.round_mut();
        self
    }
    pub fn ceil_mut(&mut self) {
        self.0 = self.0.ceil();
    }
    pub fn ceil(mut self) -> Self {
        self.ceil_mut();
        self
    }
    pub fn floor_mut(&mut self) {
        self.0 = self.0.floor();
    }
    pub fn floor(mut self) -> Self {
        self.floor_mut();
        self
    }
    pub fn trunc_mut(&mut self) {
        self.0 = self.0.trunc();
    }
    pub fn trunc(mut self) -> Self {
        self.trunc_mut();
        self
    }
    pub fn fract_mut(&mut self) {
        self.0 = self.0.fract();
    }
    pub fn fract(mut self) -> Self {
        self.fract_mut();
        self
    }
    pub fn tetration_mut(&mut self, other: &Self) {
        //TODO
        *self = self.clone().tetration(other)
    }
    pub fn tetration(self, other: &Self) -> Self {
        let other = other.clone().round();
        if other.0 <= 0.0 {
            Float::from(1)
        } else {
            self.clone().pow(self.tetration(&(other - Float::from(1))))
        }
    }
    pub fn subfactorial_mut(&mut self) {
        //TODO
        if self.is_zero() {
            *self = Self::from(1);
        } else {
            *self = ((self.clone() + Float::from(1)).gamma() / Float::from(Constant::E)).round()
        }
    }
    pub fn subfactorial(mut self) -> Self {
        self.subfactorial_mut();
        self
    }
    pub fn parse_radix(src: &str, _: i32) -> Result<Self, ()> {
        //TODO
        src.parse().map(Self).map_err(|_| ())
    }
}
impl Complex {
    pub fn real(&self) -> &Float {
        &self.real
    }
    pub fn imag(&self) -> &Float {
        &self.imag
    }
    pub fn real_mut(&mut self) -> &mut Float {
        &mut self.real
    }
    pub fn imag_mut(&mut self) -> &mut Float {
        &mut self.imag
    }
    pub fn zero_real(&mut self) {
        self.real = Float::default()
    }
    pub fn zero_imag(&mut self) {
        self.imag = Float::default()
    }
    pub fn parse_radix(src: &str, _: i32) -> Result<Self, ()> {
        //TODO
        src.parse()
            .map(|real| Self {
                real: Float(real),
                imag: Float(0.0),
            })
            .map_err(|_| ())
    }
    pub fn tetration_mut(&mut self, other: &Self) {
        //TODO
        *self = self.clone().tetration(other)
    }
    pub fn tetration(self, other: &Self) -> Self {
        let other = Complex::from(other.real.clone().round());
        if other.real.0 <= 0.0 {
            Complex::from(1)
        } else {
            self.clone().pow(self.tetration(&(other - Float::from(1))))
        }
    }
    pub fn subfactorial_mut(&mut self) {
        //TODO
        if self.is_zero() {
            *self = Self::from(1);
        } else {
            *self = ((self.clone() + Float::from(1)).gamma() / Float::from(Constant::E)).round()
        }
    }
    pub fn subfactorial(mut self) -> Self {
        self.subfactorial_mut();
        self
    }
    pub fn round_mut(&mut self) {
        self.real.round_mut();
        self.imag.round_mut();
    }
    pub fn round(mut self) -> Self {
        self.round_mut();
        self
    }
    pub fn ceil_mut(&mut self) {
        self.real.ceil_mut();
        self.imag.ceil_mut();
    }
    pub fn ceil(mut self) -> Self {
        self.ceil_mut();
        self
    }
    pub fn floor_mut(&mut self) {
        self.real.floor_mut();
        self.imag.floor_mut();
    }
    pub fn floor(mut self) -> Self {
        self.floor_mut();
        self
    }
    pub fn trunc_mut(&mut self) {
        self.real.trunc_mut();
        self.imag.trunc_mut();
    }
    pub fn trunc(mut self) -> Self {
        self.trunc_mut();
        self
    }
    pub fn fract_mut(&mut self) {
        self.real.fract_mut();
        self.imag.fract_mut();
    }
    pub fn fract(mut self) -> Self {
        self.fract_mut();
        self
    }
    pub fn tan_mut(&mut self) {
        let cos = self.clone().cos();
        self.sin_mut();
        *self /= cos;
    }
    pub fn tan(mut self) -> Self {
        self.tan_mut();
        self
    }
    pub fn tanh_mut(&mut self) {
        let cosh = self.clone().cosh();
        self.sinh_mut();
        *self /= cosh;
    }
    pub fn tanh(mut self) -> Self {
        self.tanh_mut();
        self
    }
    pub fn atanh_mut(&mut self) {
        *self = ((self.clone() + Float::from(1)).ln() - (Float::from(1) - self.clone()).ln())
            / Float::from(2)
    }
    pub fn atanh(mut self) -> Self {
        self.atanh_mut();
        self
    }
    pub fn atan_mut(&mut self) {
        *self = ((Float::from(1) - self.clone().mul_i(false))
            .ln()
            .mul_i(false)
            - (self.clone().mul_i(false) + Float::from(1))
                .ln()
                .mul_i(false))
            / Float::from(2)
    }
    pub fn atan(mut self) -> Self {
        self.atan_mut();
        self
    }
    pub fn asinh_mut(&mut self) {
        *self = (self.clone() + (self.clone() * self.clone() + Float::from(1)).sqrt()).ln();
    }
    pub fn asinh(mut self) -> Self {
        self.asinh_mut();
        self
    }
    pub fn acosh_mut(&mut self) {
        *self = (self.clone() + (self.clone() * self.clone() - Float::from(1)).sqrt()).ln();
    }
    pub fn acosh(mut self) -> Self {
        self.acosh_mut();
        self
    }
    pub fn sinh_mut(&mut self) {
        *self = (self.clone().exp() - self.clone().neg().exp()) / Float::from(2);
    }
    pub fn sinh(mut self) -> Self {
        self.sinh_mut();
        self
    }
    pub fn cosh_mut(&mut self) {
        *self = (self.clone().exp() + self.clone().neg().exp()) / Float::from(2);
    }
    pub fn cosh(mut self) -> Self {
        self.cosh_mut();
        self
    }
    pub fn gamma_mut(&mut self) {
        //TODO
        self.real.gamma_mut();
    }
    pub fn gamma(mut self) -> Self {
        self.gamma_mut();
        self
    }
    pub fn erf_mut(&mut self) {
        //TODO
        self.real.erf_mut();
    }
    pub fn erf(mut self) -> Self {
        self.erf_mut();
        self
    }
    pub fn erfc_mut(&mut self) {
        //TODO
        self.real.erfc_mut();
    }
    pub fn erfc(mut self) -> Self {
        self.erfc_mut();
        self
    }
    pub fn norm(self) -> Float {
        self.real.clone() * self.real + self.imag.clone() * self.imag
    }
    pub fn abs_mut(&mut self) {
        self.real.hypot_mut(&self.imag);
        self.imag = Float(0.0);
    }
    pub fn abs(mut self) -> Float {
        self.real.hypot_mut(&self.imag);
        self.real
    }
    pub fn sin_mut(&mut self) {
        *self = Self {
            real: self.real.clone().sin() * self.imag.clone().cosh(),
            imag: self.imag.clone().sinh() * self.real.clone().cos(),
        }
    }
    pub fn cos_mut(&mut self) {
        *self = Self {
            real: self.real.clone().cos() * self.imag.clone().cosh(),
            imag: -self.imag.clone().sinh() * self.real.clone().sin(),
        }
    }
    pub fn sin(mut self) -> Self {
        self.sin_mut();
        self
    }
    pub fn cos(mut self) -> Self {
        self.cos_mut();
        self
    }
    pub fn asin_mut(&mut self) {
        *self = (self.clone().mul_i(false) + (Float::from(1) - self.clone() * self.clone()).sqrt())
            .ln()
            .mul_i(true);
    }
    pub fn acos_mut(&mut self) {
        self.asin_mut();
        *self = Float::from(consts::FRAC_PI_2) - self.clone()
    }
    pub fn asin(mut self) -> Self {
        self.asin_mut();
        self
    }
    pub fn acos(mut self) -> Self {
        self.acos_mut();
        self
    }
    pub fn arg_mut(&mut self) {
        self.real = self.imag.clone().atan2(&self.real);
        self.imag = Float(0.0)
    }
    pub fn arg(self) -> Float {
        self.imag.atan2(&self.real)
    }
    pub fn ln_mut(&mut self) {
        *self = Self {
            real: self.clone().abs().ln(),
            imag: self.clone().arg(),
        }
    }
    pub fn ln(mut self) -> Self {
        self.ln_mut();
        self
    }
    pub fn exp_mut(&mut self) {
        let (imag, real) = self.imag.clone().sin_cos();
        *self = Self { real, imag } * self.real.clone().exp();
    }
    pub fn exp(mut self) -> Self {
        self.exp_mut();
        self
    }
    pub fn is_zero(&self) -> bool {
        self.real.is_zero() && self.imag.is_zero()
    }
    pub fn atan2_mut(&mut self, other: &Self) {
        if self.imag.is_zero() && other.imag.is_zero() {
            self.real.atan2_mut(&other.real);
        } else {
            *self = ((self.clone().mul_i(false) + other.clone())
                / (self.clone() * self.clone() + other.clone() * other.clone()).sqrt())
            .ln()
            .mul_i(true)
        }
    }
    pub fn total_cmp(&self, other: &Self) -> Ordering {
        self.real
            .total_cmp(&other.real)
            .then(self.imag.total_cmp(&other.imag))
    }
    pub fn atan2(mut self, other: &Self) -> Self {
        self.atan2_mut(other);
        self
    }
    pub fn mul_i_mut(&mut self, negative: bool) {
        *self *= Complex::from(if negative { (0, -1) } else { (0, 1) })
    }
    pub fn mul_i(mut self, negative: bool) -> Self {
        self.mul_i_mut(negative);
        self
    }
    pub fn sqrt_mut(&mut self) {
        self.pow_assign(Float::from(0.5))
    }
    pub fn sqrt(mut self) -> Self {
        self.sqrt_mut();
        self
    }
    pub fn recip_mut(&mut self) {
        *self = self.clone().conj() / self.clone().norm()
    }
    pub fn recip(mut self) -> Self {
        self.recip_mut();
        self
    }
    pub fn conj_mut(&mut self) {
        self.imag = self.imag.clone().neg();
    }
    pub fn conj(mut self) -> Self {
        self.conj_mut();
        self
    }
    pub fn min_mut(&mut self, other: &Self) {
        self.real.min_mut(&other.real);
        self.imag.min_mut(&other.imag);
    }
    pub fn max_mut(&mut self, other: &Self) {
        self.real.max_mut(&other.real);
        self.imag.max_mut(&other.imag);
    }
}
impl Pow<Float> for Float {
    type Output = Float;
    default fn pow(self, rhs: Float) -> Self {
        Self(self.0.powf(rhs.0))
    }
}
macro_rules! ops_assign {
    ($ty:ty, $assign:ident, $orig:ident, $assign_fun:ident, $orig_fun:ident) => {
        impl $assign<$ty> for $ty {
            default fn $assign_fun(&mut self, rhs: $ty) {
                $assign::$assign_fun(&mut self.0, Self::from(rhs).0);
            }
        }
        impl $orig<$ty> for $ty {
            type Output = $ty;
            default fn $orig_fun(self, rhs: $ty) -> $ty {
                Self($orig::$orig_fun(self.0, Self::from(rhs).0))
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
impl Neg for Complex {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            real: -self.real,
            imag: -self.imag,
        }
    }
}
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
impl Rem<Float> for Complex {
    type Output = Self;
    fn rem(self, rhs: Float) -> Self::Output {
        Self {
            real: self.real % rhs.clone(),
            imag: self.imag % rhs,
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
