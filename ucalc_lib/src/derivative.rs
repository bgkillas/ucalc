use crate::compute::StackToken;
use crate::{Compute, Function, Number, Rand, Token};
use ucalc_numbers::{Float, FloatFunctions, FloatFunctionsMut, NegAssign, Pow, PowAssign};
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Derivative {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Root,
    Negate,
    None,
}
impl Derivative {
    pub fn is_none(self) -> bool {
        matches!(self, Self::None)
    }
    pub fn compute_on_1(self, a: &mut Number) {
        match self {
            Self::Negate => a.neg_assign(),
            _ => unreachable!(),
        }
    }
    pub fn compute_on_2_first(self, a: &mut Number, b: &Number) {
        match self {
            Self::Add => *a = Number::from(1),
            Self::Sub => *a = Number::from(1),
            Self::Mul => *a = b.clone(),
            Self::Div => *a = b.clone().recip(),
            Self::Pow => {
                a.pow_assign(b.clone() - Float::from(1));
                *a *= b;
            }
            Self::Root => {
                a.pow_assign(b.clone().recip() - Float::from(1));
                *a /= b;
            }
            _ => unreachable!(),
        }
    }
    pub fn compute_on_2_second(self, a: &mut Number, b: Number) {
        match self {
            Self::Add => *a = Number::from(1),
            Self::Sub => *a = Number::from(-1),
            Self::Mul => {}
            Self::Div => {
                a.neg_assign();
                *a /= b.clone() * b;
            }
            Self::Pow => {
                let l = a.clone().pow(&b);
                a.ln_mut();
                *a *= l;
            }
            Self::Root => {
                let l = a.clone().pow(b.clone().recip());
                a.ln_mut();
                *a *= l;
                *a /= b.clone() * b;
            }
            _ => unreachable!(),
        }
    }
}
impl From<Function> for Derivative {
    fn from(value: Function) -> Self {
        match value {
            Function::Add => Self::Add,
            Function::Sub => Self::Sub,
            Function::Mul => Self::Mul,
            Function::Div => Self::Div,
            Function::Pow => Self::Pow,
            Function::Root => Self::Root,
            Function::Negate => Self::Negate,
            _ => Self::None,
        }
    }
}
pub struct DiffToken {
    value: Number,
    derivative: Number,
}
impl DiffToken {
    pub fn new(value: Number, derivative: Number) -> Self {
        Self { value, derivative }
    }
}
impl Compute<'_> {
    pub(crate) fn derivative(
        self,
        inner_vars: &[Number],
        stack: &mut Vec<StackToken>,
        point: Number,
        var: u16,
        #[cfg(feature = "float_rand")] rand: &mut Rand,
    ) -> Option<Number> {
        let mut tokens = self.tokens.iter().enumerate();
        let end = stack.len();
        while let Some((i, token)) = tokens.next() {
            match token {
                &Token::CustomFun(_, d) => {
                    if d.get() != 0 {
                        todo!()
                    }
                    todo!()
                }
                &Token::Function(fun, d) => {
                    if d.get() != 0 {
                        todo!()
                    }
                    let derivative = Derivative::from(fun);
                    if derivative.is_none() {
                        stack.drain(end..);
                        return None;
                    }
                    match fun.inputs().get() {
                        1 => {
                            let g = stack.last_mut().unwrap().diff_mut();
                            let mut f_g = g.value.clone();
                            derivative.compute_on_1(&mut f_g);
                            fun.compute_on_1(&mut g.value);
                            g.derivative *= f_g;
                        }
                        2 => {
                            let h = stack.pop().unwrap().diff();
                            let g = stack.last_mut().unwrap().diff_mut();
                            let mut d1 = g.value.clone();
                            let mut d2 = g.value.clone();
                            derivative.compute_on_2_first(&mut d1, &h.value);
                            derivative.compute_on_2_second(&mut d2, h.value.clone());
                            fun.compute_on_2(
                                &mut g.value,
                                h.value,
                                #[cfg(feature = "float_rand")]
                                rand,
                            );
                            g.derivative *= d1;
                            g.derivative += h.derivative * d2;
                        }
                        _ => todo!(),
                    }
                }
                &Token::InnerVar(n) => {
                    if n == var {
                        stack.push(DiffToken::new(point.clone(), Number::from(1)).into())
                    } else {
                        stack.push(
                            DiffToken::new(inner_vars[n as usize].clone(), Number::default())
                                .into(),
                        )
                    }
                }
                &Token::Skip(to) => {
                    stack.push(StackToken::Skip(i + 1));
                    tokens.advance_by(to).unwrap();
                }
                Token::Number(n) => stack.push(DiffToken::new(n.clone(), Number::default()).into()),
                _ => todo!(),
            }
        }
        Some(stack.pop().unwrap().diff().derivative)
    }
}
