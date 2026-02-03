use crate::parse::TokensRef;
use crate::{Function, Functions, Operators, Token, Tokens};
use std::mem;
use std::ops::{Add, Deref, DerefMut, Div, Mul, Neg, Sub};
use ucalc_numbers::{Complex, Pow};
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Poly(pub Vec<Complex>);
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
impl From<Complex> for Polynomial {
    fn from(value: Complex) -> Self {
        Self {
            quotient: vec![value].into(),
            divisor: vec![Complex::from(1)].into(),
            functions: Vec::new(),
        }
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
        if self.functions != rhs.functions {
            return None;
        }
        Some(Self {
            quotient: &self.quotient * &rhs.quotient,
            divisor: &self.divisor * &rhs.divisor,
            functions: self.functions,
        })
    }
}
impl Div<&Self> for Polynomial {
    type Output = Option<Self>;
    fn div(self, rhs: &Self) -> Self::Output {
        if self.functions != rhs.functions {
            return None;
        }
        Some(Self {
            quotient: &self.quotient * &rhs.divisor,
            divisor: &self.divisor * &rhs.quotient,
            functions: self.functions,
        })
    }
}
impl Add<&Self> for Polynomial {
    type Output = Option<Self>;
    fn add(self, rhs: &Self) -> Self::Output {
        if self.functions != rhs.functions {
            return None;
        }
        Some(Self {
            quotient: &(&self.quotient * &rhs.divisor) + &(&self.divisor * &rhs.quotient),
            divisor: &self.divisor * &rhs.divisor,
            functions: self.functions,
        })
    }
}
impl Sub<&Self> for Polynomial {
    type Output = Option<Self>;
    fn sub(self, rhs: &Self) -> Self::Output {
        if self.functions != rhs.functions {
            return None;
        }
        Some(Self {
            quotient: &(&self.quotient * &rhs.divisor) - &(&self.divisor * &rhs.quotient),
            divisor: &self.divisor * &rhs.divisor,
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
impl Add<Self> for &Poly {
    type Output = Poly;
    fn add(self, rhs: Self) -> Self::Output {
        let mut new = vec![Complex::from(0); self.len() + rhs.len()];
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
        let mut new = vec![Complex::from(0); self.len() + rhs.len()];
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
    ) -> Option<()> {
        let i = 0;
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
                Token::Num(n) => stack.push(Token::Num(*n)),
                Token::InnerVar(i) => {
                    if *i == to_poly {
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
        }
        Some(())
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
                *a = (old + b)?
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
