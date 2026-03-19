use crate::{
    Complex, ComplexFunctionsMut, ComplexTrait, Float, FloatFunctionsMut, FloatTrait, NegAssign,
    Pow, PowAssign, Quantity, Units,
};
use std::array;
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
pub trait One {
    fn one() -> Self;
}
impl<T: Default + One, const N: usize> Units<T, N> {
    pub fn from(set: [&'static str; N], str: &str) -> Self {
        if let Some(n) = set.into_iter().position(|s| str == s) {
            let s = array::from_fn(|i| if i == n { T::one() } else { T::default() });
            Self(Some(Box::new(s)))
        } else {
            Self(None)
        }
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
impl<F, T: FloatTrait<F> + FloatFunctionsMut<F>, N, const K: usize> FloatFunctionsMut<F>
    for Quantity<T, N, K>
{
    fn sin_mut(&mut self) {
        self.num.sin_mut()
    }
    fn cos_mut(&mut self) {
        self.num.cos_mut()
    }
    fn asin_mut(&mut self) {
        self.num.asin_mut()
    }
    fn acos_mut(&mut self) {
        self.num.acos_mut()
    }
    fn sinh_mut(&mut self) {
        self.num.sinh_mut()
    }
    fn cosh_mut(&mut self) {
        self.num.cosh_mut()
    }
    fn asinh_mut(&mut self) {
        self.num.asinh_mut()
    }
    fn acosh_mut(&mut self) {
        self.num.acosh_mut()
    }
    fn tan_mut(&mut self) {
        self.num.tan_mut()
    }
    fn tanh_mut(&mut self) {
        self.num.tanh_mut()
    }
    fn atan_mut(&mut self) {
        self.num.atan_mut()
    }
    fn atanh_mut(&mut self) {
        self.num.atanh_mut()
    }
    fn ln_mut(&mut self) {
        self.num.ln_mut()
    }
    fn exp_mut(&mut self) {
        self.num.exp_mut()
    }
    fn hypot_mut(&mut self, other: &Self) {
        self.num.hypot_mut(&other.num)
    }
    fn atan2_mut(&mut self, other: &Self) {
        self.num.atan2_mut(&other.num)
    }
    fn min_mut(&mut self, other: &Self) {
        self.num.min_mut(&other.num)
    }
    fn max_mut(&mut self, other: &Self) {
        self.num.max_mut(&other.num)
    }
    fn recip_mut(&mut self) {
        self.num.recip_mut()
    }
    fn sqrt_mut(&mut self) {
        self.num.sqrt_mut()
    }
    fn cbrt_mut(&mut self) {
        self.num.cbrt_mut()
    }
    fn abs_mut(&mut self) {
        self.num.abs_mut()
    }
    fn gamma_mut(&mut self) {
        self.num.gamma_mut()
    }
    fn erf_mut(&mut self) {
        self.num.erf_mut()
    }
    fn erfc_mut(&mut self) {
        self.num.erfc_mut()
    }
    fn round_mut(&mut self) {
        self.num.round_mut()
    }
    fn ceil_mut(&mut self) {
        self.num.ceil_mut()
    }
    fn floor_mut(&mut self) {
        self.num.floor_mut()
    }
    fn trunc_mut(&mut self) {
        self.num.trunc_mut()
    }
    fn fract_mut(&mut self) {
        self.num.fract_mut()
    }
    fn tetration_mut(&mut self, other: &Self) {
        self.num.tetration_mut(&other.num)
    }
    fn subfactorial_mut(&mut self) {
        self.num.subfactorial_mut()
    }
}
impl<F, T: FloatTrait<F>, N, const K: usize> FloatTrait<F> for Quantity<T, N, K> {
    fn to_real(self) -> F {
        self.num.to_real()
    }
    fn real(&self) -> &F {
        self.num.real()
    }
    fn real_mut(&mut self) -> &mut F {
        self.num.real_mut()
    }
    fn is_zero(&self) -> bool {
        self.num.is_zero()
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
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.num.total_cmp(&other.num)
    }
}
impl<F, T: ComplexTrait<F> + ComplexFunctionsMut<F>, N, const K: usize> ComplexFunctionsMut<F>
    for Quantity<T, N, K>
{
    fn arg_mut(&mut self) {
        self.num.arg_mut();
    }
    fn mul_i_mut(&mut self, negative: bool) {
        self.num.mul_i_mut(negative);
    }
    fn conj_mut(&mut self) {
        self.num.conj_mut()
    }
    fn norm_mut(&mut self) {
        self.num.norm_mut()
    }
}
impl<F, T: ComplexTrait<F>, N, const K: usize> ComplexTrait<F> for Quantity<T, N, K> {
    fn to_imag(self) -> F {
        self.num.to_imag()
    }
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
}
