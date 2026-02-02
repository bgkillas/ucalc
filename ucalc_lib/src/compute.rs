use crate::functions::Function;
use crate::operators::Operators;
use crate::parse::{Parsed, Token};
use crate::{Functions, InnerVariable, InnerVariables, Variables};
use ucalc_numbers::{Complex, Constant, Float};
impl Parsed {
    pub fn clone_compute(&mut self, vars: &Variables, funs: &Functions) -> Complex {
        let parsed = self.parsed.clone();
        let ret = self.compute(vars, funs);
        self.parsed = parsed;
        ret
    }
    pub fn clone_compute_inner(
        &mut self,
        fun_vars: Option<&InnerVariables>,
        vars: &Variables,
        funs: &Functions,
    ) -> Complex {
        let parsed = self.parsed.clone();
        let ret = self.compute_inner(fun_vars, vars, funs);
        self.parsed = parsed;
        ret
    }
    pub fn compute(&mut self, vars: &Variables, funs: &Functions) -> Complex {
        self.compute_inner(None, vars, funs)
    }
    pub fn range(
        &mut self,
        i: usize,
        fun_vars: Option<&InnerVariables>,
        vars: &Variables,
        funs: &Functions,
        fun: impl FnOnce(&mut dyn Iterator<Item = Complex>) -> Token,
    ) {
        let end = self.parsed.remove(i - 2).num().real.to_usize();
        let tokens = self.parsed.remove(i - 2).tokens();
        let first = self.parsed.get_mut(i - 3).unwrap();
        let start = first.clone().num().real.to_usize();
        let mut parsed = Parsed { parsed: tokens };
        let mut new_vars = Vec::with_capacity(fun_vars.map(|v| v.len()).unwrap_or(0) + 1);
        if let Some(slice) = fun_vars {
            new_vars.extend_from_slice(slice)
        }
        new_vars.push(InnerVariable::new(Complex::from(start)));
        let mut new_vars = InnerVariables(new_vars);
        let mut iter = (start..=end).map(|_| {
            let ret = parsed.clone_compute_inner(Some(&new_vars), vars, funs);
            new_vars.last_mut().unwrap().value.real += Float::from(1);
            ret
        });
        *first = fun(&mut iter);
    }
    pub fn compute_inner(
        &mut self,
        fun_vars: Option<&InnerVariables>,
        vars: &Variables,
        funs: &Functions,
    ) -> Complex {
        let mut b = Vec::with_capacity(Operators::MAX_INPUT - 1);
        let mut i = 0;
        while i < self.parsed.len() {
            match self.parsed[i] {
                Token::Operator(operator) => {
                    self.parsed.remove(i);
                    let inputs = operator.inputs();
                    match operator {
                        Operators::Function(Function::Sum) => {
                            self.range(i, fun_vars, vars, funs, |iter| {
                                iter.sum::<Complex>().into()
                            });
                        }
                        Operators::Function(Function::Prod) => {
                            self.range(i, fun_vars, vars, funs, |iter| {
                                iter.product::<Complex>().into()
                            });
                        }
                        Operators::Function(Function::Iter) => {
                            let steps = self.parsed.remove(i - 2).num().real.to_usize();
                            let tokens = self.parsed.remove(i - 2).tokens();
                            let first = self.parsed.get_mut(i - 3).unwrap();
                            let start = first.clone().num();
                            let mut parsed = Parsed { parsed: tokens };
                            let mut inner_vars =
                                Vec::with_capacity(fun_vars.map(|v| v.len()).unwrap_or(0) + 1);
                            if let Some(slice) = fun_vars {
                                inner_vars.extend_from_slice(slice)
                            }
                            inner_vars.push(InnerVariable::new(Complex::from(start)));
                            let mut new_vars = InnerVariables(inner_vars);
                            (0..steps).for_each(|_| {
                                let next = parsed.clone_compute_inner(Some(&new_vars), vars, funs);
                                new_vars.last_mut().unwrap().value = next;
                            });
                            *first.num_mut() = new_vars.last_mut().unwrap().value;
                        }
                        _ => {
                            let chain = if operator.is_chainable()
                                && self.parsed.get(i).is_some_and(|o| {
                                    if let Token::Operator(o) = o {
                                        o.is_chainable()
                                    } else {
                                        false
                                    }
                                }) {
                                self.parsed.get(i - inputs).map(|n| n.clone().num())
                            } else {
                                None
                            };
                            b.extend(self.parsed.drain(i + 1 - inputs..i).map(|a| a.num()));
                            let a = self.parsed.get_mut(i - inputs).unwrap().num_mut();
                            operator.compute(a, &b);
                            if let Some(b) = chain {
                                *a = if a.is_zero() {
                                    Complex::from(Constant::Nan)
                                } else {
                                    b
                                };
                            }
                            b.clear();
                        }
                    }
                    i -= inputs - 1;
                }
                Token::InnerVar(index) => {
                    self.parsed[i] = Token::Num(fun_vars.as_ref().unwrap()[index].value);
                    i += 1
                }
                Token::Var(index) => {
                    self.parsed[i] = Token::Num(vars[index].value);
                    i += 1
                }
                Token::Fun(index) => {
                    self.parsed.remove(i);
                    let inputs = funs[index].vars.len();
                    let mut inner = funs[index].vars.clone();
                    for (a, b) in inner[1..]
                        .iter_mut()
                        .zip(self.parsed.drain(i + 1 - inputs..i).map(|a| a.num()))
                    {
                        a.value = b;
                    }
                    let a = self.parsed.get_mut(i - inputs).unwrap().num_mut();
                    inner[0].value = *a;
                    *a = Parsed {
                        parsed: funs[index].tokens.clone(),
                    }
                    .compute_inner(Some(&inner), vars, funs);
                    i -= inputs - 1;
                }
                Token::Num(_) | Token::Tokens(_) => i += 1,
            }
        }
        self.parsed.remove(0).num()
    }
}
