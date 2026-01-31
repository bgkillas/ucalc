use crate::InnerVariables;
use crate::parse::{Operators, Parsed, Token};
impl Parsed {
    pub fn clone_compute(&mut self) -> f64 {
        let parsed = self.parsed.clone();
        let ret = self.compute();
        self.parsed = parsed;
        ret
    }
    pub fn clone_compute_inner(&mut self, vars: Option<InnerVariables>) -> f64 {
        let parsed = self.parsed.clone();
        let ret = self.compute_inner(vars);
        self.parsed = parsed;
        ret
    }
    pub fn compute(&mut self) -> f64 {
        self.compute_inner(None)
    }
    pub fn compute_inner(&mut self, vars: Option<InnerVariables>) -> f64 {
        let mut b = Vec::with_capacity(Operators::MAX_INPUT - 1);
        let mut i = 0;
        while i < self.parsed.len() {
            match self.parsed[i] {
                Token::Operator(operator) => {
                    self.parsed.remove(i);
                    let inputs = operator.inputs();
                    b.extend(self.parsed.drain(i - (inputs - 1)..i).map(|a| a.num()));
                    let a = self.parsed.get_mut(i - inputs).unwrap().num_mut();
                    operator.compute(a, &b);
                    b.clear();
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
                        .zip(self.parsed.drain(i - (inputs - 1)..i).map(|a| a.num()))
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
                    .compute_inner(Some(inner));
                    i -= inputs - 1;
                }
                Token::Num(_) => i += 1,
            }
        }
        self.parsed.remove(0).num()
    }
}
