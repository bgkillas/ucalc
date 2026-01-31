use crate::{Constant, Pow};
#[cfg(feature = "f16")]
use std::f16::consts;
#[cfg(feature = "f32")]
use std::f32::consts;
#[cfg(feature = "f64")]
use std::f64::consts;
#[cfg(feature = "f128")]
use std::f128::consts;
use std::fmt::{Display, Formatter};
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
#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd)]
pub struct Integer(pub i128);
#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd)]
pub struct Float(pub F);
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Complex {
    pub real: Float,
    pub imag: Float,
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
impl Float {
    pub fn is_sign_negative(&self) -> bool {
        self.0.is_sign_negative()
    }
    pub fn is_sign_positive(&self) -> bool {
        self.0.is_sign_positive()
    }
    pub fn sin_mut(&mut self) {
        *self = self.sin();
    }
    pub fn sin(self) -> Self {
        Self(self.0.sin())
    }
    pub fn cos_mut(&mut self) {
        *self = self.cos();
    }
    pub fn cos(self) -> Self {
        Self(self.0.cos())
    }
    pub fn asin_mut(&mut self) {
        *self = self.asin();
    }
    pub fn asin(self) -> Self {
        Self(self.0.asin())
    }
    pub fn acos_mut(&mut self) {
        *self = self.acos();
    }
    pub fn acos(self) -> Self {
        Self(self.0.acos())
    }
    pub fn ln_mut(&mut self) {
        *self = self.ln();
    }
    pub fn ln(self) -> Self {
        Self(self.0.ln())
    }
    pub fn exp_mut(&mut self) {
        *self = self.exp();
    }
    pub fn exp(self) -> Self {
        Self(self.0.exp())
    }
    pub fn atan2_mut(&mut self, other: &Self) {
        *self = self.atan2(other);
    }
    pub fn atan2(self, other: &Self) -> Self {
        Self(self.0.atan2(other.0))
    }
    pub fn min_mut(&mut self, other: &Self) {
        *self = self.min(other);
    }
    pub fn min(self, other: &Self) -> Self {
        Self(self.0.min(other.0))
    }
    pub fn max_mut(&mut self, other: &Self) {
        *self = self.max(other);
    }
    pub fn max(self, other: &Self) -> Self {
        Self(self.0.max(other.0))
    }
    pub fn recip_mut(&mut self) {
        *self = self.recip();
    }
    pub fn recip(self) -> Self {
        Self(self.0.recip())
    }
    pub fn sqrt_mut(&mut self) {
        *self = self.sqrt();
    }
    pub fn sqrt(self) -> Self {
        Self(self.0.sqrt())
    }
}
impl Complex {
    pub fn parse_radix(src: &str, _: i32) -> Result<Self, ()> {
        //TODO
        src.parse()
            .map(|real| Self {
                real: Float(real),
                imag: Float(0.0),
            })
            .map_err(|_| ())
    }
    pub fn norm_mut(&mut self) {
        self.real = self.real * self.real + self.imag * self.imag;
        self.imag = Float(0.0);
    }
    pub fn norm(mut self) -> Self {
        self.norm_mut();
        self
    }
    pub fn sin_mut(&mut self) {
        //TODO
        self.real.sin_mut();
    }
    pub fn cos_mut(&mut self) {
        //TODO
        self.real.cos_mut();
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
        //TODO
        self.real.asin_mut();
    }
    pub fn acos_mut(&mut self) {
        //TODO
        self.real.acos_mut();
    }
    pub fn asin(mut self) -> Self {
        self.asin_mut();
        self
    }
    pub fn acos(mut self) -> Self {
        self.acos_mut();
        self
    }
    pub fn ln_mut(&mut self) {
        //TODO
        self.real.ln_mut();
    }
    pub fn exp_mut(&mut self) {
        //TODO
        self.real.exp_mut();
    }
    pub fn atan2_mut(&mut self, other: &Self) {
        //TODO
        self.real.atan2_mut(&other.real);
    }
    pub fn sqrt_mut(&mut self) {
        //TODO
        self.real.sqrt_mut();
    }
    pub fn sqrt(mut self) -> Self {
        self.sqrt_mut();
        self
    }
    pub fn recip_mut(&mut self) {
        //TODO
        self.real.recip_mut();
    }
    pub fn recip(mut self) -> Self {
        self.recip_mut();
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
impl Pow<Self> for Float {
    fn pow(self, rhs: Self) -> Self {
        Self(self.0.powf(rhs.0))
    }
}
impl Mul<Self> for Float {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}
impl MulAssign<Self> for Float {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}
impl Div<Self> for Float {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}
impl DivAssign<Self> for Float {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}
impl Add<Self> for Float {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl AddAssign<Self> for Float {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
impl Sub<Self> for Float {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
impl SubAssign<Self> for Float {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}
impl Rem<Self> for Float {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0 % rhs.0)
    }
}
impl RemAssign<Self> for Float {
    fn rem_assign(&mut self, rhs: Self) {
        self.0 %= rhs.0;
    }
}
impl Neg for Float {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(self.0.neg())
    }
}
impl Pow<Self> for Complex {
    fn pow(self, rhs: Self) -> Self {
        //TODO
        self.real.pow(rhs.real).into()
    }
}
impl Mul<Self> for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real * rhs.real - self.imag * rhs.imag,
            imag: self.real * rhs.imag + self.imag * rhs.real,
        }
    }
}
impl MulAssign<Self> for Complex {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
impl Div<Self> for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let norm = rhs.norm();
        Self {
            real: self.real * rhs.real + self.imag * rhs.imag,
            imag: self.imag * rhs.real - self.real * rhs.imag,
        } / norm.real
    }
}
impl DivAssign<Self> for Complex {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
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
        *self = *self + rhs;
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
        *self = *self - rhs;
    }
}
impl Rem<Self> for Complex {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real % rhs.real,
            imag: self.imag % rhs.real,
        }
    }
}
impl RemAssign<Self> for Complex {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
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
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
);
impl<T> Mul<T> for Complex
where
    Float: From<T>,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self {
            real: self.real * rhs,
            imag: self.imag * rhs,
        }
    }
}
impl<T> MulAssign<T> for Complex
where
    Float: From<T>,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}
impl<T> Div<T> for Complex
where
    Float: From<T>,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self {
            real: self.real / rhs,
            imag: self.imag / rhs,
        }
    }
}
impl<T> DivAssign<T> for Complex
where
    Float: From<T>,
{
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}
impl<T> Add<T> for Complex
where
    Float: From<T>,
{
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self {
            real: self.real + rhs,
            imag: self.imag,
        }
    }
}
impl<T> AddAssign<T> for Complex
where
    Float: From<T>,
{
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}
impl<T> Sub<T> for Complex
where
    Float: From<T>,
{
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self {
            real: self.real - rhs,
            imag: self.imag,
        }
    }
}
impl<T> SubAssign<T> for Complex
where
    Float: From<T>,
{
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs;
    }
}
impl<T> Rem<T> for Complex
where
    Float: From<T>,
{
    type Output = Self;
    fn rem(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self {
            real: self.real % rhs,
            imag: self.imag % rhs,
        }
    }
}
impl<T> RemAssign<T> for Complex
where
    Float: From<T>,
{
    fn rem_assign(&mut self, rhs: T) {
        *self = *self % rhs;
    }
}
