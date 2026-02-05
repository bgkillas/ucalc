use crate::float;
#[cfg(feature = "float")]
pub type Integer = float::Integer;
#[cfg(feature = "float")]
pub type Float = float::Float;
#[cfg(feature = "float")]
pub type Complex = float::Complex;
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
