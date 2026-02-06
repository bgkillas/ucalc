use crate::{HalfUsize, Matrix, Number, Units, Vector};
use std::ops::{Deref, DerefMut, Index, IndexMut, Range};
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
impl<T> From<T> for Number<T> {
    fn from(value: T) -> Self {
        Self::Value(value)
    }
}
#[allow(irrefutable_let_patterns)]
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
