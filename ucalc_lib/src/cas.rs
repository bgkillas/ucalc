use crate::inverse::Inverse;
use crate::tokens::{Token, TokensRef};
use crate::{Functions, Number, Tokens, Variables};
use std::ops::Deref;
impl<'a> TokensRef<'a> {
    pub fn get_inverse(
        &'a self,
        fun_vars: &mut Vec<Number>,
        vars: &[Number],
        funs: &Functions,
        custom_vars: &Variables,
        offset: usize,
    ) -> Option<Number> {
        let mut ret = Number::from(0);
        let mut inner_stack = Tokens(Vec::with_capacity(self.len()));
        let inner = self.inner(
            fun_vars,
            vars,
            funs,
            custom_vars,
            offset,
            &mut ret,
            &mut inner_stack,
            None,
        )?;
        Some(if let Some(inner) = inner { inner } else { ret })
    }
    #[allow(clippy::too_many_arguments)]
    fn inner(
        &'a self,
        fun_vars: &mut Vec<Number>,
        vars: &[Number],
        funs: &Functions,
        custom_vars: &Variables,
        offset: usize,
        ret: &mut Number,
        inner_stack: &mut Tokens,
        args: Option<&mut Vec<TokensRef>>,
    ) -> Option<Option<Number>> {
        let mut i = self.len();
        let mut start = 0;
        while i > start + 1 {
            i -= 1;
            match self[i] {
                Token::Fun(n) => {
                    let fun = &funs[n];
                    let tokens = TokensRef(&self[start..=i]);
                    let mut args = tokens.get_lasts(funs);
                    let count = args
                        .iter()
                        .filter(|a| a.contains(&Token::InnerVar(fun_vars.len())))
                        .count();
                    if count != 1 {
                        todo!() //polynomial
                    }
                    let end = fun_vars.len();
                    for arg in args.iter() {
                        if arg.contains(&Token::InnerVar(fun_vars.len())) {
                            fun_vars.push(Number::default())
                        } else {
                            let n = arg.compute_buffer_with(
                                fun_vars,
                                vars,
                                funs,
                                custom_vars,
                                inner_stack,
                                offset,
                            );
                            fun_vars.push(n)
                        }
                    }
                    let roots = TokensRef(&fun.tokens).inner(
                        fun_vars,
                        vars,
                        funs,
                        custom_vars,
                        end,
                        ret,
                        inner_stack,
                        Some(&mut args),
                    )?;
                    if let Some(n) = roots {
                        *ret = n;
                    }
                    fun_vars.drain(end..);
                    return args[0].inner(
                        fun_vars,
                        vars,
                        funs,
                        custom_vars,
                        offset,
                        ret,
                        inner_stack,
                        None,
                    );
                }
                Token::Function(operator) => {
                    let inverse = Inverse::from(operator);
                    if inverse.is_none() {
                        return None;
                    }
                    if let Some(inv) = inverse.get_inverse() {
                        inv.compute_on(ret, &[]);
                    } else {
                        let right_tokens = TokensRef(&self[start..i]);
                        let (right_tokens, last) = right_tokens.get_from_last(funs);
                        if args
                            .as_ref()
                            .map(|a| {
                                a.iter().enumerate().any(|(i, a)| {
                                    right_tokens.contains(&Token::InnerVar(i))
                                        && a.contains(&Token::InnerVar(offset))
                                })
                            })
                            .unwrap_or(right_tokens.contains(&Token::InnerVar(fun_vars.len())))
                        {
                            let left_tokens = TokensRef(&self[start..last]);
                            let (left_tokens, _) = left_tokens.get_from_last(funs);
                            if args
                                .as_ref()
                                .map(|a| {
                                    a.iter().enumerate().any(|(i, a)| {
                                        left_tokens.contains(&Token::InnerVar(i))
                                            && a.contains(&Token::InnerVar(offset))
                                    })
                                })
                                .unwrap_or(left_tokens.contains(&Token::InnerVar(fun_vars.len())))
                            {
                                let poly = TokensRef(&self[start..=i]).compute_polynomial(
                                    fun_vars,
                                    vars,
                                    funs,
                                    custom_vars,
                                    inner_stack,
                                    offset,
                                    Some(
                                        args.and_then(|a| {
                                            a.iter()
                                                .position(|a| a.contains(&Token::InnerVar(offset)))
                                        })
                                        .unwrap_or(fun_vars.len()),
                                    ),
                                )?;
                                let poly = *poly.poly() - ret.deref();
                                let roots = poly.roots();
                                return Some(roots);
                            } else {
                                let num = left_tokens.compute_buffer_with(
                                    fun_vars,
                                    vars,
                                    funs,
                                    custom_vars,
                                    inner_stack,
                                    offset,
                                );
                                start = last;
                                inverse.right_inverse(ret, num);
                            }
                        } else {
                            let num = right_tokens.compute_buffer_with(
                                fun_vars,
                                vars,
                                funs,
                                custom_vars,
                                inner_stack,
                                offset,
                            );
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
