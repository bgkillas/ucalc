use crate::Functions;
use crate::functions::Function;
use crate::operators::Operators;
use crate::parse::{Token, Tokens};
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
    pub fn get_skip_tokens<const N: usize>(&self) -> ([&[Token]; N], usize) {
        let len = self.len();
        let mut t = len - 1;
        let mut l = 0;
        let ret = array::from_fn(|_| {
            let back = self[t].skip();
            let ret = &self[back..t];
            l += t - back + 1;
            t = back - 1;
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
            let ret = inner(tokens, fun_vars, vars, funs);
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
        inner(self.as_slice(), fun_vars, vars, funs)
    }
}
fn inner(
    tokens: &[Token],
    fun_vars: &mut Vec<Complex>,
    vars: &[Complex],
    funs: &Functions,
) -> Complex {
    let mut stack = Tokens(Vec::with_capacity(tokens.len()));
    let mut b = Vec::with_capacity(
        iter::once(Operators::MAX_INPUT - 1)
            .chain(funs.iter().map(|f| f.inputs))
            .max()
            .unwrap(),
    );
    let mut i = 0;
    while i < tokens.len() {
        let len = stack.len();
        match &tokens[i] {
            Token::Operator(operator) => {
                let inputs = operator.inputs();
                match operator {
                    Operators::Function(Function::Sum) => {
                        stack.range(fun_vars, vars, funs, |iter| iter.sum::<Complex>().into());
                    }
                    Operators::Function(Function::Prod) => {
                        stack.range(fun_vars, vars, funs, |iter| {
                            iter.product::<Complex>().into()
                        });
                    }
                    Operators::Function(Function::Fold) => {
                        let ([tokens], l) = stack.get_skip_tokens();
                        let [end, start, value] = stack.get_skip_var(l);
                        let start = start.num_ref().real.to_usize();
                        let end = end.num_ref().real.to_usize();
                        fun_vars.push(value.num_ref());
                        fun_vars.push(Complex::from(start));
                        let nl = fun_vars.len();
                        (start..=end).for_each(|_| {
                            fun_vars[nl - 2] = inner(tokens, fun_vars, vars, funs);
                            fun_vars.last_mut().unwrap().real += Float::from(1);
                        });
                        *stack[len - (l + 3)].num_mut() = fun_vars[nl - 2];
                        stack.drain(len - (l + 2)..);
                        fun_vars.pop();
                        fun_vars.pop();
                    }
                    Operators::Function(Function::Set) => {
                        let ([tokens], l) = stack.get_skip_tokens();
                        let [value] = stack.get_skip_var(l);
                        fun_vars.push(value.num_ref());
                        *stack[len - (l + 1)].num_mut() = inner(tokens, fun_vars, vars, funs);
                        stack.drain(len - l..);
                        fun_vars.pop();
                    }
                    Operators::Function(Function::Iter) => {
                        let ([tokens], l) = stack.get_skip_tokens();
                        let [steps, first] = stack.get_skip_var(l);
                        fun_vars.push(first.num_ref());
                        let steps = steps.num_ref().real.to_usize();
                        (0..steps).for_each(|_| {
                            let next = inner(tokens, fun_vars, vars, funs);
                            *fun_vars.last_mut().unwrap() = next;
                        });
                        *stack[len - (l + 2)].num_mut() = *fun_vars.last().unwrap();
                        stack.drain(len - (l + 1)..);
                        fun_vars.pop();
                    }
                    Operators::Function(Function::If) => {
                        let ([ifelse, ifthen], l) = stack.get_skip_tokens();
                        let [condition] = stack.get_skip_var(l);
                        let condition = condition.num_ref();
                        let tokens = if condition.is_zero() { ifelse } else { ifthen };
                        *stack[len - (l + 1)].num_mut() = inner(tokens, fun_vars, vars, funs);
                        stack.drain(len - l..);
                    }
                    _ if operator.is_chainable() => {
                        let chain = if tokens.get(i + 1).is_some_and(|o| {
                            if let Token::Operator(o) = o {
                                o.is_chainable()
                            } else {
                                false
                            }
                        }) {
                            tokens.get(len - inputs).map(|n| n.num_ref())
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
                stack.extend_from_slice(&tokens[i + 1..=i + *to]);
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
