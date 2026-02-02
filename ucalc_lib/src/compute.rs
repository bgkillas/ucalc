use crate::Functions;
use crate::operators::Operators;
use crate::parse::{Token, Tokens, TokensRef};
use std::{array, iter};
use ucalc_numbers::{Complex, Constant, Float};
impl Tokens {
    pub fn compute(&self, vars: &[Complex], funs: &Functions) -> Complex {
        self.compute_inner(&mut Vec::with_capacity(self.len()), vars, funs)
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
        fun: impl FnOnce(&mut dyn Iterator<Item = Complex>) -> Token,
    ) {
        let len = self.len();
        let ([tokens], l) = self.get_skip_tokens();
        let [end, start] = self.get_skip_var(l);
        let start = start.num_ref().real.to_usize();
        let end = end.num_ref().real.to_usize();
        fun_vars.push(Complex::from(start));
        let mut iter = (start..=end).map(|_| {
            let ret = tokens.compute(fun_vars, vars, funs);
            fun_vars.last_mut().unwrap().real += Float::from(1);
            ret
        });
        self[len - (l + 2)] = fun(&mut iter);
        fun_vars.pop();
        self.drain(len - (l + 1)..);
    }
    pub fn compute_inner(
        &self,
        fun_vars: &mut Vec<Complex>,
        vars: &[Complex],
        funs: &Functions,
    ) -> Complex {
        TokensRef(self.as_slice()).compute(fun_vars, vars, funs)
    }
}
impl TokensRef<'_> {
    pub fn compute(
        &self,
        fun_vars: &mut Vec<Complex>,
        vars: &[Complex],
        funs: &Functions,
    ) -> Complex {
        let mut stack = Tokens(Vec::with_capacity(self.len()));
        let mut b = Vec::with_capacity(
            iter::once(Operators::MAX_INPUT - 1)
                .chain(funs.iter().map(|f| f.inputs))
                .max()
                .unwrap(),
        );
        let mut i = 0;
        while i < self.len() {
            let len = stack.len();
            match &self[i] {
                Token::Operator(operator) => {
                    let inputs = operator.inputs();
                    match operator {
                        Operators::Function(fun) if fun.has_var() => {
                            fun.compute_var(&mut stack, fun_vars, vars, funs)
                        }
                        _ if operator.is_chainable() => {
                            let chain = if self.get(i + 1).is_some_and(|o| {
                                if let Token::Operator(o) = o {
                                    o.is_chainable()
                                } else {
                                    false
                                }
                            }) {
                                self.get(len - inputs).map(|n| n.num_ref())
                            } else {
                                None
                            };
                            b.extend(stack.drain(len + 1 - inputs..).map(|t| t.num()));
                            let a = stack.get_mut(len - inputs).unwrap().num_mut();
                            operator.compute(a, &b);
                            b.clear();
                            if let Some(b) = chain {
                                *a = if a.is_zero() {
                                    Complex::from(Constant::Nan)
                                } else {
                                    b
                                };
                            }
                        }
                        _ => {
                            b.extend(stack.drain(len + 1 - inputs..).map(|t| t.num()));
                            let a = stack.get_mut(len - inputs).unwrap().num_mut();
                            operator.compute(a, &b);
                            b.clear()
                        }
                    }
                }
                Token::Fun(index) => {
                    let inputs = funs[*index].inputs;
                    b.push(stack.get(len - inputs).unwrap().num_ref());
                    b.extend(stack.drain(len + 1 - inputs..).map(|a| a.num()));
                    let a = stack.get_mut(len - inputs).unwrap().num_mut();
                    *a = funs[*index].tokens.compute_inner(&mut b, vars, funs);
                    b.clear();
                }
                Token::InnerVar(index) => {
                    stack.push(Token::Num(fun_vars[*index]));
                }
                Token::GraphVar(index) => {
                    stack.push(Token::Num(vars[*index]));
                }
                Token::Skip(to) => {
                    let back = stack.len();
                    stack.extend_from_slice(&self[i + 1..=i + *to]);
                    stack.push(Token::Skip(back));
                    i += *to;
                }
                Token::Num(n) => {
                    stack.push(Token::Num(*n));
                }
            }
            i += 1;
        }
        stack.pop().unwrap().num()
    }
}
