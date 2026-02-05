use std::mem;
pub trait Pow<Rhs> {
    type Output;
    fn pow(self, rhs: Rhs) -> Self::Output;
}
pub trait PowAssign<Rhs> {
    fn pow_assign(&mut self, rhs: Rhs);
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
