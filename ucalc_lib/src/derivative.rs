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
    Sin,
    Cos,
    Tan,
    None,
}
impl Derivative {
    pub fn is_none(self) -> bool {
        matches!(self, Self::None)
    }
    pub fn compute_on_1(self, a: &mut Number) {
        match self {
            Self::Negate => a.neg_assign(),
            Self::Sin => a.cos_mut(),
            Self::Cos => {
                a.sin_mut();
                a.neg_assign()
            }
            Self::Tan => {
                a.cos_mut();
                a.recip_mut();
                *a *= a.clone();
            }
            _ => unreachable!(),
        }
    }
    pub fn compute_on_2<const N: usize>(self, a: &mut Number, b: &Number) {
        match self {
            Self::Add => *a = Number::from(1),
            Self::Sub => *a = Number::from(if N == 0 { 1 } else { -1 }),
            Self::Mul => {
                if N == 0 {
                    *a = b.clone()
                }
            }
            Self::Div => {
                if N == 0 {
                    *a = b.clone().recip()
                } else {
                    a.neg_assign();
                    *a /= b.clone() * b
                }
            }
            Self::Pow => {
                if N == 0 {
                    a.pow_assign(b.clone() - Float::from(1));
                    *a *= b;
                } else {
                    let l = a.clone().pow(b);
                    a.ln_mut();
                    *a *= l;
                }
            }
            Self::Root => {
                if N == 0 {
                    a.pow_assign(b.clone().recip() - Float::from(1));
                    *a /= b;
                } else {
                    let l = a.clone().pow(b.clone().recip());
                    a.ln_mut();
                    *a *= l;
                    *a /= b.clone() * b;
                }
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
            Function::Sin => Self::Sin,
            Function::Cos => Self::Cos,
            Function::Tan => Self::Tan,
            _ => Self::None,
        }
    }
}
#[derive(Debug)]
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
        &self,
        inner_vars: &mut Vec<Number>,
        stack: &mut Vec<StackToken>,
        var: u16,
        #[cfg(feature = "float_rand")] rand: &mut Rand,
    ) -> Option<Number> {
        let mut tokens = self.tokens.iter().enumerate();
        let stack_end = stack.len();
        while let Some((i, token)) = tokens.next() {
            match token {
                &Token::CustomFun(n, d) => {
                    if d.get() != 0 {
                        todo!()
                    }
                    let end = inner_vars.len();
                    let len = stack.len();
                    let fun = &self.custom_funs[n as usize];
                    let inputs = fun.inputs.get() as usize;
                    let compute = self.tokens(&fun.tokens[..]).offset(end);
                    inner_vars.extend(
                        stack[len - inputs..]
                            .iter()
                            .map(|a| a.diff_ref().value.clone()),
                    );
                    stack[len - inputs].diff_mut().value = compute.compute(
                        inner_vars,
                        stack,
                        #[cfg(feature = "float_rand")]
                        rand,
                    );
                    let Some(d) = compute.derivative(
                        inner_vars,
                        stack,
                        end as u16,
                        #[cfg(feature = "float_rand")]
                        rand,
                    ) else {
                        stack.drain(stack_end..);
                        return None;
                    };
                    stack[len - inputs].diff_mut().derivative *= d;
                    for i in 1..inputs {
                        let Some(mut d) = compute.derivative(
                            inner_vars,
                            stack,
                            (end + i) as u16,
                            #[cfg(feature = "float_rand")]
                            rand,
                        ) else {
                            stack.drain(stack_end..);
                            return None;
                        };
                        d *= &stack[len - inputs + i].diff_ref().derivative;
                        stack[len - inputs].diff_mut().derivative += d;
                    }
                    inner_vars.drain(end..);
                    stack.drain(len - inputs + 1..);
                }
                &Token::Function(fun, d) => {
                    if d.get() != 0 {
                        todo!()
                    }
                    let derivative = Derivative::from(fun);
                    if derivative.is_none() {
                        stack.drain(stack_end..);
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
                            derivative.compute_on_2::<0>(&mut d1, &h.value);
                            derivative.compute_on_2::<1>(&mut d2, &h.value);
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
                &Token::InnerVar(n) => stack.push(
                    DiffToken::new(
                        inner_vars[self.offset + n as usize].clone(),
                        if self.offset + n as usize == var as usize {
                            Number::from(1)
                        } else {
                            Number::default()
                        },
                    )
                    .into(),
                ),
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
