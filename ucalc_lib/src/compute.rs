use crate::parse::{Operators, Parsed, Token};
impl Parsed {
    pub fn compute(&mut self) -> f64 {
        let mut b = Vec::with_capacity(Operators::MAX_INPUT - 1);
        let mut i = 0;
        while i < self.parsed.len() {
            if let Token::Operator(operator) = self.parsed[i] {
                self.parsed.remove(i);
                let inputs = operator.inputs();
                b.extend(self.parsed.drain(i - (inputs - 1)..i).map(|a| a.num()));
                let a = self.parsed.get_mut(i - inputs).unwrap().num_mut();
                operator.compute(a, &b);
                b.clear();
                i -= inputs - 1;
            } else {
                i += 1;
            }
        }
        self.parsed.remove(0).num()
    }
}
