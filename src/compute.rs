use crate::parse::{Operators, Parsed, Token};
impl Parsed {
    pub fn compute(&mut self) -> f64 {
        self.compute_buffer(&mut Vec::with_capacity(Operators::MAX_INPUT - 1))
    }
    pub fn compute_buffer(&mut self, b: &mut Vec<f64>) -> f64 {
        let mut i = 0;
        while i < self.parsed.len() {
            if let Token::Operator(operator) = self.parsed[i] {
                self.parsed.remove(i);
                let inputs = operator.inputs().unwrap();
                b.extend(self.parsed.drain(i - (inputs - 1)..i).map(|a| a.num()));
                let a = self.parsed.get_mut(i - inputs).unwrap().num_mut();
                operator.compute(a, |k| b[k]);
                b.clear();
                i -= inputs;
            }
            i += 1;
        }
        self.parsed[0].num_ref()
    }
}
