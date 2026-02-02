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
        let mut stack = Tokens(Vec::with_capacity(self.len()));
        let mut inner_stack = Tokens(Vec::with_capacity(self.len()));
        while i > 1 {
            i -= 1;
            match self[i] {
                Token::Operator(operator) if let Some(inv) = operator.inverse() => {
                    stack.extend((0..operator.inputs() - 1).map(|_| Token::Skip(0)));
                    for k in 0..operator.inputs() - 1 {
                        let last = TokensRef(&self[..i]).get_last(funs);
                        stack[operator.inputs() - (k + 2)] = TokensRef(&self[last..i])
                            .compute_buffer_with(fun_vars, vars, funs, &mut inner_stack, offset)
                            .into();
                        i = last;
                    }
                    inv.compute_on(&mut ret, &stack);
                    stack.clear();
                }
                _ => return Complex::from(Constant::Nan),
            }
        }
        ret
    }
}
