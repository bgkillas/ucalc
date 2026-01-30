use std::ops::Neg;
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Parsed {
    pub parsed: Vec<Token>,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Num(f64),
    Operator(Operators),
}
impl From<f64> for Token {
    fn from(value: f64) -> Self {
        Self::Num(value)
    }
}
impl From<Operators> for Token {
    fn from(value: Operators) -> Self {
        Self::Operator(value)
    }
}
impl From<Function> for Operators {
    fn from(value: Function) -> Self {
        Self::Fun(value)
    }
}
impl From<Function> for Token {
    fn from(value: Function) -> Self {
        Self::Operator(value.into())
    }
}
impl Token {
    pub fn num(self) -> f64 {
        let Token::Num(num) = self else {
            unreachable!()
        };
        num
    }
    pub fn num_ref(&self) -> f64 {
        let Token::Num(num) = self else {
            unreachable!()
        };
        *num
    }
    pub fn num_mut(&mut self) -> &mut f64 {
        let Token::Num(num) = self else {
            unreachable!()
        };
        num
    }
}
#[derive(Debug, PartialEq)]
pub enum ParseError {}
impl Parsed {
    pub fn rpn(value: &str) -> Result<Self, ParseError> {
        let mut parsed = Vec::with_capacity(value.len());
        for token in value.split(' ') {
            if token.is_empty() {
                continue;
            }
            if let Ok(operator) = Operators::try_from(token) {
                parsed.push(operator.into());
            } else if let Ok(n) = token.parse::<f64>() {
                parsed.push(n.into());
            } else if let Some(value) = get_constant(token) {
                parsed.push(value.into());
            } else {
                parsed.push(Function::try_from(token).unwrap().into());
            }
        }
        Ok(Self { parsed })
    }
    pub fn infix(value: &str) -> Result<Self, ParseError> {
        let mut parsed = Vec::with_capacity(value.len());
        let mut operator_stack: Vec<Operators> = Vec::with_capacity(value.len());
        let mut chars = value.char_indices();
        let mut negate = true;
        while let Some((i, c)) = chars.next() {
            match c {
                ' ' => {}
                ',' => {
                    while let Some(top) = operator_stack.last()
                        && *top != Operators::LeftParenthesis
                    {
                        parsed.push(operator_stack.pop().unwrap().into());
                    }
                    negate = true;
                }
                ')' => {
                    while let Some(top) = operator_stack.last()
                        && *top != Operators::LeftParenthesis
                    {
                        parsed.push(operator_stack.pop().unwrap().into());
                    }
                    operator_stack.pop();
                    if let Some(top) = operator_stack.last()
                        && matches!(top, Operators::Fun(_))
                    {
                        parsed.push(operator_stack.pop().unwrap().into());
                    }
                    negate = false;
                }
                'a'..='z' => {
                    let mut l = c.len_utf8();
                    let mut count = 1;
                    for t in value[i + l..].chars() {
                        if t.is_ascii_alphabetic() {
                            l += t.len_utf8();
                            count += 1;
                        } else {
                            break;
                        }
                    }
                    if let Some(value) = get_constant(&value[i..i + l]) {
                        parsed.push(value.into());
                    } else {
                        operator_stack.push(Function::try_from(&value[i..i + l]).unwrap().into());
                    }
                    let _ = chars.advance_by(count - 1);
                    negate = false;
                }
                '0'..='9' if c.is_ascii_digit() => {
                    let mut l = 1;
                    for t in value[i + 1..].chars() {
                        if t.is_ascii_digit() || t == '.' {
                            l += 1;
                        } else {
                            break;
                        }
                    }
                    parsed.push(value[i..i + l].parse::<f64>().unwrap().into());
                    let _ = chars.advance_by(l - 1);
                    negate = false;
                }
                _ => {
                    let mut l = c.len_utf8();
                    if let Some(next) = value[i + l..].chars().next()
                        && c == next
                    {
                        chars.next();
                        l += next.len_utf8();
                    }
                    if let Ok(mut operator) = Operators::try_from(&value[i..i + l]) {
                        if negate && Operators::Sub == operator {
                            operator = Operators::Negate;
                        }
                        if operator != Operators::LeftParenthesis {
                            while let Some(top) = operator_stack.last()
                                && *top != Operators::LeftParenthesis
                                && (top.precedence() > operator.precedence()
                                    || (top.precedence() == operator.precedence()
                                        && operator.left_associative()))
                            {
                                parsed.push(operator_stack.pop().unwrap().into());
                            }
                        }
                        operator_stack.push(operator);
                        negate = true;
                    } else {
                        unreachable!()
                    }
                }
            }
        }
        while let Some(operator) = operator_stack.pop() {
            parsed.push(operator.into());
        }
        Ok(Self { parsed })
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operators {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Root,
    LeftParenthesis,
    Negate,
    Fun(Function),
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Function {
    Sin,
    Asin,
    Cos,
    Acos,
    Ln,
    Exp,
    Atan,
    Max,
    Min,
    Quadratic,
}
impl TryFrom<&str> for Function {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "exp" => Function::Exp,
            "asin" => Function::Asin,
            "acos" => Function::Acos,
            "ln" => Function::Ln,
            "min" => Function::Min,
            "max" => Function::Max,
            "sin" => Function::Sin,
            "cos" => Function::Cos,
            "atan" => Function::Atan,
            "quadratic" => Function::Quadratic,
            _ => return Err(()),
        })
    }
}
impl TryFrom<&str> for Operators {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "//" => Operators::Root,
            "^" | "**" => Operators::Pow,
            "*" => Operators::Mul,
            "/" => Operators::Div,
            "+" => Operators::Add,
            "-" => Operators::Sub,
            "_" => Operators::Negate,
            "(" => Operators::LeftParenthesis,
            _ => return Err(()),
        })
    }
}
impl Function {
    pub fn inputs(self) -> usize {
        match self {
            Function::Cos
            | Function::Sin
            | Function::Ln
            | Function::Acos
            | Function::Asin
            | Function::Exp => 1,
            Function::Atan => 2,
            Function::Max => 2,
            Function::Min => 2,
            Function::Quadratic => 3,
        }
    }
    pub fn compute(self, a: &mut f64, b: &[f64]) {
        match self {
            Function::Sin => *a = a.sin(),
            Function::Ln => *a = a.ln(),
            Function::Cos => *a = a.cos(),
            Function::Acos => *a = a.acos(),
            Function::Asin => *a = a.asin(),
            Function::Exp => *a = a.exp(),
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
        }
    }
    pub fn inverse(self) -> Option<Self> {
        Some(match self {
            Function::Sin => Function::Asin,
            Function::Cos => Function::Acos,
            Function::Ln => Function::Exp,
            Function::Asin => Function::Sin,
            Function::Acos => Function::Cos,
            Function::Exp => Function::Ln,
            Function::Max | Function::Min | Function::Quadratic | Function::Atan => return None,
        })
    }
}
impl Operators {
    pub const MAX_INPUT: usize = 3;
    pub fn inverse(self) -> Option<Self> {
        Some(match self {
            Operators::Add => Operators::Sub,
            Operators::Sub => Operators::Add,
            Operators::Mul => Operators::Div,
            Operators::Div => Operators::Mul,
            Operators::Pow => Operators::Root,
            Operators::Root => Operators::Pow,
            Operators::LeftParenthesis => return None,
            Operators::Negate => Operators::Negate,
            Operators::Fun(fun) => return fun.inverse().map(|a| a.into()),
        })
    }
    pub fn inputs(self) -> usize {
        match self {
            Operators::Mul
            | Operators::Div
            | Operators::Add
            | Operators::Sub
            | Operators::Pow
            | Operators::Root => 2,
            Operators::Negate => 1,
            Operators::Fun(fun) => fun.inputs(),
            Operators::LeftParenthesis => unreachable!(),
        }
    }
    pub fn precedence(self) -> u8 {
        match self {
            Operators::Add | Operators::Sub => 0,
            Operators::Mul | Operators::Div => 1,
            Operators::Negate => 2,
            Operators::Pow | Operators::Root => 3,
            Operators::LeftParenthesis | Operators::Fun(_) => unreachable!(),
        }
    }
    pub fn left_associative(self) -> bool {
        match self {
            Operators::Add | Operators::Sub | Operators::Mul | Operators::Div => true,
            Operators::Pow | Operators::Root | Operators::Negate => false,
            Operators::LeftParenthesis | Operators::Fun(_) => unreachable!(),
        }
    }
    pub fn is_operator(self) -> bool {
        !matches!(self, Operators::Fun(_) | Operators::LeftParenthesis)
    }
    pub fn compute(self, a: &mut f64, b: &[f64]) {
        match self {
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
            Operators::Root => {
                *a = a.powf(b[0].recip());
            }
            Operators::Negate => {
                *a = a.neg();
            }
            Operators::Fun(fun) => fun.compute(a, b),
            Operators::LeftParenthesis => {
                unreachable!()
            }
        }
    }
}
pub fn get_constant(value: &str) -> Option<f64> {
    Some(match value {
        "pi" => std::f64::consts::PI,
        "e" => std::f64::consts::E,
        _ => return None,
    })
}
