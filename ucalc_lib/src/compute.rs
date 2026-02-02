use crate::Functions;
use crate::operators::Operators;
use crate::parse::{Token, Tokens, TokensRef};
use std::array;
use ucalc_numbers::{Complex, Constant, Float};
impl Tokens {
    pub fn compute(&self, vars: &[Complex], funs: &Functions) -> Complex {
        TokensRef(self).compute(vars, funs)
    }
    pub fn compute_buffer(
        &self,
        fun_vars: &mut Vec<Complex>,
        vars: &[Complex],
        funs: &Functions,
        stack: &mut Tokens,
    ) -> Complex {
        self.compute_buffer_with(fun_vars, vars, funs, stack, 0)
    }
    pub fn compute_buffer_with(
        &self,
        fun_vars: &mut Vec<Complex>,
        vars: &[Complex],
        funs: &Functions,
        stack: &mut Tokens,
        offset: usize,
    ) -> Complex {
        TokensRef(self).compute_buffer_with(fun_vars, vars, funs, stack, offset)
    }
    pub fn get_skip_var<const N: usize>(&self, end: usize) -> [&Token; N] {
        let end = self.len() - (end + 1);
        array::from_fn(|i| &self[end - i])
    }
    pub fn get_skip_tokens<const N: usize>(&self) -> ([TokensRef<'_>; N], usize) {
        let len = self.len();
        let mut t = len - 1;
        let mut l = 0;
        let ret = array::from_fn(|_| {
            let back = self[t].skip();
            let ret = TokensRef(&self[back..t]);
            l += t - back + 1;
            t = back.saturating_sub(1);
            ret
        });
        (ret, l)
    }
    pub fn range(
        &mut self,
        fun_vars: &mut Vec<Complex>,
        vars: &[Complex],
        funs: &Functions,
        offset: usize,
        fun: impl FnOnce(&mut dyn Iterator<Item = Complex>) -> Token,
    ) {
        let len = self.len();
        let ([tokens], l) = self.get_skip_tokens();
        let [end, start] = self.get_skip_var(l);
        let start = start.num_ref().real.to_usize();
        let end = end.num_ref().real.to_usize();
        fun_vars.push(Complex::from(start));
        let mut stack = Tokens(Vec::with_capacity(tokens.len()));
        let mut iter = (start..=end).map(|_| {
            let ret = tokens.compute_buffer_with(fun_vars, vars, funs, &mut stack, offset);
            fun_vars.last_mut().unwrap().real += Float::from(1);
            ret
        });
        self[len - (l + 2)] = fun(&mut iter);
        fun_vars.pop();
        self.drain(len - (l + 1)..);
    }
}
impl TokensRef<'_> {
    pub fn compute(&self, vars: &[Complex], funs: &Functions) -> Complex {
        let mut fun_vars = Vec::with_capacity(self.len());
        let mut stack = Tokens(Vec::with_capacity(self.len()));
        self.compute_buffer(&mut fun_vars, vars, funs, &mut stack)
    }
    pub fn compute_with(&self, vars: &[Complex], funs: &Functions, offset: usize) -> Complex {
        let mut fun_vars = Vec::with_capacity(self.len());
        let mut stack = Tokens(Vec::with_capacity(self.len()));
        self.compute_buffer_with(&mut fun_vars, vars, funs, &mut stack, offset)
    }
    pub fn compute_buffer(
        &self,
        fun_vars: &mut Vec<Complex>,
        vars: &[Complex],
        funs: &Functions,
        stack: &mut Tokens,
    ) -> Complex {
        self.compute_buffer_with(fun_vars, vars, funs, stack, 0)
    }
    pub fn compute_buffer_with(
        &self,
        fun_vars: &mut Vec<Complex>,
        vars: &[Complex],
        funs: &Functions,
        stack: &mut Tokens,
        offset: usize,
    ) -> Complex {
        let mut i = 0;
        while i < self.len() {
            let len = stack.len();
            match self[i] {
                Token::Operator(operator) => {
                    let inputs = operator.inputs();
                    match operator {
                        Operators::Function(fun) if fun.has_var() => {
                            fun.compute_var(stack, fun_vars, vars, funs, offset)
                        }
                        _ if operator.is_chainable() => {
                            let chain = if self.get(i + 1).is_some_and(|o| {
                                if let Token::Operator(o) = o {
                                    o.is_chainable()
                                } else {
                                    false
                                }
                            }) {
                                Some(self[len - inputs].num_ref())
                            } else {
                                None
                            };
                            operator.compute(&mut stack[len - inputs..]);
                            if let Some(b) = chain {
                                let a = stack[len - inputs].num_mut();
                                *a = if a.is_zero() {
                                    Complex::from(Constant::Nan)
                                } else {
                                    b
                                };
                            }
                            stack.drain(len + 1 - inputs..);
                        }
                        _ => {
                            operator.compute(&mut stack[len - inputs..]);
                            stack.drain(len + 1 - inputs..);
                        }
                    }
                }
                Token::Fun(index) => {
                    let inputs = funs[index].inputs;
                    let end = fun_vars.len();
                    fun_vars.push(stack[len - inputs].num_ref());
                    fun_vars.extend(stack.drain(len + 1 - inputs..).map(|n| n.num()));
                    *stack[len - inputs].num_mut() = funs[index].tokens.compute_buffer_with(
                        fun_vars,
                        vars,
                        funs,
                        &mut Tokens(Vec::with_capacity(funs[index].tokens.len())),
                        end,
                    );
                    fun_vars.drain(end..);
                }
                Token::InnerVar(index) => stack.push(Token::Num(fun_vars[offset + index])),
                Token::GraphVar(index) => stack.push(Token::Num(vars[index])),
                Token::Skip(to) => {
                    let back = stack.len();
                    stack.extend_from_slice(&self[i + 1..=i + to]);
                    stack.push(Token::Skip(back));
                    i += to;
                }
                Token::Num(n) => stack.push(Token::Num(n)),
            }
            i += 1;
        }
        stack.pop().unwrap().num()
    }
}
