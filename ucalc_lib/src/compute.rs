use crate::functions::Function;
use crate::operators::Operators;
use crate::parse::{Parsed, Token};
use crate::{InnerVariable, InnerVariables};
use ucalc_numbers::{Complex, Float};
impl Parsed {
    pub fn clone_compute(&mut self) -> Complex {
        let parsed = self.parsed.clone();
        let ret = self.compute();
        self.parsed = parsed;
        ret
    }
    pub fn clone_compute_inner(&mut self, vars: Option<&InnerVariables>) -> Complex {
        let parsed = self.parsed.clone();
        let ret = self.compute_inner(vars);
        self.parsed = parsed;
        ret
    }
    pub fn compute(&mut self) -> Complex {
        self.compute_inner(None)
    }
    pub fn range(
        &mut self,
        i: usize,
        old_vars: Option<&InnerVariables>,
        fun: impl FnOnce(&mut dyn Iterator<Item = Complex>) -> Token,
    ) {
        let end = self.parsed.remove(i - 2).num().real.to_usize();
        let tokens = self.parsed.remove(i - 2).tokens();
        let first = self.parsed.get_mut(i - 3).unwrap();
        let start = first.clone().num().real.to_usize();
        let mut parsed = Parsed {
            parsed: tokens,
            vars: self.vars.clone(),
            funs: self.funs.clone(),
        };
        let mut vars = Vec::with_capacity(old_vars.map(|v| v.len()).unwrap_or(0) + 1);
        if let Some(slice) = old_vars {
            vars.extend_from_slice(slice)
        }
        vars.push(InnerVariable::new(Complex::from(start)));
        let mut vars = InnerVariables(vars);
        let mut iter = (start..=end).map(|_| {
            let ret = parsed.clone_compute_inner(Some(&vars));
            vars.last_mut().unwrap().value.real += Float::from(1);
            ret
        });
        *first = fun(&mut iter);
    }
    pub fn compute_inner(&mut self, vars: Option<&InnerVariables>) -> Complex {
        let mut b = Vec::with_capacity(Operators::MAX_INPUT - 1);
        let mut i = 0;
        while i < self.parsed.len() {
            match self.parsed[i] {
                Token::Operator(operator) => {
                    self.parsed.remove(i);
                    let inputs = operator.inputs();
                    match operator {
                        Operators::Fun(Function::Sum) => {
                            self.range(i, vars, |iter| iter.sum::<Complex>().into());
                        }
                        Operators::Fun(Function::Prod) => {
                            self.range(i, vars, |iter| iter.product::<Complex>().into());
                        }
                        _ => {
                            b.extend(self.parsed.drain(i + 1 - inputs..i).map(|a| a.num()));
                            let a = self.parsed.get_mut(i - inputs).unwrap().num_mut();
                            operator.compute(a, &b);
                            b.clear();
                        }
                    }
                    i -= inputs - 1;
                }
                Token::InnerVar(index) => {
                    self.parsed[i] = Token::Num(vars.as_ref().unwrap()[index].value);
                    i += 1
                }
                Token::Var(index) => {
                    self.parsed[i] = Token::Num(self.vars[index].value);
                    i += 1
                }
                Token::Fun(index) => {
                    self.parsed.remove(i);
                    let inputs = self.funs[index].vars.len();
                    let mut inner = self.funs[index].vars.clone();
                    for (a, b) in inner[1..]
                        .iter_mut()
                        .zip(self.parsed.drain(i + 1 - inputs..i).map(|a| a.num()))
                    {
                        a.value = b;
                    }
                    let a = self.parsed.get_mut(i - inputs).unwrap().num_mut();
                    inner[0].value = *a;
                    *a = Parsed {
                        parsed: self.funs[index].tokens.clone(),
                        vars: self.vars.clone(),
                        funs: self.funs.clone(),
                    }
                    .compute_inner(Some(&inner));
                    i -= inputs - 1;
                }
                Token::Num(_) | Token::Tokens(_) => i += 1,
            }
        }
        self.parsed.remove(0).num()
    }
}
