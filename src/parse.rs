#[derive(Default, Debug)]
pub struct Parsed {
    pub numbers: Vec<f64>,
    pub operations: Vec<Operators>,
}
#[derive(Debug)]
pub enum ParsedError {}
impl TryFrom<&str> for Parsed {
    type Error = ParsedError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parsed = Parsed::default();
        let mut chars = value.chars();
        while let Some(mut c) = chars.next() {
            if c.is_ascii_digit() {
                let mut n = c.to_string();
                let mut cont = true;
                while let Some(t) = chars.next() {
                    if t.is_ascii_digit() || t == '.' {
                        n.push(t);
                    } else {
                        cont = false;
                        c = t;
                        break;
                    }
                }
                parsed.numbers.push(n.parse().unwrap());
                if cont {
                    continue;
                }
            }
            let operator = match c {
                '*' => Operators::Mul,
                '/' => Operators::Div,
                '+' => Operators::Add,
                '-' => Operators::Sub,
                _ => unreachable!(),
            };
            parsed.operations.push(operator);
        }
        Ok(parsed)
    }
}
#[derive(Debug)]
pub enum Operators {
    Add,
    Sub,
    Mul,
    Div,
}
