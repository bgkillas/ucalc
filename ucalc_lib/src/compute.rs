use crate::functions::Function;
use crate::operators::Operators;
use crate::parse::{Token, Tokens};
use crate::{Functions, Variables};
use std::iter;
use ucalc_numbers::{Complex, Constant, Float};
impl Tokens {
    pub fn compute(&self, vars: &Variables, funs: &Functions) -> Complex {
        self.compute_inner(None, vars, funs)
    }
    pub fn range(
        &mut self,
        fun_vars: Option<&[Complex]>,
        vars: &Variables,
        funs: &Functions,
        fun: impl FnOnce(&mut dyn Iterator<Item = Complex>) -> Token,
    ) {
        let len = self.len();
        let end = self.remove(len - 2).num().real.to_usize();
        let tokens = self.remove(len - 2).tokens();
        let first = self.get_mut(len - 3).unwrap();
        let start = first.num_ref().real.to_usize();
        let mut new_vars = Vec::with_capacity(fun_vars.map(|v| v.len()).unwrap_or(0) + 1);
        if let Some(slice) = fun_vars {
            new_vars.extend_from_slice(slice)
        }
        new_vars.push(Complex::from(start));
        let mut iter = (start..=end).map(|_| {
            let ret = tokens.compute_inner(Some(&new_vars), vars, funs);
            new_vars.last_mut().unwrap().real += Float::from(1);
            ret
        });
        *first = fun(&mut iter);
    }
    pub fn compute_inner(
        &self,
        fun_vars: Option<&[Complex]>,
        vars: &Variables,
        funs: &Functions,
    ) -> Complex {
        let mut stack = Tokens(Vec::with_capacity(self.len()));
        let mut b = Vec::with_capacity(
            iter::once(Operators::MAX_INPUT - 1)
                .chain(funs.iter().map(|f| f.inputs))
                .max()
                .unwrap(),
        );
        for (i, op) in self.iter().enumerate() {
            let len = stack.len();
            match op {
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
                            let start = stack.remove(len - 3).num().real.to_usize();
                            let end = stack.remove(len - 3).num().real.to_usize();
                            let tokens = stack.remove(len - 3).tokens();
                            let value = stack.get_mut(len - 4).unwrap().num_mut();
                            let mut new_vars =
                                Vec::with_capacity(fun_vars.map(|v| v.len()).unwrap_or(0) + 1);
                            if let Some(slice) = fun_vars {
                                new_vars.extend_from_slice(slice)
                            }
                            new_vars.push(*value);
                            new_vars.push(Complex::from(start));
                            let len = new_vars.len();
                            (start..=end).for_each(|_| {
                                *new_vars.get_mut(len - 2).unwrap() =
                                    tokens.compute_inner(Some(&new_vars), vars, funs);
                                new_vars.last_mut().unwrap().real += Float::from(1);
                            });
                            *value = *new_vars.get(len - 2).unwrap();
                        }
                        Operators::Function(Function::Set) => {
                            let tokens = stack.remove(len - 1).tokens();
                            let value = stack.get_mut(len - 2).unwrap().num_mut();
                            let mut new_vars =
                                Vec::with_capacity(fun_vars.map(|v| v.len()).unwrap_or(0) + 1);
                            if let Some(slice) = fun_vars {
                                new_vars.extend_from_slice(slice)
                            }
                            new_vars.push(*value);
                            *value = tokens.compute_inner(Some(&new_vars), vars, funs);
                        }
                        Operators::Function(Function::Iter) => {
                            let steps = stack.remove(len - 2).num().real.to_usize();
                            let tokens = stack.remove(len - 2).tokens();
                            let first = stack.get_mut(len - 3).unwrap().num_mut();
                            let mut inner_vars =
                                Vec::with_capacity(fun_vars.map(|v| v.len()).unwrap_or(0) + 1);
                            if let Some(slice) = fun_vars {
                                inner_vars.extend_from_slice(slice)
                            }
                            inner_vars.push(Complex::from(*first));
                            (0..steps).for_each(|_| {
                                let next = tokens.compute_inner(Some(&inner_vars), vars, funs);
                                *inner_vars.last_mut().unwrap() = next;
                            });
                            *first = *inner_vars.last().unwrap();
                        }
                        Operators::Function(Function::If) => {
                            let ifthen = stack.remove(len - 2).tokens();
                            let ifelse = stack.remove(len - 2).tokens();
                            let condition = stack.get_mut(len - 3).unwrap().num_mut();
                            let tokens = if condition.is_zero() { ifelse } else { ifthen };
                            *condition = tokens.compute_inner(fun_vars, vars, funs);
                        }
                        _ => {
                            let chain = if operator.is_chainable()
                                && self.get(i + 1).is_some_and(|o| {
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
                            if let Some(b) = chain {
                                *a = if a.is_zero() {
                                    Complex::from(Constant::Nan)
                                } else {
                                    b
                                };
                            }
                            b.clear()
                        }
                    }
                }
                Token::Fun(index) => {
                    let inputs = funs[*index].inputs;
                    b.push(stack.get(len - inputs).unwrap().num_ref());
                    b.extend(stack.drain(len + 1 - inputs..).map(|a| a.num()));
                    let a = stack.get_mut(len - inputs).unwrap().num_mut();
                    *a = funs[*index].tokens.compute_inner(Some(&b), vars, funs);
                    b.clear();
                }
                Token::InnerVar(index) => {
                    stack.push(Token::Num(fun_vars.as_ref().unwrap()[*index]));
                }
                Token::Var(index) => {
                    stack.push(Token::Num(vars[*index].value));
                }
                Token::Num(n) => {
                    stack.push(Token::Num(*n));
                }
                Token::Tokens(v) => {
                    stack.push(Token::Tokens(v.clone()));
                }
            }
        }
        stack.pop().unwrap().num()
    }
}
