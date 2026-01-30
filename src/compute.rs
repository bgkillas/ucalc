use crate::parse::{Function, NumOp, Operators, Parsed};
use std::ops::Neg;
impl Parsed {
    pub fn compute(mut self) -> f64 {
        let mut i = 0;
        let mut b = Vec::with_capacity(self.parsed.len());
        while i < self.parsed.len() {
            if let NumOp::Operator(operator) = self.parsed[i] {
                self.parsed.remove(i);
                let inputs = operator.inputs().unwrap();
                b.extend(self.parsed.drain(i - (inputs - 1)..i).map(|a| a.num()));
                let a = self.parsed.get_mut(i - inputs).unwrap().num_mut();
                match operator {
                    Operators::Add => {
                        *a += b[0];
                    }
                    Operators::Sub => {
                        *a -= b[0];
                    }
                    Operators::Mul => {
                        *a *= b[0];
                    }
                    Operators::Div => {
                        *a /= b[0];
                    }
                    Operators::Pow => {
                        *a = a.powf(b[0]);
                    }
                    Operators::Negate => {
                        *a = a.neg();
                    }
                    Operators::Fun(fun) => match fun {
                        Function::Sin => *a = a.sin(),
                        Function::Ln => *a = a.ln(),
                        Function::Cos => *a = a.cos(),
                        Function::Atan => {
                            *a = a.atan2(b[0]);
                        }
                        Function::Max => {
                            *a = a.max(b[0]);
                        }
                        Function::Min => {
                            *a = a.min(b[0]);
                        }
                        Function::Quadratic => {
                            *a = ((b[0] * b[0] - 4.0 * *a * b[1]).sqrt() - b[0]) / (2.0 * *a);
                        }
                    },
                    Operators::LeftParenthesis => {
                        unreachable!()
                    }
                }
                b.clear();
                i -= inputs;
            }
            i += 1;
        }
        self.parsed.remove(0).num()
    }
}
