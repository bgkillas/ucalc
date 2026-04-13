#[cfg(feature = "units")]
use crate::Quantity;
use crate::{Complex, Float, HalfUsize, Matrix, NegAssign, Number, Vector};
use crate::{Pow, PowAssign};
use std::fmt::Display;
use std::fmt::Formatter;
use std::iter::{Product, Sum};
use std::ops::Neg;
use std::ops::{
    Add, AddAssign, Deref, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Range, Rem, RemAssign,
    Sub, SubAssign,
};
use ucalc_numbers_macros::{generate_lower, generate_types};
impl<T> Deref for Vector<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> Index<HalfUsize> for Matrix<T> {
    type Output = [T];
    fn index(&self, index: HalfUsize) -> &Self::Output {
        let start = index as usize * self.height as usize;
        unsafe {
            self.vec
                .add(start)
                .cast_slice(self.width as usize)
                .as_ref()
                .unwrap()
        }
    }
}
impl<T> IndexMut<HalfUsize> for Matrix<T> {
    fn index_mut(&mut self, index: HalfUsize) -> &mut Self::Output {
        let start = index as usize * self.height as usize;
        unsafe {
            self.vec
                .add(start)
                .cast_slice(self.width as usize)
                .as_mut()
                .unwrap()
        }
    }
}
impl<T> Index<(HalfUsize, HalfUsize)> for Matrix<T> {
    type Output = T;
    fn index(&self, (i, j): (HalfUsize, HalfUsize)) -> &Self::Output {
        let start = i as usize * self.height as usize;
        unsafe { self.vec.add(start).add(j as usize).as_ref().unwrap() }
    }
}
impl<T> IndexMut<(HalfUsize, HalfUsize)> for Matrix<T> {
    fn index_mut(&mut self, (i, j): (HalfUsize, HalfUsize)) -> &mut Self::Output {
        let start = i as usize * self.height as usize;
        unsafe { self.vec.add(start).add(j as usize).as_mut().unwrap() }
    }
}
impl<T> Index<usize> for Vector<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl<T> IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
impl<T> Index<Range<usize>> for Vector<T> {
    type Output = [T];
    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.0[index.start..index.end]
    }
}
impl<T> IndexMut<Range<usize>> for Vector<T> {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        &mut self.0[index.start..index.end]
    }
}
#[allow(irrefutable_let_patterns)]
#[cfg(not(feature = "units"))]
impl<T> Number<T> {
    pub fn value_mut(&mut self) -> &mut T {
        let Self::Value(val) = self else {
            unreachable!()
        };
        val
    }
    pub fn value_ref(&self) -> &T {
        let Self::Value(val) = self else {
            unreachable!()
        };
        val
    }
    #[cfg(feature = "list")]
    pub fn list_mut(&mut self) -> &mut Vec<Number<T>> {
        let Self::List(list) = self else {
            unreachable!()
        };
        list
    }
    #[cfg(feature = "list")]
    pub fn list_ref(&self) -> &[Number<T>] {
        let Self::List(list) = self else {
            unreachable!()
        };
        list
    }
    pub fn get_value(&self) -> Option<&T> {
        let Self::Value(val) = self else { return None };
        Some(val)
    }
}
#[allow(irrefutable_let_patterns)]
#[cfg(feature = "units")]
impl<T, K, const N: usize> Number<T, K, N> {
    pub fn value_mut(&mut self) -> &mut Quantity<T, K, N> {
        let Self::Value(val) = self else {
            unreachable!()
        };
        val
    }
    pub fn value_ref(&self) -> &Quantity<T, K, N> {
        let Self::Value(val) = self else {
            unreachable!()
        };
        val
    }
    #[cfg(feature = "list")]
    pub fn list_mut(&mut self) -> &mut Vec<Number<T, K, N>> {
        let Self::List(list) = self else {
            unreachable!()
        };
        list
    }
    #[cfg(feature = "list")]
    pub fn list_ref(&self) -> &[Number<T, K, N>] {
        let Self::List(list) = self else {
            unreachable!()
        };
        list
    }
    pub fn get_value(&self) -> Option<&Quantity<T, K, N>> {
        let Self::Value(val) = self else { return None };
        Some(val)
    }
}
generate_lower!(Complex, Float);
generate_types!(Complex);
generate_types!(Float);
