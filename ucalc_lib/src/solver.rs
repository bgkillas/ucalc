use crate::Number;
use crate::compute::{Compute, StackToken};
use crate::inverse::Inverse;
use crate::parse::{Token, TokensSlice};
#[cfg(feature = "float_rand")]
use crate::rand::Rand;
use std::ops::Deref;
impl Compute<'_> {
    pub(crate) fn solve(
        &self,
        inner_vars: &mut Vec<Number>,
        stack: &mut Vec<StackToken>,
        #[cfg(feature = "float_rand")] rand: &mut Rand,
    ) -> Option<Number> {
        let mut ret = Number::from(0);
        Some(
            if let Some(inner) = self.solve_inner(
                inner_vars,
                &mut ret,
                stack,
                None,
                #[cfg(feature = "float_rand")]
                rand,
            )? {
                inner
            } else {
                ret
            },
        )
    }
    fn solve_inner(
        &self,
        inner_vars: &mut Vec<Number>,
        ret: &mut Number,
        stack: &mut Vec<StackToken>,
        args: Option<&mut Vec<&TokensSlice>>,
        #[cfg(feature = "float_rand")] rand: &mut Rand,
    ) -> Option<Option<Number>> {
        let mut tokens = self.tokens.iter().enumerate();
        let mut start = 0;
        while let Some((i, token)) = tokens.next_back() {
            match *token {
                Token::CustomFun(n, d) => {
                    if d.get() != 0 {
                        todo!()
                    }
                    let fun = &self.custom_funs[n as usize];
                    let tokens = &self.tokens[start..=i];
                    let mut args = tokens.get_lasts(self.custom_funs);
                    let count = args
                        .iter()
                        .filter(|a| a.contains(&Token::InnerVar(inner_vars.len() as u16)))
                        .count();
                    if count != 1 {
                        todo!() //polynomial
                    }
                    let end = inner_vars.len();
                    for arg in args.iter().copied() {
                        if arg.contains(&Token::InnerVar(inner_vars.len() as u16)) {
                            inner_vars.push(Number::default())
                        } else {
                            let n = self.tokens(arg).compute(
                                inner_vars,
                                stack,
                                #[cfg(feature = "float_rand")]
                                rand,
                            );
                            inner_vars.push(n)
                        }
                    }
                    let roots = self.tokens(&fun.tokens[..]).offset(end).solve_inner(
                        inner_vars,
                        ret,
                        stack,
                        Some(&mut args),
                        #[cfg(feature = "float_rand")]
                        rand,
                    )?;
                    if let Some(n) = roots {
                        *ret = n;
                    }
                    inner_vars.drain(end..);
                    return self.tokens(args[0]).solve_inner(
                        inner_vars,
                        ret,
                        stack,
                        None,
                        #[cfg(feature = "float_rand")]
                        rand,
                    );
                }
                Token::Function(fun, d) => {
                    if d.get() != 0 {
                        todo!()
                    }
                    let Ok(inverse) = Inverse::try_from(fun) else {
                        return None;
                    };
                    if let Some(inv) = inverse.get_inverse() {
                        inv.compute_on_1(ret);
                    } else {
                        let right_tokens = &self.tokens[start..i];
                        let (right_tokens, last) = right_tokens.get_from_last(self.custom_funs);
                        if args
                            .as_ref()
                            .map(|a| {
                                a.iter().enumerate().any(|(i, a)| {
                                    right_tokens.contains(&Token::InnerVar(i as u16))
                                        && a.contains(&Token::InnerVar(self.offset as u16))
                                })
                            })
                            .unwrap_or_else(|| {
                                right_tokens.contains(&Token::InnerVar(inner_vars.len() as u16))
                            })
                        {
                            let left_tokens = &self.tokens[start..start + last];
                            let (left_tokens, _) = left_tokens.get_from_last(self.custom_funs);
                            if args
                                .as_ref()
                                .map(|a| {
                                    a.iter().enumerate().any(|(i, a)| {
                                        left_tokens.contains(&Token::InnerVar(i as u16))
                                            && a.contains(&Token::InnerVar(self.offset as u16))
                                    })
                                })
                                .unwrap_or_else(|| {
                                    left_tokens.contains(&Token::InnerVar(inner_vars.len() as u16))
                                })
                            {
                                let poly =
                                    self.tokens(&self.tokens[start..=i]).compute_polynomial(
                                        inner_vars,
                                        stack,
                                        Some(
                                            args.and_then(|a| {
                                                a.iter().position(|a| {
                                                    a.contains(&Token::InnerVar(self.offset as u16))
                                                })
                                            })
                                            .unwrap_or(inner_vars.len())
                                                as u16,
                                        ),
                                        #[cfg(feature = "float_rand")]
                                        rand,
                                    )?;
                                let poly = poly.poly() - ret.deref();
                                let roots = poly.roots();
                                return Some(roots);
                            } else {
                                let num = self.tokens(left_tokens).compute(
                                    inner_vars,
                                    stack,
                                    #[cfg(feature = "float_rand")]
                                    rand,
                                );
                                tokens.advance_by(last).unwrap();
                                start += last;
                                inverse.right_inverse(ret, num);
                            }
                        } else {
                            let num = self.tokens(right_tokens).compute(
                                inner_vars,
                                stack,
                                #[cfg(feature = "float_rand")]
                                rand,
                            );
                            tokens.advance_back_by(i - (start + last)).unwrap();
                            inverse.left_inverse(ret, num);
                        }
                    }
                }
                Token::InnerVar(_) => return Some(None),
                _ => return None,
            }
        }
        Some(None)
    }
}
