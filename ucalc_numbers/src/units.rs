use crate::{Complex, ComplexTrait, Float, FloatTrait, NegAssign, Pow, PowAssign, Quantity, Units};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::iter::{Product, Sum};
use std::ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub,
    SubAssign,
};
impl<N, const K: usize> Default for Units<N, K> {
    fn default() -> Self {
        Units(None)
    }
}
impl<T, const N: usize> Deref for Units<T, N> {
    type Target = Option<Box<[T; N]>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T, const N: usize> DerefMut for Units<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<T: Default, N, const K: usize> Default for Quantity<T, N, K> {
    fn default() -> Self {
        Self {
            num: T::default(),
            units: Units::default(),
        }
    }
}
impl<T, N, const K: usize> From<T> for Quantity<Complex, N, K>
where
    Complex: From<T>,
{
    fn from(num: T) -> Self {
        Self {
            num: num.into(),
            units: Units::default(),
        }
    }
}
impl<T, N, const K: usize> From<T> for Quantity<Float, N, K>
where
    Float: From<T>,
{
    fn from(num: T) -> Self {
        Self {
            num: num.into(),
            units: Units::default(),
        }
    }
}
impl<T: AddAssign + Default, N, const K: usize> Sum for Quantity<T, N, K> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |sum, s| sum + s)
    }
}
impl<N, const K: usize> Product for Quantity<Float, N, K> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::from(1), |sum, s| sum * s)
    }
}
impl<N, const K: usize> Product for Quantity<Complex, N, K> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::from(1), |sum, s| sum * s)
    }
}
impl<T: NegAssign, N, const K: usize> Neg for Quantity<T, N, K> {
    type Output = Self;
    fn neg(mut self) -> Self {
        self.num.neg_assign();
        self
    }
}
impl<T: Display, N, const K: usize> Display for Quantity<T, N, K> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.num)
    }
}
macro_rules! impl_ops {
    ($op:ident,$op_assign:ident,$fun:ident,$fun_assign:ident) => {
        impl<T: $op_assign<T>, N, const K: usize> $op<Self> for Quantity<T, N, K> {
            type Output = Self;
            fn $fun(mut self, rhs: Self) -> Self::Output {
                $op_assign::$fun_assign(&mut self, rhs);
                self
            }
        }
        impl<T: $op_assign<T>, N, const K: usize> $op_assign<Self> for Quantity<T, N, K> {
            fn $fun_assign(&mut self, rhs: Self) {
                $op_assign::$fun_assign(&mut self.num, rhs.num);
            }
        }
        impl<T: for<'a> $op_assign<&'a T>, N, const K: usize> $op<&Self> for Quantity<T, N, K> {
            type Output = Self;
            fn $fun(mut self, rhs: &Self) -> Self::Output {
                $op_assign::$fun_assign(&mut self, rhs);
                self
            }
        }
        impl<T: for<'a> $op_assign<&'a T>, N, const K: usize> $op_assign<&Self>
            for Quantity<T, N, K>
        {
            fn $fun_assign(&mut self, rhs: &Self) {
                $op_assign::$fun_assign(&mut self.num, &rhs.num);
            }
        }
        impl<T: $op_assign<Float>, N, const K: usize> $op<Float> for Quantity<T, N, K> {
            type Output = Self;
            fn $fun(mut self, rhs: Float) -> Self::Output {
                $op_assign::$fun_assign(&mut self, rhs);
                self
            }
        }
        impl<T: $op_assign<Float>, N, const K: usize> $op_assign<Float> for Quantity<T, N, K> {
            fn $fun_assign(&mut self, rhs: Float) {
                $op_assign::$fun_assign(&mut self.num, rhs);
            }
        }
        impl<T: for<'a> $op_assign<&'a Float>, N, const K: usize> $op<&Float>
            for Quantity<T, N, K>
        {
            type Output = Self;
            fn $fun(mut self, rhs: &Float) -> Self::Output {
                $op_assign::$fun_assign(&mut self, rhs);
                self
            }
        }
        impl<T: for<'a> $op_assign<&'a Float>, N, const K: usize> $op_assign<&Float>
            for Quantity<T, N, K>
        {
            fn $fun_assign(&mut self, rhs: &Float) {
                $op_assign::$fun_assign(&mut self.num, &rhs);
            }
        }
        impl<N, const K: usize> $op<Quantity<Float, N, K>> for Float {
            type Output = Quantity<Float, N, K>;
            fn $fun(self, mut rhs: Quantity<Float, N, K>) -> Self::Output {
                rhs.num = $op::$fun(self, rhs.num);
                rhs
            }
        }
        impl<N, const K: usize> $op<Quantity<Complex, N, K>> for Float {
            type Output = Quantity<Complex, N, K>;
            fn $fun(self, mut rhs: Quantity<Complex, N, K>) -> Self::Output {
                rhs.num = $op::$fun(self, rhs.num);
                rhs
            }
        }
    };
}
impl_ops!(Add, AddAssign, add, add_assign);
impl_ops!(Sub, SubAssign, sub, sub_assign);
impl_ops!(Mul, MulAssign, mul, mul_assign);
impl_ops!(Div, DivAssign, div, div_assign);
impl_ops!(Rem, RemAssign, rem, rem_assign);
impl<T: PowAssign<T>, N, const K: usize> Pow<Self> for Quantity<T, N, K> {
    type Output = Self;
    fn pow(mut self, rhs: Self) -> Self::Output {
        self.num.pow_assign(rhs.num);
        self
    }
}
impl<T: for<'a> PowAssign<&'a T>, N, const K: usize> Pow<&Self> for Quantity<T, N, K> {
    type Output = Self;
    fn pow(mut self, rhs: &Self) -> Self::Output {
        self.num.pow_assign(&rhs.num);
        self
    }
}
impl<T: PowAssign<Float>, N, const K: usize> Pow<Float> for Quantity<T, N, K> {
    type Output = Self;
    fn pow(mut self, rhs: Float) -> Self::Output {
        self.num.pow_assign(rhs);
        self
    }
}
impl<T: for<'a> PowAssign<&'a Float>, N, const K: usize> Pow<&Float> for Quantity<T, N, K> {
    type Output = Self;
    fn pow(mut self, rhs: &Float) -> Self::Output {
        self.num.pow_assign(rhs);
        self
    }
}
impl<T: PartialEq, const N: usize> Add<Units<T, N>> for Units<T, N> {
    type Output = Self;
    fn add(self, rhs: Units<T, N>) -> Self::Output {
        if self == rhs { self } else { Self::default() }
    }
}
impl<T: PartialEq, const N: usize> Sub<Units<T, N>> for Units<T, N> {
    type Output = Self;
    fn sub(self, rhs: Units<T, N>) -> Self::Output {
        if self == rhs { self } else { Self::default() }
    }
}
impl<T: AddAssign<T>, const N: usize> Mul<Units<T, N>> for Units<T, N> {
    type Output = Self;
    fn mul(self, rhs: Units<T, N>) -> Self::Output {
        Self(match (self.0, rhs.0) {
            (Some(mut a), Some(b)) => {
                a.iter_mut().zip(*b).for_each(|(a, b)| *a += b);
                Some(a)
            }
            (a, None) | (None, a) => a,
        })
    }
}
impl<T: SubAssign<T> + NegAssign, const N: usize> Div<Units<T, N>> for Units<T, N> {
    type Output = Self;
    fn div(self, rhs: Units<T, N>) -> Self::Output {
        Self(match (self.0, rhs.0) {
            (Some(mut a), Some(b)) => {
                a.iter_mut().zip(*b).for_each(|(a, b)| *a -= b);
                Some(a)
            }
            (None, Some(mut b)) => {
                b.iter_mut().for_each(|b| b.neg_assign());
                Some(b)
            }
            (a, None) => a,
        })
    }
}
impl<T: for<'a> MulAssign<&'a T> + TryFrom<K>, K, const N: usize> Pow<Quantity<K, T, N>>
    for Units<T, N>
{
    type Output = Self;
    fn pow(self, rhs: Quantity<K, T, N>) -> Self::Output {
        Self(match (self.0, rhs.units.0) {
            (Some(mut a), None) if let Ok(b) = rhs.num.try_into() => {
                a.iter_mut().for_each(|a| *a *= &b);
                Some(a)
            }
            _ => None,
        })
    }
}
impl<F, T: FloatTrait<F>, N, const K: usize> FloatTrait<F> for Quantity<T, N, K> {
    fn real(&self) -> &F {
        self.num.real()
    }
    fn real_mut(&mut self) -> &mut F {
        self.num.real_mut()
    }
    fn is_zero(&self) -> bool {
        self.num.is_zero()
    }
    fn sin_mut(&mut self) {
        self.num.sin_mut()
    }
    fn sin(mut self) -> Self {
        self.num.sin_mut();
        self
    }
    fn cos_mut(&mut self) {
        self.num.cos_mut()
    }
    fn cos(mut self) -> Self {
        self.num.cos_mut();
        self
    }
    fn asin_mut(&mut self) {
        self.num.asin_mut()
    }
    fn asin(mut self) -> Self {
        self.num.asin_mut();
        self
    }
    fn acos_mut(&mut self) {
        self.num.acos_mut()
    }
    fn acos(mut self) -> Self {
        self.num.acos_mut();
        self
    }
    fn sinh_mut(&mut self) {
        self.num.sinh_mut()
    }
    fn sinh(mut self) -> Self {
        self.num.sinh_mut();
        self
    }
    fn cosh_mut(&mut self) {
        self.num.cosh_mut()
    }
    fn cosh(mut self) -> Self {
        self.num.cosh_mut();
        self
    }
    fn asinh_mut(&mut self) {
        self.num.asinh_mut()
    }
    fn asinh(mut self) -> Self {
        self.num.asinh_mut();
        self
    }
    fn acosh_mut(&mut self) {
        self.num.acosh_mut()
    }
    fn acosh(mut self) -> Self {
        self.num.acosh_mut();
        self
    }
    fn tan_mut(&mut self) {
        self.num.tan_mut()
    }
    fn tan(mut self) -> Self {
        self.num.tan_mut();
        self
    }
    fn tanh_mut(&mut self) {
        self.num.tanh_mut()
    }
    fn tanh(mut self) -> Self {
        self.num.tanh_mut();
        self
    }
    fn atan_mut(&mut self) {
        self.num.atan_mut()
    }
    fn atan(mut self) -> Self {
        self.num.atan_mut();
        self
    }
    fn atanh_mut(&mut self) {
        self.num.atanh_mut()
    }
    fn atanh(mut self) -> Self {
        self.num.atanh_mut();
        self
    }
    fn ln_mut(&mut self) {
        self.num.ln_mut()
    }
    fn ln(mut self) -> Self {
        self.num.ln_mut();
        self
    }
    fn exp_mut(&mut self) {
        self.num.exp_mut()
    }
    fn exp(mut self) -> Self {
        self.num.exp_mut();
        self
    }
    fn atan2_mut(&mut self, other: &Self) {
        self.num.atan2_mut(&other.num)
    }
    fn atan2(mut self, other: &Self) -> Self {
        self.num.atan2_mut(&other.num);
        self
    }
    fn min_mut(&mut self, other: &Self) {
        self.num.min_mut(&other.num)
    }
    fn min(mut self, other: &Self) -> Self {
        self.num.min_mut(&other.num);
        self
    }
    fn max_mut(&mut self, other: &Self) {
        self.num.max_mut(&other.num)
    }
    fn max(mut self, other: &Self) -> Self {
        self.num.max_mut(&other.num);
        self
    }
    fn recip_mut(&mut self) {
        self.num.recip_mut()
    }
    fn recip(mut self) -> Self {
        self.num.recip_mut();
        self
    }
    fn sqrt_mut(&mut self) {
        self.num.sqrt_mut()
    }
    fn sqrt(mut self) -> Self {
        self.num.sqrt_mut();
        self
    }
    fn cbrt_mut(&mut self) {
        self.num.cbrt_mut()
    }
    fn cbrt(mut self) -> Self {
        self.num.cbrt_mut();
        self
    }
    fn abs_mut(&mut self) {
        self.num.abs_mut()
    }
    fn abs(self) -> F {
        self.num.abs()
    }
    fn gamma_mut(&mut self) {
        self.num.gamma_mut()
    }
    fn gamma(mut self) -> Self {
        self.num.gamma_mut();
        self
    }
    fn erf_mut(&mut self) {
        self.num.erf_mut()
    }
    fn erf(mut self) -> Self {
        self.num.erf_mut();
        self
    }
    fn erfc_mut(&mut self) {
        self.num.erfc_mut()
    }
    fn erfc(mut self) -> Self {
        self.num.erfc_mut();
        self
    }
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.num.total_cmp(&other.num)
    }
    fn round_mut(&mut self) {
        self.num.round_mut()
    }
    fn round(mut self) -> Self {
        self.num.round_mut();
        self
    }
    fn ceil_mut(&mut self) {
        self.num.ceil_mut()
    }
    fn ceil(mut self) -> Self {
        self.num.ceil_mut();
        self
    }
    fn floor_mut(&mut self) {
        self.num.floor_mut()
    }
    fn floor(mut self) -> Self {
        self.num.floor_mut();
        self
    }
    fn trunc_mut(&mut self) {
        self.num.trunc_mut()
    }
    fn trunc(mut self) -> Self {
        self.num.trunc_mut();
        self
    }
    fn fract_mut(&mut self) {
        self.num.fract_mut()
    }
    fn fract(mut self) -> Self {
        self.num.fract_mut();
        self
    }
    fn tetration_mut(&mut self, other: &Self) {
        self.num.tetration_mut(&other.num)
    }
    fn tetration(mut self, other: &Self) -> Self {
        self.num.tetration_mut(&other.num);
        self
    }
    fn subfactorial_mut(&mut self) {
        self.num.subfactorial_mut()
    }
    fn subfactorial(mut self) -> Self {
        self.num.subfactorial_mut();
        self
    }
    fn parse_radix(src: &str, base: u8) -> Option<Self> {
        T::parse_radix(src, base).map(|num| Self {
            num,
            units: Units::default(),
        })
    }
    fn to_string_radix(&self, base: u8) -> String {
        self.num.to_string_radix(base)
    }
    fn get_closest_fraction(&self) -> impl Display {
        self.num.get_closest_fraction()
    }
}

impl<F, T: ComplexTrait<F>, N, const K: usize> ComplexTrait<F> for Quantity<T, N, K> {
    fn imag(&self) -> &F {
        self.num.imag()
    }
    fn imag_mut(&mut self) -> &mut F {
        self.num.imag_mut()
    }
    fn zero_real(&mut self) {
        self.num.zero_real()
    }
    fn zero_imag(&mut self) {
        self.num.zero_imag()
    }
    fn norm(self) -> F {
        self.num.norm()
    }
    fn arg_mut(&mut self) {
        self.num.arg_mut()
    }
    fn arg(self) -> F {
        self.num.arg()
    }
    fn mul_i_mut(&mut self, negative: bool) {
        self.num.mul_i_mut(negative)
    }
    fn mul_i(mut self, negative: bool) -> Self {
        self.num.mul_i_mut(negative);
        self
    }
    fn conj_mut(&mut self) {
        self.num.conj_mut()
    }
    fn conj(mut self) -> Self {
        self.num.conj_mut();
        self
    }
}
