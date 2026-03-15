use crate::compute::Compute;
use crate::inverse::Inverse;
use crate::parse::{Token, TokensRef};
use crate::{Number, Tokens};
use std::ops::Deref;
impl Compute<'_> {
    pub(crate) fn get_inverse(self, fun_vars: &mut Vec<Number>) -> Option<Number> {
        let mut ret = Number::from(0);
        let mut inner_stack = Tokens(Vec::with_capacity(self.tokens.len()));
        Some(
            if let Some(inner) = self.cas_inner(fun_vars, &mut ret, &mut inner_stack, None)? {
                inner
            } else {
                ret
            },
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn cas_inner(
        self,
        fun_vars: &mut Vec<Number>,
        ret: &mut Number,
        inner_stack: &mut Tokens,
        args: Option<&mut Vec<TokensRef>>,
    ) -> Option<Option<Number>> {
        let mut i = self.tokens.len();
        let mut start = 0;
        while i > start + 1 {
            i -= 1;
            match self.tokens[i] {
                Token::Fun(n) => {
                    let fun = &self.funs[n as usize];
                    let tokens = TokensRef(&self.tokens[start..=i]);
                    let mut args = tokens.get_lasts(self.funs);
                    let count = args
                        .iter()
                        .filter(|a| a.contains(&Token::InnerVar(fun_vars.len() as u16)))
                        .count();
                    if count != 1 {
                        todo!() //polynomial
                    }
                    let end = fun_vars.len();
                    for arg in args.iter().copied() {
                        if arg.contains(&Token::InnerVar(fun_vars.len() as u16)) {
                            fun_vars.push(Number::default())
                        } else {
                            let n = self.tokens(arg).compute_buffer_with(fun_vars, inner_stack);
                            fun_vars.push(n)
                        }
                    }
                    let roots = self.tokens(TokensRef(&fun.tokens)).offset(end).cas_inner(
                        fun_vars,
                        ret,
                        inner_stack,
                        Some(&mut args),
                    )?;
                    if let Some(n) = roots {
                        *ret = n;
                    }
                    fun_vars.drain(end..);
                    return self
                        .tokens(args[0])
                        .cas_inner(fun_vars, ret, inner_stack, None);
                }
                Token::Function(operator) => {
                    let inverse = Inverse::from(operator);
                    if inverse.is_none() {
                        return None;
                    }
                    if let Some(inv) = inverse.get_inverse() {
                        inv.compute_on(ret, &[]);
                    } else {
                        let right_tokens = TokensRef(&self.tokens[start..i]);
                        let (right_tokens, last) = right_tokens.get_from_last(self.funs);
                        if args
                            .as_ref()
                            .map(|a| {
                                a.iter().enumerate().any(|(i, a)| {
                                    right_tokens.contains(&Token::InnerVar(i as u16))
                                        && a.contains(&Token::InnerVar(self.offset as u16))
                                })
                            })
                            .unwrap_or_else(|| {
                                right_tokens.contains(&Token::InnerVar(fun_vars.len() as u16))
                            })
                        {
                            let left_tokens = TokensRef(&self.tokens[start..last]);
                            let (left_tokens, _) = left_tokens.get_from_last(self.funs);
                            if args
                                .as_ref()
                                .map(|a| {
                                    a.iter().enumerate().any(|(i, a)| {
                                        left_tokens.contains(&Token::InnerVar(i as u16))
                                            && a.contains(&Token::InnerVar(self.offset as u16))
                                    })
                                })
                                .unwrap_or_else(|| {
                                    left_tokens.contains(&Token::InnerVar(fun_vars.len() as u16))
                                })
                            {
                                let poly = self
                                    .tokens(TokensRef(&self.tokens[start..=i]))
                                    .compute_polynomial(
                                        fun_vars,
                                        inner_stack,
                                        Some(
                                            args.and_then(|a| {
                                                a.iter().position(|a| {
                                                    a.contains(&Token::InnerVar(self.offset as u16))
                                                })
                                            })
                                            .unwrap_or(fun_vars.len())
                                                as u16,
                                        ),
                                    )?;
                                let poly = *poly.poly() - ret.deref();
                                let roots = poly.roots();
                                return Some(roots);
                            } else {
                                let num = self
                                    .tokens(left_tokens)
                                    .compute_buffer_with(fun_vars, inner_stack);
                                start = last;
                                inverse.right_inverse(ret, num);
                            }
                        } else {
                            let num = self
                                .tokens(right_tokens)
                                .compute_buffer_with(fun_vars, inner_stack);
                            i = last;
                            inverse.left_inverse(ret, num);
                        }
                    }
                }
                _ => return None,
            }
        }
        Some(None)
    }
}
