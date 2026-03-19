use std::cmp::Ordering;
use std::fmt::Display;
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
    fn to_real(self) -> F;
    fn real(&self) -> &F;
    fn real_mut(&mut self) -> &mut F;
    fn is_zero(&self) -> bool;
    fn parse_radix(src: &str, base: u8) -> Option<Self>;
    fn to_string_radix(&self, base: u8) -> String;
    fn get_closest_fraction(&self) -> impl Display;
    fn total_cmp(&self, other: &Self) -> Ordering;
}
pub trait FloatFunctions<F>: FloatFunctionsMut<F> + FloatTrait<F> {
    fn sin(mut self) -> Self {
        self.sin_mut();
        self
    }
    fn cos(mut self) -> Self {
        self.cos_mut();
        self
    }
    fn asin(mut self) -> Self {
        self.asin_mut();
        self
    }
    fn acos(mut self) -> Self {
        self.acos_mut();
        self
    }
    fn sinh(mut self) -> Self {
        self.sinh_mut();
        self
    }
    fn cosh(mut self) -> Self {
        self.cosh_mut();
        self
    }
    fn asinh(mut self) -> Self {
        self.asinh_mut();
        self
    }
    fn acosh(mut self) -> Self {
        self.acosh_mut();
        self
    }
    fn tan(mut self) -> Self {
        self.tan_mut();
        self
    }
    fn tanh(mut self) -> Self {
        self.tanh_mut();
        self
    }
    fn atan(mut self) -> Self {
        self.atan_mut();
        self
    }
    fn atanh(mut self) -> Self {
        self.atanh_mut();
        self
    }
    fn ln(mut self) -> Self {
        self.ln_mut();
        self
    }
    fn exp(mut self) -> Self {
        self.exp_mut();
        self
    }
    fn atan2(mut self, other: &Self) -> Self {
        self.atan2_mut(other);
        self
    }
    fn min(mut self, other: &Self) -> Self {
        self.min_mut(other);
        self
    }
    fn max(mut self, other: &Self) -> Self {
        self.max_mut(other);
        self
    }
    fn recip(mut self) -> Self {
        self.recip_mut();
        self
    }
    fn sqrt(mut self) -> Self {
        self.sqrt_mut();
        self
    }
    fn cbrt(mut self) -> Self {
        self.cbrt_mut();
        self
    }
    fn abs(mut self) -> F {
        self.abs_mut();
        self.to_real()
    }
    fn gamma(mut self) -> Self {
        self.gamma_mut();
        self
    }
    fn erf(mut self) -> Self {
        self.erf_mut();
        self
    }
    fn erfc(mut self) -> Self {
        self.erfc_mut();
        self
    }
    fn round(mut self) -> Self {
        self.round_mut();
        self
    }
    fn ceil(mut self) -> Self {
        self.ceil_mut();
        self
    }
    fn floor(mut self) -> Self {
        self.floor_mut();
        self
    }
    fn trunc(mut self) -> Self {
        self.trunc_mut();
        self
    }
    fn fract(mut self) -> Self {
        self.fract_mut();
        self
    }
    fn tetration(mut self, other: &Self) -> Self {
        self.tetration_mut(other);
        self
    }
    fn subfactorial(mut self) -> Self {
        self.subfactorial_mut();
        self
    }
    fn hypot(mut self, other: &Self) -> Self {
        self.hypot_mut(other);
        self
    }
}
pub trait FloatFunctionsMut<F>: Sized {
    fn sin_mut(&mut self);
    fn cos_mut(&mut self);
    fn asin_mut(&mut self);
    fn acos_mut(&mut self);
    fn sinh_mut(&mut self);
    fn cosh_mut(&mut self);
    fn asinh_mut(&mut self);
    fn acosh_mut(&mut self);
    fn tan_mut(&mut self);
    fn tanh_mut(&mut self);
    fn atan_mut(&mut self);
    fn atanh_mut(&mut self);
    fn ln_mut(&mut self);
    fn exp_mut(&mut self);
    fn hypot_mut(&mut self, other: &Self);
    fn atan2_mut(&mut self, other: &Self);
    fn min_mut(&mut self, other: &Self);
    fn max_mut(&mut self, other: &Self);
    fn recip_mut(&mut self);
    fn sqrt_mut(&mut self);
    fn cbrt_mut(&mut self);
    fn abs_mut(&mut self);
    fn gamma_mut(&mut self);
    fn erf_mut(&mut self);
    fn erfc_mut(&mut self);
    fn round_mut(&mut self);
    fn ceil_mut(&mut self);
    fn floor_mut(&mut self);
    fn trunc_mut(&mut self);
    fn fract_mut(&mut self);
    fn tetration_mut(&mut self, other: &Self);
    fn subfactorial_mut(&mut self);
}
pub trait ComplexFunctions<F>: ComplexFunctionsMut<F> + FloatTrait<F> {
    fn arg(mut self) -> F {
        self.arg_mut();
        self.to_real()
    }
    fn mul_i(mut self, negative: bool) -> Self {
        self.mul_i_mut(negative);
        self
    }
    fn conj(mut self) -> Self {
        self.conj_mut();
        self
    }
    fn norm(mut self) -> F {
        self.norm_mut();
        self.to_real()
    }
}
pub trait ComplexFunctionsMut<F>: Sized {
    fn arg_mut(&mut self);
    fn mul_i_mut(&mut self, negative: bool);
    fn conj_mut(&mut self);
    fn norm_mut(&mut self);
}
pub trait ComplexTrait<F>: FloatTrait<F> {
    fn to_imag(self) -> F;
    fn imag(&self) -> &F;
    fn imag_mut(&mut self) -> &mut F;
    fn zero_real(&mut self);
    fn zero_imag(&mut self);
}
pub trait RealTrait<F>: FloatTrait<F> {
    fn is_sign_negative(&self) -> bool;
    fn is_sign_positive(&self) -> bool;
    fn into_isize(self) -> isize;
    fn into_usize(self) -> usize;
    fn closest_fraction(&self) -> Option<(bool, usize, usize)>;
}
impl<T: FloatFunctionsMut<F> + FloatTrait<F>, F> FloatFunctions<F> for T {}
impl<T: ComplexFunctionsMut<F> + FloatTrait<F>, F> ComplexFunctions<F> for T {}
