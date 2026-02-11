use crate::inverse::Inverse;
use crate::tokens::TokensRef;
use crate::{Function, Functions, Number, Operators, Token, Tokens, Variables};
use std::mem;
use ucalc_numbers::{Float, FloatTrait, NegAssign, Pow};
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Poly(pub Vec<Number>);
#[derive(Debug, Clone)]
pub struct PolyRef<'a>(pub &'a [Number]);
#[derive(Debug, PartialEq, Clone)]
pub enum Func {
    Function(Function),
    Power(Number),
}
impl From<Func> for Inverse {
    fn from(value: Func) -> Self {
        match value {
            Func::Function(func) => func.into(),
            Func::Power(_) => {
                todo!()
            }
        }
    }
}
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Polynomial {
    pub quotient: Poly,
    pub divisor: Poly,
    pub functions: Vec<Func>,
}
#[derive(Debug, Clone)]
pub struct PolynomialRef<'a> {
    pub quotient: PolyRef<'a>,
    pub divisor: PolyRef<'a>,
}
impl Poly {
    pub fn as_ref(&self) -> PolyRef<'_> {
        PolyRef(self)
    }
    pub fn mul_buffer(mut self, rhs: &Self, buffer: &mut Poly) -> Self {
        self.mul_assign_buffer(rhs, buffer);
        self
    }
    pub fn mul_assign_buffer(&mut self, rhs: &Self, buffer: &mut Poly) {
        buffer
            .0
            .resize(self.len() + rhs.len() - 1, Number::default());
        mem::swap(self, buffer);
        for (i, a) in buffer.iter_mut().enumerate() {
            if !a.is_zero() {
                for (j, b) in rhs.iter().enumerate() {
                    if !b.is_zero() {
                        self[i + j] = a.clone() * b.clone();
                    }
                }
                *a = Number::default()
            }
        }
    }
}
impl PolynomialRef<'_> {
    #[cfg(feature = "list")]
    pub fn roots(&self) -> Option<Number> {
        let mut roots = self.quotient.roots()?;
        if self.divisor.len() != 1 {
            let anti_roots = self.divisor.roots()?;
            roots.retain(|r| !anti_roots.contains(r));
        }
        Some(ucalc_numbers::Number::List(roots))
    }
    #[cfg(not(feature = "list"))]
    pub fn roots(&self) -> Option<Vec<Number>> {
        let mut roots = self.quotient.roots()?;
        if self.divisor.len() != 1 {
            let anti_roots = self.divisor.roots()?;
            for r in anti_roots {
                if let Some(i) = roots.iter().position(|a| *a == r) {
                    roots.remove(i);
                }
            }
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
    pub fn roots(&self) -> Option<Vec<Number>> {
        match self.len() {
            2 => Some(vec![self.linear()]),
            3 => Some(self.quadratic().into()),
            4 => Some(self.cubic().into()),
            5 => Some(self.quartic().into()),
            //TODO
            _ => None,
        }
    }
    pub fn linear(&self) -> Number {
        self[0].clone() / self[1].clone()
    }
    pub fn quadratic(&self) -> [Number; 2] {
        let t = self[2].clone() * Float::from(2);
        let a = -self[1].clone() / t.clone();
        let b = (self[1].clone() * self[1].clone()
            - self[2].clone() * self[0].clone() * Float::from(4))
        .sqrt()
            / t;
        [a.clone() + b.clone(), a - b]
    }
    pub fn cubic(&self) -> [Number; 3] {
        todo!()
    }
    pub fn quartic(&self) -> [Number; 4] {
        todo!()
    }
}
impl Polynomial {
    pub fn new() -> Self {
        let mut quotient = Vec::with_capacity(8);
        quotient.push(Number::from(0));
        quotient.push(Number::from(1));
        let mut divisor = Vec::with_capacity(8);
        divisor.push(Number::from(1));
        Self {
            quotient: quotient.into(),
            divisor: divisor.into(),
            functions: Vec::with_capacity(8),
        }
    }
    pub fn recip(mut self) -> Self {
        mem::swap(&mut self.quotient, &mut self.divisor);
        self
    }
    pub fn neg_mut(&mut self) {
        self.quotient.iter_mut().for_each(|a| a.neg_assign())
    }
    pub fn as_ref(&self) -> PolynomialRef<'_> {
        PolynomialRef {
            quotient: self.quotient.as_ref(),
            divisor: self.divisor.as_ref(),
        }
    }
    #[cfg(feature = "list")]
    pub fn roots(self) -> Option<Number> {
        let mut ret = self.as_ref().roots()?;
        ret.iter_mut().for_each(|a| {
            self.functions
                .iter()
                .rev()
                .for_each(|f| Inverse::from(*f).get_inverse().unwrap().compute_on(a, &[]))
        });
        Some(ret)
    }
    #[cfg(not(feature = "list"))]
    pub fn roots(self) -> Option<Number> {
        let mut ret = self.as_ref().roots()?;
        ret.iter_mut().for_each(|a| {
            self.functions.iter().rev().for_each(|f| {
                Inverse::from(f.clone())
                    .get_inverse()
                    .unwrap()
                    .compute_on(a, &[])
            })
        });
        Some(ret[0].clone())
    }
    pub fn is_constant(&self) -> bool {
        self.quotient.len() <= 1 && self.divisor.len() <= 1
    }
    fn mul_buffer(self, rhs: &Self, buffer: &mut Poly) -> Option<Self> {
        if self.functions != rhs.functions && !self.is_constant() && !rhs.is_constant() {
            return None;
        }
        let constant = self.is_constant();
        Some(Self {
            quotient: self.quotient.mul_buffer(&rhs.quotient, buffer),
            divisor: self.divisor.mul_buffer(&rhs.divisor, buffer),
            functions: if constant {
                rhs.functions.clone()
            } else {
                self.functions
            },
        })
    }
    fn div_buffer(self, rhs: &Self, buffer: &mut Poly) -> Option<Self> {
        if self.functions != rhs.functions && !self.is_constant() && !rhs.is_constant() {
            return None;
        }
        let constant = self.is_constant();
        Some(Self {
            quotient: self.quotient.mul_buffer(&rhs.divisor, buffer),
            divisor: self.divisor.mul_buffer(&rhs.quotient, buffer),
            functions: if constant {
                rhs.functions.clone()
            } else {
                self.functions
            },
        })
    }
    fn add_buffer(self, rhs: &Self, buffer: &mut Poly) -> Option<Self> {
        if self.functions != rhs.functions && !self.is_constant() && !rhs.is_constant() {
            return None;
        }
        let constant = self.is_constant();
        Some(Self {
            quotient: self.quotient.mul_buffer(&rhs.divisor, buffer)
                + self.divisor.clone().mul_buffer(&rhs.quotient, buffer),
            divisor: self.divisor.mul_buffer(&rhs.divisor, buffer),
            functions: if constant {
                rhs.functions.clone()
            } else {
                self.functions
            },
        })
    }
    fn sub_buffer(self, rhs: &Self, buffer: &mut Poly) -> Option<Self> {
        if self.functions != rhs.functions && !self.is_constant() && !rhs.is_constant() {
            return None;
        }
        let constant = self.is_constant();
        Some(Self {
            quotient: self.quotient.mul_buffer(&rhs.divisor, buffer)
                - self.divisor.clone().mul_buffer(&rhs.quotient, buffer),
            divisor: self.divisor.mul_buffer(&rhs.divisor, buffer),
            functions: if constant {
                rhs.functions.clone()
            } else {
                self.functions
            },
        })
    }
}
impl TokensRef<'_> {
    #[allow(clippy::too_many_arguments)]
    pub fn compute_polynomial(
        &self,
        fun_vars: &mut Vec<Number>,
        vars: &[Number],
        funs: &Functions,
        custom_vars: &Variables,
        stack: &mut Tokens,
        offset: usize,
        to_poly: Option<usize>,
    ) -> Option<Token> {
        let mut i = 0;
        let mut poly = Vec::with_capacity(8).into();
        while i < self.len() {
            let len = stack.len();
            match &self[i] {
                Token::Operator(operator) => {
                    let inputs = operator.inputs();
                    operator.compute_poly(&mut stack[len - inputs..], &mut poly)?;
                    stack.drain(len + 1 - inputs..);
                }
                Token::Var(index) => stack.push(Token::Num(custom_vars[*index].value.clone())),
                Token::Fun(index) => {
                    let inputs = funs[*index].inputs;
                    let end = fun_vars.len();
                    fun_vars.push(stack[len - inputs].num_ref().clone());
                    fun_vars.extend(stack.drain(len + 1 - inputs..).map(|n| n.num()));
                    stack[len - inputs] = TokensRef(&funs[*index].tokens).compute_polynomial(
                        fun_vars,
                        vars,
                        funs,
                        custom_vars,
                        &mut Tokens(Vec::with_capacity(funs[*index].tokens.len())),
                        end,
                        None,
                    )?;
                    fun_vars.drain(end..);
                }
                Token::Num(n) => {
                    stack.push(Token::Num(n.clone()));
                }
                Token::InnerVar(index) => {
                    if Some(*index) == to_poly {
                        stack.push(Polynomial::new().into())
                    } else {
                        stack.push(Token::Num(fun_vars[offset + index].clone()))
                    }
                }
                Token::GraphVar(index) => stack.push(Token::Num(vars[*index].clone())),
                Token::Skip(to) => {
                    let back = stack.len();
                    stack.extend_from_slice(&self[i + 1..=i + to]);
                    stack.push(Token::Skip(back));
                    i += to;
                }
                Token::Polynomial(_) => unreachable!(),
            }
            i += 1;
        }
        Some(stack.remove(0))
    }
}
impl Function {
    pub fn compute_poly(self, a: &mut Polynomial) {
        //TODO
        a.functions.push(Func::Function(self));
    }
}
impl Operators {
    pub fn compute_poly(self, a: &mut [Token], buffer: &mut Poly) -> Option<()> {
        let ([a], b) = a.split_first_chunk_mut().unwrap();
        self.compute_poly_on(a, b, buffer)
    }
    fn compute_poly_on(self, a: &mut Token, b: &mut [Token], buffer: &mut Poly) -> Option<()> {
        if let Token::Polynomial(a) = a {
            if b.len() == 1 {
                if let Token::Num(n) = b[0].clone() {
                    self.poly_complex(a, n);
                } else {
                    let b = b[0].poly_ref();
                    self.poly(a, b, buffer);
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
    fn poly(self, a: &mut Polynomial, b: &Polynomial, buffer: &mut Poly) -> Option<()> {
        let old = mem::take(a);
        match self {
            Self::Add => *a = old.add_buffer(b, buffer)?,
            Self::Sub => *a = old.sub_buffer(b, buffer)?,
            Self::Mul => *a = old.mul_buffer(b, buffer)?,
            Self::Div => *a = old.div_buffer(b, buffer)?,
            _ => return None,
        }
        Some(())
    }
    fn poly_complex(self, a: &mut Polynomial, b: Number) -> Option<()> {
        match self {
            Self::Add => *a += b,
            Self::Sub => *a -= b,
            Self::Mul => *a *= b,
            Self::Div => *a /= b,
            Self::Pow => {
                let old = mem::take(a);
                *a = old.pow(b)?
            }
            Self::Root => {
                let old = mem::take(a);
                *a = old.pow(b.recip())?
            }
            _ => return None,
        }
        Some(())
    }
    fn complex_poly(self, a: &Number, b: Polynomial) -> Option<Polynomial> {
        Some(match self {
            Self::Add => b + a.clone(),
            Self::Sub => a.clone() - b,
            Self::Mul => b * a.clone(),
            Self::Div => a.clone() / b,
            _ => return None,
        })
    }
}
