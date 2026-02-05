use crate::inverse::Inverse;
use crate::parse::TokensRef;
use crate::{Function, Functions, Operators, Token, Tokens};
use std::mem;
use std::ops::{Add, Deref, DerefMut, Div, Mul, Neg, Sub};
use ucalc_numbers::{Complex, Pow};
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Poly(pub Vec<Complex>);
#[derive(Debug, PartialEq, Clone)]
pub struct PolyRef<'a>(pub &'a [Complex]);
impl<'a> From<&'a Poly> for PolyRef<'a> {
    fn from(value: &'a Poly) -> Self {
        Self(value)
    }
}
impl<'a> From<&'a [Complex]> for PolyRef<'a> {
    fn from(value: &'a [Complex]) -> Self {
        Self(value)
    }
}
impl Deref for Poly {
    type Target = [Complex];
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
    type Target = [Complex];
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl From<Vec<Complex>> for Poly {
    fn from(value: Vec<Complex>) -> Self {
        Self(value)
    }
}
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Polynomial {
    pub quotient: Poly,
    pub divisor: Poly,
    pub functions: Vec<Function>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct PolynomialRef<'a> {
    pub quotient: PolyRef<'a>,
    pub divisor: PolyRef<'a>,
}
impl From<Complex> for Polynomial {
    fn from(value: Complex) -> Self {
        Self {
            quotient: vec![value].into(),
            divisor: vec![Complex::from(1)].into(),
            functions: Vec::new(),
        }
    }
}
impl Poly {
    pub fn as_ref(&self) -> PolyRef<'_> {
        PolyRef(self)
    }
}
impl PolynomialRef<'_> {
    pub fn roots(&self) -> Option<Vec<Complex>> {
        let mut roots = self.quotient.roots()?;
        if self.divisor.len() != 1 {
            let anti_roots = self.divisor.roots()?;
            roots.retain(|r| !anti_roots.contains(r));
        }
        Some(roots)
    }
}
impl PolyRef<'_> {
    pub fn len(&self) -> usize {
        if let Some(n) = self.iter().rposition(|a| !a.is_zero()) {
            n + 1
        } else {
            0
        }
    }
    pub fn roots(&self) -> Option<Vec<Complex>> {
        match self.len() {
            2 => Some(vec![self.linear()]),
            3 => Some(self.quadratic().into()),
            //TODO
            _ => None,
        }
    }
    pub fn quadratic(&self) -> [Complex; 2] {
        let a = -self[1] / (self[2] * 2);
        let b = (self[1] * self[1] - self[2] * self[0] * 4).sqrt() / (self[2] * 2);
        [a - b, a + b]
    }
    pub fn linear(&self) -> Complex {
        self[0] / self[1]
    }
}
impl Polynomial {
    pub fn new() -> Self {
        Self {
            quotient: vec![Complex::from(0), Complex::from(1)].into(),
            divisor: vec![Complex::from(1)].into(),
            functions: Vec::new(),
        }
    }
    pub fn recip(mut self) -> Self {
        mem::swap(&mut self.quotient, &mut self.divisor);
        self
    }
    pub fn neg_mut(&mut self) {
        self.quotient.iter_mut().for_each(|a| *a = -*a)
    }
    pub fn as_ref(&self) -> PolynomialRef<'_> {
        PolynomialRef {
            quotient: self.quotient.as_ref(),
            divisor: self.divisor.as_ref(),
        }
    }
    pub fn roots(self) -> Option<Vec<Complex>> {
        let mut ret = self.as_ref().roots()?;
        ret.iter_mut().for_each(|a| {
            self.functions
                .iter()
                .rev()
                .for_each(|f| Inverse::from(*f).get_inverse().unwrap().compute_on(a, &[]))
        });
        Some(ret)
    }
    pub fn is_constant(&self) -> bool {
        self.quotient.len() <= 1 && self.divisor.len() <= 1
    }
    pub fn pow(self, rhs: Complex) -> Option<Self> {
        if rhs.imag.is_zero() && rhs.real.fract().is_zero() {
            let n = rhs.real.to_isize();
            let k = n.unsigned_abs();
            let mut poly = Self {
                quotient: self.quotient.pow(k),
                divisor: self.divisor.pow(k),
                functions: self.functions,
            };
            if n.is_negative() {
                poly = poly.recip()
            }
            Some(poly)
        } else {
            None
        }
    }
}
impl Pow<usize, Self> for Poly {
    fn pow(self, rhs: usize) -> Self {
        //TODO
        let mut poly = self.clone();
        for _ in 1..rhs {
            poly = &poly * &self;
        }
        poly
    }
}
impl Mul<&Self> for Polynomial {
    type Output = Option<Self>;
    fn mul(self, rhs: &Self) -> Self::Output {
        if self.functions != rhs.functions && !self.is_constant() && !rhs.is_constant() {
            return None;
        }
        Some(Self {
            quotient: &self.quotient * &rhs.quotient,
            divisor: &self.divisor * &rhs.divisor,
            functions: if self.is_constant() {
                rhs.functions.clone()
            } else {
                self.functions
            },
        })
    }
}
impl Div<&Self> for Polynomial {
    type Output = Option<Self>;
    fn div(self, rhs: &Self) -> Self::Output {
        if self.functions != rhs.functions && !self.is_constant() && !rhs.is_constant() {
            return None;
        }
        Some(Self {
            quotient: &self.quotient * &rhs.divisor,
            divisor: &self.divisor * &rhs.quotient,
            functions: if self.is_constant() {
                rhs.functions.clone()
            } else {
                self.functions
            },
        })
    }
}
impl Add<&Self> for Polynomial {
    type Output = Option<Self>;
    fn add(self, rhs: &Self) -> Self::Output {
        if self.functions != rhs.functions && !self.is_constant() && !rhs.is_constant() {
            return None;
        }
        Some(Self {
            quotient: &(&self.quotient * &rhs.divisor) + &(&self.divisor * &rhs.quotient),
            divisor: &self.divisor * &rhs.divisor,
            functions: if self.is_constant() {
                rhs.functions.clone()
            } else {
                self.functions
            },
        })
    }
}
impl Sub<&Self> for Polynomial {
    type Output = Option<Self>;
    fn sub(self, rhs: &Self) -> Self::Output {
        if self.functions != rhs.functions && !self.is_constant() && !rhs.is_constant() {
            return None;
        }
        Some(Self {
            quotient: &(&self.quotient * &rhs.divisor) - &(&self.divisor * &rhs.quotient),
            divisor: &self.divisor * &rhs.divisor,
            functions: if self.is_constant() {
                rhs.functions.clone()
            } else {
                self.functions
            },
        })
    }
}
impl Sub<Complex> for Polynomial {
    type Output = Self;
    fn sub(self, rhs: Complex) -> Self::Output {
        Self {
            quotient: &self.quotient - &(&self.divisor * rhs),
            divisor: self.divisor,
            functions: self.functions,
        }
    }
}
impl Add<Complex> for Polynomial {
    type Output = Self;
    fn add(self, rhs: Complex) -> Self::Output {
        Self {
            quotient: &self.quotient + &(&self.divisor * rhs),
            divisor: self.divisor,
            functions: self.functions,
        }
    }
}
impl Mul<Complex> for Polynomial {
    type Output = Self;
    fn mul(self, rhs: Complex) -> Self::Output {
        Self {
            quotient: &self.quotient * rhs,
            divisor: self.divisor,
            functions: self.functions,
        }
    }
}
impl Div<Complex> for Polynomial {
    type Output = Self;
    fn div(self, rhs: Complex) -> Self::Output {
        Self {
            quotient: &self.quotient / rhs,
            divisor: self.divisor,
            functions: self.functions,
        }
    }
}
impl Sub<Polynomial> for Complex {
    type Output = Polynomial;
    fn sub(self, rhs: Polynomial) -> Self::Output {
        Polynomial {
            quotient: &(&rhs.divisor * self) - &rhs.quotient,
            divisor: rhs.divisor,
            functions: rhs.functions,
        }
    }
}
impl Div<Polynomial> for Complex {
    type Output = Polynomial;
    fn div(self, rhs: Polynomial) -> Self::Output {
        Polynomial {
            quotient: self / &rhs.quotient,
            divisor: rhs.divisor,
            functions: rhs.functions,
        }
    }
}
impl Mul<Self> for &Poly {
    type Output = Poly;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut new = vec![Complex::from(0); self.len() * rhs.len()];
        for (i, a) in self.iter().enumerate() {
            for (j, b) in rhs.iter().enumerate() {
                new[i + j] = *a * *b;
            }
        }
        new.into()
    }
}
impl Mul<Complex> for &Poly {
    type Output = Poly;
    fn mul(self, rhs: Complex) -> Self::Output {
        let mut new = vec![Complex::from(0); self.len()];
        for (i, a) in self.iter().enumerate() {
            new[i] = *a * rhs;
        }
        new.into()
    }
}
impl Div<Complex> for &Poly {
    type Output = Poly;
    fn div(self, rhs: Complex) -> Self::Output {
        let mut new = vec![Complex::from(0); self.len()];
        for (i, a) in self.iter().enumerate() {
            new[i] = *a / rhs;
        }
        new.into()
    }
}
impl Div<&Poly> for Complex {
    type Output = Poly;
    fn div(self, rhs: &Poly) -> Self::Output {
        let mut new = vec![Complex::from(0); rhs.len()];
        for (i, a) in rhs.iter().enumerate() {
            new[i] = self / *a;
        }
        new.into()
    }
}
impl Sub<&Poly> for Complex {
    type Output = Poly;
    fn sub(self, rhs: &Poly) -> Self::Output {
        let mut new = vec![Complex::from(0); rhs.len()];
        for (i, a) in rhs.iter().enumerate() {
            new[i] = self - *a;
        }
        new.into()
    }
}
impl Add<Self> for &Poly {
    type Output = Poly;
    fn add(self, rhs: Self) -> Self::Output {
        let mut new = vec![Complex::from(0); self.len().max(rhs.len())];
        for (b, a) in self.iter().zip(new.iter_mut()) {
            *a = *b;
        }
        for (b, a) in rhs.iter().zip(new.iter_mut()) {
            *a += *b;
        }
        new.into()
    }
}
impl Sub<Self> for &Poly {
    type Output = Poly;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut new = vec![Complex::from(0); self.len().max(rhs.len())];
        for (b, a) in self.iter().zip(new.iter_mut()) {
            *a = *b;
        }
        for (b, a) in rhs.iter().zip(new.iter_mut()) {
            *a -= *b;
        }
        new.into()
    }
}
impl Neg for Poly {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        self.iter_mut().for_each(|x| *x = x.neg());
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
impl TokensRef<'_> {
    pub fn compute_polynomial(
        &self,
        _fun_vars: &mut Vec<Complex>,
        _vars: &[Complex],
        _funs: &Functions,
        stack: &mut Tokens,
        _offset: usize,
        to_poly: usize,
    ) -> Option<Polynomial> {
        let mut i = 0;
        while i < self.len() {
            let len = stack.len();
            match &self[i] {
                Token::Operator(operator) => {
                    let inputs = operator.inputs();
                    operator.compute_poly(&mut stack[len - inputs..])?;
                    stack.drain(len + 1 - inputs..);
                }
                Token::Fun(_) => {
                    todo!()
                }
                Token::Num(n) => {
                    stack.push(Token::Num(*n));
                }
                Token::InnerVar(v) => {
                    if *v == to_poly {
                        stack.push(Polynomial::new().into())
                    } else {
                        todo!()
                    }
                }
                Token::GraphVar(_) => {
                    todo!()
                }
                Token::Skip(_) => {
                    todo!()
                }
                Token::Polynomial(_) => unreachable!(),
            }
            i += 1;
        }
        Some(*stack.remove(0).poly())
    }
}
impl Function {
    pub fn compute_poly(self, a: &mut Polynomial) {
        //TODO
        a.functions.push(self);
    }
}
impl Operators {
    pub fn compute_poly(self, a: &mut [Token]) -> Option<()> {
        let ([a], b) = a.split_first_chunk_mut().unwrap();
        self.compute_poly_on(a, b)
    }
    fn compute_poly_on(self, a: &mut Token, b: &mut [Token]) -> Option<()> {
        if let Token::Polynomial(a) = a {
            if b.len() == 1 {
                if let Token::Num(n) = b[0] {
                    self.poly_complex(a, n);
                } else {
                    let b = b[0].poly_ref();
                    self.poly(a, b);
                }
            } else {
                match self {
                    Self::Negate => a.neg_mut(),
                    Self::Function(fun) => fun.compute_poly(a),
                    _ => return None,
                }
            }
        } else if let Token::Num(_) = b[0] {
            self.compute_on(a.num_mut(), b)
        } else if let Token::Num(c) = a {
            *a = self.complex_poly(c, mem::take(b[0].poly_mut()))?.into()
        }
        Some(())
    }
    fn poly(self, a: &mut Polynomial, b: &Polynomial) -> Option<()> {
        let old = mem::take(a);
        match self {
            Self::Add => *a = (old + b)?,
            Self::Sub => *a = (old - b)?,
            Self::Mul => *a = (old * b)?,
            Self::Div => *a = (old / b)?,
            _ => return None,
        }
        Some(())
    }
    fn poly_complex(self, a: &mut Polynomial, b: Complex) -> Option<()> {
        let old = mem::take(a);
        match self {
            Self::Add => *a = old + b,
            Self::Sub => *a = old - b,
            Self::Mul => *a = old * b,
            Self::Div => *a = old / b,
            Self::Pow => *a = old.pow(b)?,
            Self::Root => *a = old.pow(b.recip())?,
            _ => return None,
        }
        Some(())
    }
    fn complex_poly(self, a: &Complex, b: Polynomial) -> Option<Polynomial> {
        Some(match self {
            Self::Add => b + *a,
            Self::Sub => *a - b,
            Self::Mul => b * *a,
            Self::Div => *a / b,
            _ => return None,
        })
    }
}
