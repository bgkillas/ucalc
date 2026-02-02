use crate::parse::{Token, TokensRef};
use crate::{Functions, Tokens};
use ucalc_numbers::{Complex, Constant};
//TODO
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
        while i > 1 {
            i -= 1;
            match self[i] {
                Token::Operator(operator) if let Some(inv) = operator.inverse() => {
                    if operator.inputs() == 2 {
                        let last = TokensRef(&self[..i]).get_last(funs);
                        let num = TokensRef(&self[last..i])
                            .compute_buffer_with(fun_vars, vars, funs, &mut inner_stack, offset)
                            .into();
                        i = last;
                        inv.compute_on(&mut ret, &[num]);
                    } else {
                        inv.compute_on(&mut ret, &[]);
                    }
                }
                _ => return Complex::from(Constant::Nan),
            }
        }
        ret
    }
}
