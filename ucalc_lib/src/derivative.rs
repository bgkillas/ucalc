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
impl Compute<'_> {
    pub(crate) fn derivative(
        self,
        inner_vars: &mut Vec<Number>,
        stack: &mut Vec<StackToken>,
        #[cfg(feature = "float_rand")] rand: &mut Rand,
    ) -> Option<Number> {
        match *self.tokens.last().unwrap() {
            Token::CustomFun(_, d) => {
                if d.get() != 0 {
                    todo!()
                }
                todo!()
            }
            Token::Function(fun, d) => {
                if d.get() != 0 {
                    todo!()
                }
                let derivative = Derivative::from(fun);
                if derivative.is_none() {
                    None
                } else {
                    let tokens = &self.tokens[..self.tokens.len() - 1];
                    match fun.inputs().get() {
                        1 => {
                            let g_prime = self.tokens(tokens).derivative(
                                inner_vars,
                                stack,
                                #[cfg(feature = "float_rand")]
                                rand,
                            )?;
                            let mut g = self.tokens(tokens).compute(
                                inner_vars,
                                stack,
                                #[cfg(feature = "float_rand")]
                                rand,
                            );
                            derivative.compute_on_1(&mut g);
                            Some(g_prime * g)
                        }
                        2 => {
                            let (right_tokens, last) = tokens.get_from_last(self.custom_funs);
                            let left_tokens = &self.tokens[..last];
                            let g_prime = self.tokens(left_tokens).derivative(
                                inner_vars,
                                stack,
                                #[cfg(feature = "float_rand")]
                                rand,
                            )?;
                            let h_prime = self.tokens(right_tokens).derivative(
                                inner_vars,
                                stack,
                                #[cfg(feature = "float_rand")]
                                rand,
                            )?;
                            let mut g1 = self.tokens(left_tokens).compute(
                                inner_vars,
                                stack,
                                #[cfg(feature = "float_rand")]
                                rand,
                            );
                            let mut g2 = g1.clone();
                            let h = self.tokens(right_tokens).compute(
                                inner_vars,
                                stack,
                                #[cfg(feature = "float_rand")]
                                rand,
                            );
                            derivative.compute_on_2_first(&mut g1, &h);
                            derivative.compute_on_2_second(&mut g2, h);
                            Some(g_prime * g1 + h_prime * g2)
                        }
                        _ => todo!(),
                    }
                }
            }
            Token::InnerVar(_) => Some(Number::from(1)),
            Token::Number(_) => Some(Number::default()),
            _ => None,
        }
    }
}
