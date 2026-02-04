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
impl Deref for Poly {
    type Target = Vec<Complex>;
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
    type Target = &'a [Complex];
    fn deref(&self) -> &Self::Target {
        &self.0
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
    pub fn inverse(&self) -> Option<Vec<Complex>> {
        let mut roots = self.quotient.inverse()?;
        if self.divisor.len() != 1 {
            let anti_roots = self.divisor.inverse()?;
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
    pub fn inverse(&self) -> Option<Vec<Complex>> {
        match self.len() {
            2 => Some(vec![self.linear()]),
            3 => Some(self.quadratic().into()),
            //TODO
            _ => None,
        }
    }
    pub fn quadratic(&self) -> [Complex; 2] {
        println!("{self:?}");
        let a = -self[1] / (self[2] * 2);
        let b = (self[1] * self[1] - self[2] * self[0] * 4).sqrt() / (self[2] * 2);
        dbg!([a - b, a + b])
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
    pub fn pow_mut(&mut self, _other: &Self) -> Option<()> {
        todo!()
    }
    pub fn root_mut(&mut self, _other: &Self) -> Option<()> {
        todo!()
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
    pub fn inverse(self) -> Option<Vec<Complex>> {
        let mut ret = self.as_ref().inverse()?;
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
}
impl Pow<&Self> for Polynomial {
    fn pow(mut self, rhs: &Self) -> Self {
        self.pow_mut(rhs);
        self
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
    type Output = Option<Self>;
    fn sub(self, rhs: Complex) -> Self::Output {
        Some(Self {
            quotient: &self.quotient - &(&self.divisor * rhs),
            divisor: self.divisor,
            functions: self.functions,
        })
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
        if let Token::Num(n) = a {
            //TODO
            *a = Polynomial::from(*n).into()
        }
        let a = a.poly_mut();
        self.compute_poly_on(a, b)
    }
    pub fn compute_poly_on(self, a: &mut Polynomial, b: &[Token]) -> Option<()> {
        if b.len() == 1 {
            if let Token::Num(n) = b[0] {
                let b = Polynomial::from(n);
                self.poly_inner(a, &b);
            } else {
                let b = b[0].poly_ref();
                self.poly_inner(a, b);
            }
        } else {
            match self {
                Self::Negate => a.neg_mut(),
                Self::Function(fun) => fun.compute_poly(a),
                _ => {
                    unreachable!()
                }
            }
        }
        Some(())
    }
    fn poly_inner(self, a: &mut Polynomial, b: &Polynomial) -> Option<()> {
        match self {
            Self::Add => {
                let old = mem::take(a);
                *a = (old + b)?;
            }
            Self::Sub => {
                let old = mem::take(a);
                *a = (old - b)?
            }
            Self::Mul => {
                let old = mem::take(a);
                *a = (old * b)?
            }
            Self::Div => {
                let old = mem::take(a);
                *a = (old / b)?
            }
            Self::Pow => a.pow_mut(b)?,
            Self::Root => a.root_mut(b)?,
            _ => unreachable!(),
        }
        Some(())
    }
}
