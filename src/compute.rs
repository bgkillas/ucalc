use crate::parse::{Operators, Parsed};
impl Parsed {
    pub fn compute(mut self) -> f64 {
        for op in self.operations.into_iter() {
            let b = self.numbers.pop().unwrap();
            let a = self.numbers.last_mut().unwrap();
            match op {
                Operators::Add => {
                    *a += b;
                }
                Operators::Sub => {
                    *a -= b;
                }
                Operators::Mul => {
                    *a *= b;
                }
                Operators::Div => {
                    *a /= b;
                }
            }
        }
        self.numbers[0]
    }
}
