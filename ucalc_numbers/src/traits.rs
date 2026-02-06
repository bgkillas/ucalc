use std::cmp::Ordering;
use std::mem;
use std::ops::Neg;
pub trait Pow<Rhs> {
    type Output;
    fn pow(self, rhs: Rhs) -> Self::Output;
}
pub trait PowAssign<Rhs> {
    fn pow_assign(&mut self, rhs: Rhs);
}
pub trait NegAssign {
    fn neg_assign(&mut self);
}
impl<T, K> PowAssign<K> for T
where
    T: Pow<K, Output = T> + Default,
{
    default fn pow_assign(&mut self, rhs: K) {
        let old = mem::take(self);
        *self = old.pow(rhs)
    }
}
impl<T> NegAssign for T
where
    T: Neg<Output = T> + Default,
{
    default fn neg_assign(&mut self) {
        let old = mem::take(self);
        *self = old.neg();
    }
}
#[rustc_specialization_trait]
pub trait Primative: Copy {}
#[rustc_specialization_trait]
pub trait PrimativeFloat: Primative {}
#[rustc_specialization_trait]
pub trait PrimativeInteger: Primative + TryInto<i32> {}
macro_rules! primative {
    ($($ty:ty),*) => {
        $(
            impl Primative for $ty {}
        )*
    }
}
primative!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f16, f32, f64, f128
);
macro_rules! primative_integer {
    ($($ty:ty),*) => {
        $(
            impl PrimativeInteger for $ty {}
        )*
    }
}
primative_integer!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);
macro_rules! primative_float {
    ($($ty:ty),*) => {
        $(
            impl PrimativeFloat for $ty {}
        )*
    }
}
primative_float!(f16, f32, f64, f128);
pub trait FloatTrait<F>: Sized {
    fn real(&self) -> &F;
    fn real_mut(&mut self) -> &mut F;
    fn sin_cos(self) -> (Self, Self);
    fn is_zero(&self) -> bool;
    fn sin_mut(&mut self);
    fn sin(self) -> Self;
    fn cos_mut(&mut self);
    fn cos(self) -> Self;
    fn asin_mut(&mut self);
    fn asin(self) -> Self;
    fn acos_mut(&mut self);
    fn acos(self) -> Self;
    fn sinh_mut(&mut self);
    fn sinh(self) -> Self;
    fn cosh_mut(&mut self);
    fn cosh(self) -> Self;
    fn asinh_mut(&mut self);
    fn asinh(self) -> Self;
    fn acosh_mut(&mut self);
    fn acosh(self) -> Self;
    fn tan_mut(&mut self);
    fn tan(self) -> Self;
    fn tanh_mut(&mut self);
    fn tanh(self) -> Self;
    fn atan_mut(&mut self);
    fn atan(self) -> Self;
    fn atanh_mut(&mut self);
    fn atanh(self) -> Self;
    fn ln_mut(&mut self);
    fn ln(self) -> Self;
    fn exp_mut(&mut self);
    fn exp(self) -> Self;
    fn atan2_mut(&mut self, other: &Self);
    fn atan2(self, other: &Self) -> Self;
    fn min_mut(&mut self, other: &Self);
    fn min(self, other: &Self) -> Self;
    fn max_mut(&mut self, other: &Self);
    fn max(self, other: &Self) -> Self;
    fn recip_mut(&mut self);
    fn recip(self) -> Self;
    fn sqrt_mut(&mut self);
    fn sqrt(self) -> Self;
    fn abs_mut(&mut self);
    fn abs(self) -> F;
    fn gamma_mut(&mut self);
    fn gamma(self) -> Self;
    fn erf_mut(&mut self);
    fn erf(self) -> Self;
    fn erfc_mut(&mut self);
    fn erfc(self) -> Self;
    fn total_cmp(&self, other: &Self) -> Ordering;
    fn round_mut(&mut self);
    fn round(self) -> Self;
    fn ceil_mut(&mut self);
    fn ceil(self) -> Self;
    fn floor_mut(&mut self);
    fn floor(self) -> Self;
    fn trunc_mut(&mut self);
    fn trunc(self) -> Self;
    fn fract_mut(&mut self);
    fn fract(self) -> Self;
    fn tetration_mut(&mut self, other: &Self);
    fn tetration(self, other: &Self) -> Self;
    fn subfactorial_mut(&mut self);
    fn subfactorial(self) -> Self;
    fn parse_radix(src: &str, _: i32) -> Option<Self>;
}
pub trait ComplexTrait<F>: FloatTrait<F> {
    fn imag(&self) -> &F;
    fn imag_mut(&mut self) -> &mut F;
    fn zero_real(&mut self);
    fn zero_imag(&mut self);
    fn norm(self) -> F;
    fn arg_mut(&mut self);
    fn arg(self) -> F;
    fn mul_i_mut(&mut self, negative: bool);
    fn mul_i(self, negative: bool) -> Self;
    fn conj_mut(&mut self);
    fn conj(self) -> Self;
}
pub trait RealTrait<F>: FloatTrait<F> {
    fn is_sign_negative(&self) -> bool;
    fn is_sign_positive(&self) -> bool;
    fn hypot_mut(&mut self, other: &Self);
    fn into_isize(self) -> isize;
}
