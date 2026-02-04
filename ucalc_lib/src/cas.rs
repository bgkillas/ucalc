use crate::inverse::Inverse;
use crate::parse::{Token, TokensRef};
use crate::{Functions, Tokens};
use ucalc_numbers::Complex;
impl<'a> TokensRef<'a> {
    pub fn get_inverse(
        &'a self,
        fun_vars: &mut Vec<Complex>,
        vars: &[Complex],
        funs: &Functions,
        offset: usize,
    ) -> Option<Vec<Complex>> {
        let mut i = self.len();
        let mut ret = Complex::from(0);
        let mut inner_stack = Tokens(Vec::with_capacity(self.len()));
        let mut start = 0;
        while i > start + 1 {
            i -= 1;
            match self[i] {
                Token::Operator(operator) => {
                    let inverse = Inverse::from(operator);
                    if inverse.is_none() {
                        return None;
                    }
                    if let Some(inv) = inverse.get_inverse() {
                        inv.compute_on(&mut ret, &[]);
                    } else {
                        let right_tokens = TokensRef(&self[start..i]);
                        let (right_tokens, last) = right_tokens.get_from_last(funs);
                        if right_tokens.contains(&Token::InnerVar(fun_vars.len())) {
                            let left_tokens = TokensRef(&self[start..last]);
                            let (left_tokens, _) = left_tokens.get_from_last(funs);
                            if left_tokens.contains(&Token::InnerVar(fun_vars.len())) {
                                let poly = TokensRef(&self[start..=i]).compute_polynomial(
                                    fun_vars,
                                    vars,
                                    funs,
                                    &mut inner_stack,
                                    offset,
                                    fun_vars.len(),
                                )? - ret;
                                return poly.roots();
                            } else {
                                let num = left_tokens.compute_buffer_with(
                                    fun_vars,
                                    vars,
                                    funs,
                                    &mut inner_stack,
                                    offset,
                                );
                                start = last;
                                ret = inverse.right_inverse(ret, num)[0];
                            }
                        } else {
                            let num = right_tokens.compute_buffer_with(
                                fun_vars,
                                vars,
                                funs,
                                &mut inner_stack,
                                offset,
                            );
                            i = last;
                            ret = inverse.left_inverse(ret, num)[0];
                        }
                    }
                }
                _ => return None,
            }
        }
        //TODO
        Some(vec![ret])
    }
}
