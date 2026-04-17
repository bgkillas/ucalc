#[cfg(feature = "float_rand")]
use crate::Rand;
use crate::compute::StackToken;
use crate::{Compute, Function, Number, Token};
use ucalc_numbers::{Float, FloatFunctions, FloatFunctionsMut, NegAssign, Pow, PowAssign};
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Derivative {
    Add,
    #[cfg(feature = "complex")]
    Addi,
    Sub,
    Mul,
    Div,
    Pow,
    Root,
    Negate,
    Sin,
    Cos,
    Tan,
    Ln,
    Exp,
    Sqrt,
    Cbrt,
    Abs,
    #[cfg(feature = "complex")]
    Arg,
    Recip,
    #[cfg(feature = "complex")]
    Conj,
    Rem,
    Ceil,
    Floor,
    Round,
    Trunc,
    #[cfg(feature = "units")]
    Convert,
    Fract,
    #[cfg(feature = "complex")]
    Real,
    #[cfg(feature = "complex")]
    Imag,
}
impl Derivative {
    pub fn compute_on_1(self, a: &mut Number) -> Result<(), ()> {
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
            Self::Ln => a.recip_mut(),
            Self::Exp => a.exp_mut(),
            Self::Sqrt => {
                a.sqrt_mut();
                a.recip_mut();
                *a *= Float::from(0.5);
            }
            Self::Cbrt => {
                a.cbrt_mut();
                *a *= a.clone();
                a.recip_mut();
                *a /= Float::from(3);
            }
            Self::Abs => todo!(),
            Self::Recip => {
                a.recip_mut();
                *a *= a.clone();
                a.neg_assign();
            }
            Self::Ceil => todo!(),
            Self::Floor => todo!(),
            Self::Round => todo!(),
            Self::Trunc => todo!(),
            Self::Fract => todo!(),
            #[cfg(feature = "complex")]
            Self::Arg => todo!(),
            #[cfg(feature = "complex")]
            Self::Conj => todo!(),
            #[cfg(feature = "complex")]
            Self::Real => todo!(),
            #[cfg(feature = "complex")]
            Self::Imag => todo!(),
            _ => unreachable!(),
        }
        Ok(())
    }
    pub fn compute_on_2<const N: usize>(self, a: &mut Number, b: &Number) -> Result<(), ()> {
        match self {
            Self::Add => *a = Number::from(1),
            #[cfg(feature = "complex")]
            Self::Addi => {
                if N == 0 {
                    *a = Number::from(1)
                } else {
                    *a = Number::from((0, 1))
                }
            }
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
            #[cfg(feature = "units")]
            Self::Convert => todo!(),
            Self::Rem => todo!(),
            _ => unreachable!(),
        }
        Ok(())
    }
}
impl TryFrom<Function> for Derivative {
    type Error = ();
    fn try_from(value: Function) -> Result<Self, Self::Error> {
        Ok(match value {
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
            Function::Ln => Self::Ln,
            Function::Exp => Self::Exp,
            Function::Sqrt => Self::Sqrt,
            Function::Cbrt => Self::Cbrt,
            Function::Abs => Self::Abs,
            #[cfg(feature = "complex")]
            Function::Arg => Self::Arg,
            Function::Recip => Self::Recip,
            #[cfg(feature = "complex")]
            Function::Conj => Self::Conj,
            Function::Rem => Self::Rem,
            Function::Ceil => Self::Ceil,
            Function::Floor => Self::Floor,
            Function::Round => Self::Round,
            Function::Trunc => Self::Trunc,
            #[cfg(feature = "units")]
            Function::Convert => Self::Convert,
            Function::Fract => Self::Fract,
            #[cfg(feature = "complex")]
            Function::Real => Self::Real,
            #[cfg(feature = "complex")]
            Function::Imag => Self::Imag,
            #[cfg(feature = "complex")]
            Function::Addi => Self::Addi,
            _ => return Err(()),
        })
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
        let stack_end = stack.len();
        let res = try {
            let mut tokens = self.tokens.iter().enumerate();
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
                        let d = compute.derivative(
                            inner_vars,
                            stack,
                            end as u16,
                            #[cfg(feature = "float_rand")]
                            rand,
                        )?;
                        stack[len - inputs].diff_mut().derivative *= d;
                        for i in 1..inputs {
                            let mut d = compute.derivative(
                                inner_vars,
                                stack,
                                (end + i) as u16,
                                #[cfg(feature = "float_rand")]
                                rand,
                            )?;
                            d *= &stack[len - inputs + i].diff_ref().derivative;
                            stack[len - inputs].diff_mut().derivative += d;
                        }
                        inner_vars.truncate(end);
                        stack.truncate(len - inputs + 1);
                    }
                    &Token::Function(fun, d) => {
                        if d.get() != 0 {
                            todo!()
                        }
                        let derivative = Derivative::try_from(fun).ok()?;
                        match fun.inputs().get() {
                            1 => {
                                let g = stack.last_mut().unwrap().diff_mut();
                                let mut f_g = g.value.clone();
                                derivative.compute_on_1(&mut f_g).ok()?;
                                fun.compute_on_1(&mut g.value);
                                g.derivative *= f_g;
                            }
                            2 => {
                                let h = stack.pop().unwrap().diff();
                                let g = stack.last_mut().unwrap().diff_mut();
                                let mut d1 = g.value.clone();
                                let mut d2 = g.value.clone();
                                derivative.compute_on_2::<0>(&mut d1, &h.value).ok()?;
                                derivative.compute_on_2::<1>(&mut d2, &h.value).ok()?;
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
                    &Token::CustomVar(n) => stack.push(
                        DiffToken::new(
                            self.custom_vars[n as usize].value.clone(),
                            Number::default(),
                        )
                        .into(),
                    ),
                    &Token::GraphVar(n) => stack.push(
                        DiffToken::new(self.graph_vars[n as usize].clone(), Number::default())
                            .into(),
                    ),
                    &Token::Skip(to) => {
                        stack.push(StackToken::Skip(i + 1));
                        tokens.advance_by(to).unwrap();
                    }
                    Token::Number(n) => {
                        stack.push(DiffToken::new(n.clone(), Number::default()).into())
                    }
                }
            }
            stack.pop().unwrap().diff().derivative
        };
        if let Some(res) = res {
            Some(res)
        } else {
            stack.truncate(stack_end);
            None
        }
    }
}
