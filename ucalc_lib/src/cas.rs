use crate::inverse::Inverse;
use crate::parse::{Token, TokensRef};
use crate::{Functions, Tokens};
use ucalc_numbers::{Complex, Constant};
impl TokensRef<'_> {
    pub fn get_inverse(
        &self,
        fun_vars: &mut Vec<Complex>,
        vars: &[Complex],
        funs: &Functions,
        offset: usize,
    ) -> Complex {
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
                        return Complex::from(Constant::Nan);
                    }
                    if let Some(inv) = inverse.get_inverse() {
                        inv.compute_on(&mut ret, &[]);
                    } else {
                        let (right_tokens, last) = TokensRef(&self[..i]).get_from_last(funs);
                        if right_tokens.contains(&Token::InnerVar(fun_vars.len())) {
                            let (left_tokens, _) = TokensRef(&self[..last]).get_from_last(funs);
                            if left_tokens.contains(&Token::InnerVar(fun_vars.len())) {
                                todo!()
                            } else {
                                let num = left_tokens.compute_buffer_with(
                                    fun_vars,
                                    vars,
                                    funs,
                                    &mut inner_stack,
                                    offset,
                                );
                                start = last;
                                inverse.right_inverse(&mut ret, num);
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
                            inverse.left_inverse(&mut ret, num);
                        }
                    }
                }
                _ => return Complex::from(Constant::Nan),
            }
        }
        ret
    }
}
