use crate::{Complex, Float, HalfUsize, Matrix, NegAssign, Number, Units, Vector};
use crate::{Pow, PowAssign};
use std::fmt::Display;
use std::fmt::Formatter;
use std::iter::{Product, Sum};
use std::ops::Neg;
use std::ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Range, Rem,
    RemAssign, Sub, SubAssign,
};
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
macro_rules! impl_lower_ops {
    ($ty:ty, $oty:ty,$op:ident,$op_assign:ident,$fun:ident,$fun_assign:ident) => {
        impl $op<$oty> for Number<$ty> {
            type Output = Self;
            fn $fun(mut self, rhs: $oty) -> Self::Output {
                $op_assign::$fun_assign(&mut self, rhs);
                self
            }
        }
        impl $op<Number<$ty>> for $oty {
            type Output = Number<$ty>;
            fn $fun(self, rhs: Number<$ty>) -> Self::Output {
                match rhs {
                    Number::Value(b) => $op::$fun(self, b).into(),
                    #[cfg(feature = "list")]
                    Number::List(mut b) => {
                        b.iter_mut().for_each(|b| {
                            let old = std::mem::replace(b, Number::Value(<$ty>::from(0)));
                            *b = $op::$fun(self.clone(), old);
                        });
                        Number::List(b)
                    }
                }
            }
        }
        impl $op_assign<$oty> for Number<$ty> {
            fn $fun_assign(&mut self, rhs: $oty) {
                match self {
                    Self::Value(a) => $op_assign::$fun_assign(a, rhs.clone()),
                    #[cfg(feature = "list")]
                    Self::List(a) => a
                        .iter_mut()
                        .for_each(|a| $op_assign::$fun_assign(a, rhs.clone())),
                }
            }
        }
    };
}
macro_rules! impl_ops {
    ($ty:ty,$op:ident,$op_assign:ident,$fun:ident,$fun_assign:ident) => {
        impl $op<Self> for Number<$ty> {
            type Output = Self;
            fn $fun(mut self, rhs: Self) -> Self::Output {
                $op_assign::$fun_assign(&mut self, rhs);
                self
            }
        }
        impl $op_assign<Self> for Number<$ty> {
            fn $fun_assign(&mut self, rhs: Self) {
                match (self, rhs) {
                    (Self::Value(a), Self::Value(b)) => $op_assign::$fun_assign(a, b),
                    #[cfg(feature = "list")]
                    (Self::List(a), Self::Value(b)) => a
                        .iter_mut()
                        .for_each(|a| $op_assign::$fun_assign(a, b.clone())),
                    #[cfg(feature = "list")]
                    (s @ Self::Value(_), mut r @ Self::List(_)) => {
                        std::mem::swap(s, &mut r);
                        let (Self::List(a), Self::Value(b)) = (s, r) else {
                            unreachable!()
                        };
                        a.iter_mut().for_each(|a| {
                            let old = std::mem::replace(a, Number::Value(<$ty>::from(0)));
                            *a = $op::$fun(b.clone(), old)
                        })
                    }
                    #[cfg(feature = "list")]
                    (Self::List(a), Self::List(b)) => a
                        .iter_mut()
                        .zip(b.into_iter())
                        .for_each(|(a, b)| $op_assign::$fun_assign(a, b)),
                }
            }
        }
        impl_lower_ops!($ty, $ty, $op, $op_assign, $fun, $fun_assign);
    };
}
macro_rules! impl_num {
    ($ty:ty) => {
        impl<K> From<K> for Number<$ty>
        where
            $ty: From<K>,
        {
            fn from(value: K) -> Self {
                Self::Value(value.into())
            }
        }
        impl Default for Number<$ty> {
            fn default() -> Self {
                Self::Value(<$ty>::default())
            }
        }
        impl Sum for Number<$ty> {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::default(), |sum, s| sum + s)
            }
        }
        impl Product for Number<$ty> {
            fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::from(1.0), |sum, s| sum * s)
            }
        }
        impl NegAssign for Number<$ty> {
            fn neg_assign(&mut self) {
                match self {
                    Self::Value(a) => a.neg_assign(),
                    #[cfg(feature = "list")]
                    Self::List(a) => a.iter_mut().for_each(|a| a.neg_assign()),
                }
            }
        }
        impl Neg for Number<$ty> {
            type Output = Self;
            fn neg(mut self) -> Self {
                self.neg_assign();
                self
            }
        }
        impl Display for Number<$ty> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                match self {
                    Self::Value(a) => write!(f, "{}", a),
                    #[cfg(feature = "list")]
                    Self::List(a) => {
                        write!(f, "[")?;
                        let mut first = true;
                        for a in a.iter() {
                            if !first {
                                write!(f, ",")?
                            } else {
                                first = false;
                            }
                            write!(f, "{}", a)?
                        }
                        write!(f, "]")
                    }
                }
            }
        }
        impl_ops!($ty, Add, AddAssign, add, add_assign);
        impl_ops!($ty, Sub, SubAssign, sub, sub_assign);
        impl_ops!($ty, Mul, MulAssign, mul, mul_assign);
        impl_ops!($ty, Div, DivAssign, div, div_assign);
        impl_ops!($ty, Rem, RemAssign, rem, rem_assign);
        impl_ops!($ty, Pow, PowAssign, pow, pow_assign);
    };
}
impl_num!(Complex);
impl_lower_ops!(Complex, Float, Add, AddAssign, add, add_assign);
impl_lower_ops!(Complex, Float, Sub, SubAssign, sub, sub_assign);
impl_lower_ops!(Complex, Float, Mul, MulAssign, mul, mul_assign);
impl_lower_ops!(Complex, Float, Div, DivAssign, div, div_assign);
impl_lower_ops!(Complex, Float, Pow, PowAssign, pow, pow_assign);
impl_num!(Float);
