use crate::Number;
use crate::polynomial::{Poly, PolyRef, Polynomial};
use std::mem;
use std::ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
};
use ucalc_numbers::{FloatTrait, NegAssign, Pow, PowAssign, RealTrait};
impl<'a> From<&'a Poly> for PolyRef<'a> {
    fn from(value: &'a Poly) -> Self {
        Self(value)
    }
}
impl<'a> From<&'a [Number]> for PolyRef<'a> {
    fn from(value: &'a [Number]) -> Self {
        Self(value)
    }
}
impl Deref for Poly {
    type Target = [Number];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Poly {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<'a> Deref for PolyRef<'a> {
    type Target = [Number];
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl From<Vec<Number>> for Poly {
    fn from(value: Vec<Number>) -> Self {
        Self(value)
    }
}
impl From<Number> for Polynomial {
    fn from(value: Number) -> Self {
        let mut quotient = Vec::with_capacity(8);
        quotient.push(value);
        let mut divisor = Vec::with_capacity(8);
        divisor.push(Number::from(1));
        Self {
            quotient: quotient.into(),
            divisor: divisor.into(),
            functions: Vec::with_capacity(8),
        }
    }
}
impl Pow<Number> for Polynomial {
    type Output = Option<Self>;
    fn pow(mut self, rhs: Number) -> Self::Output {
        #[cfg(feature = "complex")]
        if !rhs.imag.is_zero() {
            return None;
        }
        if rhs.real().clone().fract().is_zero() {
            let n = rhs.real().clone().into_isize();
            let k = n.unsigned_abs();
            self.quotient.pow_assign(k);
            self.divisor.pow_assign(k);
            if n.is_negative() {
                self = self.recip()
            }
            Some(self)
        } else {
            None
        }
    }
}
impl Pow<usize> for Poly {
    type Output = Self;
    fn pow(self, rhs: usize) -> Self {
        //TODO
        let mut poly = self.clone();
        let mut buffer = Vec::with_capacity(8).into();
        for _ in 1..rhs {
            poly.mul_assign_buffer(&self, &mut buffer);
        }
        poly
    }
}
impl Sub<Number> for Polynomial {
    type Output = Self;
    fn sub(self, rhs: Number) -> Self::Output {
        Self {
            quotient: self.quotient - (self.divisor.clone() * rhs),
            divisor: self.divisor,
            functions: self.functions,
        }
    }
}
impl Add<Number> for Polynomial {
    type Output = Self;
    fn add(self, rhs: Number) -> Self::Output {
        Self {
            quotient: self.quotient + (self.divisor.clone() * rhs),
            divisor: self.divisor,
            functions: self.functions,
        }
    }
}
impl Mul<Number> for Polynomial {
    type Output = Self;
    fn mul(self, rhs: Number) -> Self::Output {
        Self {
            quotient: self.quotient * rhs,
            divisor: self.divisor,
            functions: self.functions,
        }
    }
}
impl Div<Number> for Polynomial {
    type Output = Self;
    fn div(self, rhs: Number) -> Self::Output {
        Self {
            quotient: self.quotient / rhs,
            divisor: self.divisor,
            functions: self.functions,
        }
    }
}
impl Sub<Polynomial> for Number {
    type Output = Polynomial;
    fn sub(self, rhs: Polynomial) -> Self::Output {
        Polynomial {
            quotient: (rhs.divisor.clone() * self) - rhs.quotient,
            divisor: rhs.divisor,
            functions: rhs.functions,
        }
    }
}
#[allow(clippy::suspicious_arithmetic_impl)]
impl Div<Polynomial> for Number {
    type Output = Polynomial;
    fn div(self, rhs: Polynomial) -> Self::Output {
        Polynomial {
            quotient: rhs.divisor * self,
            divisor: rhs.quotient,
            functions: rhs.functions,
        }
    }
}
impl MulAssign<Number> for Polynomial {
    fn mul_assign(&mut self, rhs: Number) {
        self.quotient *= rhs;
    }
}
impl DivAssign<Number> for Polynomial {
    fn div_assign(&mut self, rhs: Number) {
        self.quotient /= rhs;
    }
}
impl SubAssign<Number> for Polynomial {
    fn sub_assign(&mut self, rhs: Number) {
        self.quotient -= self.divisor.clone() * rhs;
    }
}
impl AddAssign<Number> for Polynomial {
    fn add_assign(&mut self, rhs: Number) {
        self.quotient += self.divisor.clone() * rhs;
    }
}
impl Mul<Number> for Poly {
    type Output = Poly;
    fn mul(mut self, rhs: Number) -> Self::Output {
        self *= rhs;
        self
    }
}
impl Div<Number> for Poly {
    type Output = Poly;
    fn div(mut self, rhs: Number) -> Self::Output {
        self /= rhs;
        self
    }
}
impl Sub<Poly> for Number {
    type Output = Poly;
    fn sub(self, mut rhs: Poly) -> Self::Output {
        rhs.iter_mut().for_each(|b| *b = self.clone() - b.clone());
        rhs
    }
}
impl MulAssign<Number> for Poly {
    fn mul_assign(&mut self, rhs: Number) {
        self.iter_mut().for_each(|c| *c *= rhs.clone())
    }
}
impl DivAssign<Number> for Poly {
    fn div_assign(&mut self, rhs: Number) {
        self.iter_mut().for_each(|c| *c /= rhs.clone())
    }
}
impl AddAssign<Poly> for Poly {
    fn add_assign(&mut self, mut rhs: Poly) {
        if rhs.len() > self.len() {
            mem::swap(self, &mut rhs)
        }
        self.iter_mut().zip(rhs.0).for_each(|(a, b)| *a += b);
    }
}
impl SubAssign<Poly> for Poly {
    fn sub_assign(&mut self, mut rhs: Poly) {
        if rhs.len() > self.len() {
            mem::swap(self, &mut rhs)
        }
        self.iter_mut().zip(rhs.0).for_each(|(a, b)| *a -= b);
    }
}
impl Add<Self> for Poly {
    type Output = Poly;
    fn add(mut self, mut rhs: Self) -> Self::Output {
        if rhs.len() > self.len() {
            mem::swap(&mut self, &mut rhs)
        }
        self.iter_mut().zip(rhs.0).for_each(|(a, b)| *a += b);
        self
    }
}
impl Sub<Self> for Poly {
    type Output = Poly;
    fn sub(mut self, mut rhs: Self) -> Self::Output {
        if rhs.len() > self.len() {
            mem::swap(&mut self, &mut rhs)
        }
        self.iter_mut().zip(rhs.0).for_each(|(a, b)| *a -= b);
        self
    }
}
impl Neg for Poly {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        self.iter_mut().for_each(|x| x.neg_assign());
        self
    }
}
impl Neg for Polynomial {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        self.neg_mut();
        self
    }
}
