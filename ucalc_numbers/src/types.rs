use crate::{float, integer};
#[cfg(feature = "float")]
pub type Integer = integer::Integer;
#[cfg(feature = "float")]
pub type UInteger = integer::UInteger;
#[cfg(feature = "float")]
pub type Float = float::Float;
#[cfg(feature = "float")]
pub type Complex = float::Complex;
#[derive(Copy, Clone)]
pub enum Constant {
    Pi,
    Tau,
    E,
    Infinity,
    NegInfinity,
    Nan,
}
#[derive(Debug, PartialEq, Clone)]
#[repr(transparent)]
pub struct Units<T, const N: usize>(pub Option<Box<[T; N]>>);
#[derive(Debug, PartialEq, Clone)]
pub struct Quantity<T, K, const N: usize> {
    pub num: T,
    pub units: Units<K, N>,
}
#[cfg(feature = "units")]
#[derive(Debug, PartialEq, Clone)]
pub enum Number<T, K, const N: usize> {
    Value(Quantity<T, K, N>),
    #[cfg(feature = "vector")]
    Vector(Vector<Quantity<T, K, N>>),
    #[cfg(feature = "matrix")]
    Matrix(Matrix<Quantity<T, K, N>>),
    #[cfg(feature = "list")]
    List(Vec<Number<T, K, N>>),
    Units(Units<K, N>),
}
#[cfg(not(feature = "units"))]
#[derive(Debug, PartialEq, Clone)]
pub enum Number<T> {
    Value(T),
    #[cfg(feature = "vector")]
    Vector(Vector<T>),
    #[cfg(feature = "matrix")]
    Matrix(Matrix<T>),
    #[cfg(feature = "list")]
    List(Vec<Number<T>>),
}
#[derive(Debug, PartialEq, Clone)]
#[repr(transparent)]
pub struct Vector<T>(pub(crate) Vec<T>);
#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T> {
    pub(crate) vec: *mut T,
    pub(crate) capacity_width: HalfUsize,
    pub(crate) capacity_height: HalfUsize,
    pub(crate) width: HalfUsize,
    pub(crate) height: HalfUsize,
}
#[cfg(target_pointer_width = "64")]
pub type HalfUsize = u32;
#[cfg(target_pointer_width = "32")]
pub type HalfUsize = u16;
#[cfg(target_pointer_width = "16")]
pub type HalfUsize = u8;
