#![feature(f16)]
#![feature(f128)]
#![feature(float_gamma)]
#![feature(float_erf)]
use std::ops::{Deref, DerefMut, Index, IndexMut, Range};
#[cfg(feature = "float")]
mod float;
#[cfg(feature = "float")]
#[cfg(test)]
mod float_test;
#[cfg(feature = "rug")]
mod rug;
#[cfg(feature = "float")]
pub type Integer = float::Integer;
#[cfg(feature = "float")]
pub type Float = float::Float;
#[cfg(feature = "float")]
pub type Complex = float::Complex;
pub trait Pow<Rhs> {
    fn pow(self, rhs: Rhs) -> Self;
}
pub enum Constant {
    Pi,
    Tau,
    E,
    Infinity,
    NegInfinity,
    Nan,
}
pub struct Rational<T> {
    pub quotient: T,
    pub divisor: T,
}
pub struct Units<T, const N: usize>(pub [T; N]);
pub struct Quantity<T, K, const N: usize> {
    pub num: T,
    pub units: Units<K, N>,
}
#[cfg(not(feature = "units"))]
pub enum Number<T> {
    Complex(T),
    #[cfg(feature = "vector")]
    Vector(Vector<T>),
    #[cfg(feature = "matrix")]
    Matrix(Matrix<T>),
    #[cfg(feature = "list")]
    List(Vec<Number<T>>),
}
#[cfg(feature = "units")]
pub enum Number<T, K, const N: usize> {
    Complex(Quantity<T, K, N>),
    #[cfg(feature = "vector")]
    Vector(Quantity<Vector<T>, K, N>),
    #[cfg(feature = "matrix")]
    Matrix(Quantity<Matrix<T>, K, N>),
    #[cfg(feature = "list")]
    List(Vec<Number<T, K, N>>),
}
pub struct Vector<T>(pub Vec<T>);
pub struct Matrix<T> {
    pub vec: Vector<T>,
    pub width: HalfUsize,
    pub height: HalfUsize,
}
#[cfg(target_pointer_width = "64")]
pub type HalfUsize = u32;
#[cfg(target_pointer_width = "32")]
pub type HalfUsize = u16;
#[cfg(target_pointer_width = "16")]
pub type HalfUsize = u8;
impl<T> Deref for Vector<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T, const N: usize> Deref for Units<T, N> {
    type Target = [T; N];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T, const N: usize> DerefMut for Units<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<T> Index<HalfUsize> for Matrix<T> {
    type Output = [T];
    fn index(&self, index: HalfUsize) -> &Self::Output {
        let start = index * self.height;
        &self.vec[start..start + self.width]
    }
}
impl<T> IndexMut<HalfUsize> for Matrix<T> {
    fn index_mut(&mut self, index: HalfUsize) -> &mut Self::Output {
        let start = index * self.height;
        &mut self.vec[start..start + self.width]
    }
}
impl<T> Index<HalfUsize> for Vector<T> {
    type Output = T;
    fn index(&self, index: HalfUsize) -> &Self::Output {
        &self.0[index as usize]
    }
}
impl<T> IndexMut<HalfUsize> for Vector<T> {
    fn index_mut(&mut self, index: HalfUsize) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}
impl<T> Index<Range<HalfUsize>> for Vector<T> {
    type Output = [T];
    fn index(&self, index: Range<HalfUsize>) -> &Self::Output {
        &self.0[index.start as usize..index.end as usize]
    }
}
impl<T> IndexMut<Range<HalfUsize>> for Vector<T> {
    fn index_mut(&mut self, index: Range<HalfUsize>) -> &mut Self::Output {
        &mut self.0[index.start as usize..index.end as usize]
    }
}
