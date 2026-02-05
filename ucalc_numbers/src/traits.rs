use crate::{Complex, Float, Integer};
use std::mem;
use std::ops::{Add, AddAssign};
pub trait Pow<Rhs, Target> {
    fn pow(self, rhs: Rhs) -> Target;
}
pub trait PowAssign<Rhs> {
    fn pow_assign(&mut self, rhs: Rhs);
}
impl<T, K> PowAssign<K> for T
where
    T: Pow<K, T> + Default,
{
    default fn pow_assign(&mut self, rhs: K) {
        let old = mem::take(self);
        *self = old.pow(rhs)
    }
}
pub trait Primative: Copy {}
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
macro_rules! sealed {
    ($($ty:ty),*) => {
        $(
impl<K> AddAssign<K> for $ty
where
    $ty: Add<K, Output = $ty> + Default,
{
    default fn add_assign(&mut self, rhs: K) {
        let old = mem::take(self);
        *self = old.add(rhs);
    }
}
        )*
    }
}
sealed!(Float, Complex, Integer);
