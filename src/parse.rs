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
            if token.len() == 1
                && let Ok(operator) = Operators::try_from(token.chars().next().unwrap())
            {
                parsed.push(Token::Operator(operator));
            } else if let Ok(n) = token.parse() {
                parsed.push(Token::Num(n));
            } else if let Some(value) = Constants::get(token) {
                parsed.push(Token::Num(value));
            } else {
                parsed.push(Token::Operator(Operators::Fun(
                    Function::try_from(token).unwrap(),
                )));
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
                        parsed.push(Token::Operator(operator_stack.pop().unwrap()));
                    }
                    negate = true;
                }
                ')' => {
                    while let Some(top) = operator_stack.last()
                        && *top != Operators::LeftParenthesis
                    {
                        parsed.push(Token::Operator(operator_stack.pop().unwrap()));
                    }
                    operator_stack.pop();
                    if let Some(top) = operator_stack.last()
                        && matches!(top, Operators::Fun(_))
                    {
                        parsed.push(Token::Operator(operator_stack.pop().unwrap()));
                    }
                    negate = false;
                }
                'a'..='z' => {
                    let mut l = 0;
                    for (i, t) in value[i..].char_indices() {
                        if t.is_ascii_alphabetic() {
                            l = i;
                        } else {
                            break;
                        }
                    }
                    let l = l + c.len_utf8();
                    if let Some(value) = Constants::get(&value[i..i + l]) {
                        parsed.push(Token::Num(value));
                    } else {
                        operator_stack.push(Operators::Fun(
                            Function::try_from(&value[i..i + l]).unwrap(),
                        ));
                    }
                    let _ = chars.advance_by(l - 1);
                    negate = false;
                }
                '0'..='9' if c.is_ascii_digit() => {
                    let mut l = 0;
                    for t in value[i..].chars() {
                        if t.is_ascii_digit() || t == '.' {
                            l += 1;
                        } else {
                            break;
                        }
                    }
                    parsed.push(Token::Num(value[i..i + l].parse().unwrap()));
                    let _ = chars.advance_by(l - 1);
                    negate = false;
                }
                _ if let Ok(mut operator) = Operators::try_from(c) => {
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
                            parsed.push(Token::Operator(operator_stack.pop().unwrap()));
                        }
                    }
                    operator_stack.push(operator);
                    negate = true;
                }
                _ => unreachable!(),
            }
        }
        while let Some(operator) = operator_stack.pop() {
            parsed.push(Token::Operator(operator));
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
    LeftParenthesis,
    Negate,
    Fun(Function),
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Function {
    Sin,
    Cos,
    Atan,
    Max,
    Min,
    Quadratic,
    Ln,
}
impl TryFrom<&str> for Function {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
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
impl TryFrom<char> for Operators {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '^' => Operators::Pow,
            '*' => Operators::Mul,
            '/' => Operators::Div,
            '+' => Operators::Add,
            '-' => Operators::Sub,
            '_' => Operators::Negate,
            '(' => Operators::LeftParenthesis,
            _ => return Err(()),
        })
    }
}
impl Function {
    pub fn inputs(self) -> usize {
        match self {
            Function::Cos | Function::Sin | Function::Ln => 1,
            Function::Atan => 2,
            Function::Max => 2,
            Function::Min => 2,
            Function::Quadratic => 3,
        }
    }
}
impl Operators {
    pub const MAX_INPUT: usize = 3;
    pub fn inputs(self) -> usize {
        match self {
            Operators::Mul | Operators::Div | Operators::Add | Operators::Sub | Operators::Pow => 2,
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
            Operators::Pow => 3,
            Operators::LeftParenthesis | Operators::Fun(_) => unreachable!(),
        }
    }
    pub fn left_associative(self) -> bool {
        match self {
            Operators::Add | Operators::Sub | Operators::Mul | Operators::Div => true,
            Operators::Pow | Operators::Negate => false,
            Operators::LeftParenthesis | Operators::Fun(_) => unreachable!(),
        }
    }
    pub fn is_operator(self) -> bool {
        !matches!(self, Operators::Fun(_) | Operators::LeftParenthesis)
    }
    pub fn compute<T>(self, a: &mut f64, b: T)
    where
        T: Fn(usize) -> f64,
    {
        match self {
            Operators::Add => {
                *a += b(0);
            }
            Operators::Sub => {
                *a -= b(0);
            }
            Operators::Mul => {
                *a *= b(0);
            }
            Operators::Div => {
                *a /= b(0);
            }
            Operators::Pow => {
                *a = a.powf(b(0));
            }
            Operators::Negate => {
                *a = a.neg();
            }
            Operators::Fun(fun) => match fun {
                Function::Sin => *a = a.sin(),
                Function::Ln => *a = a.ln(),
                Function::Cos => *a = a.cos(),
                Function::Atan => {
                    *a = a.atan2(b(0));
                }
                Function::Max => {
                    *a = a.max(b(0));
                }
                Function::Min => {
                    *a = a.min(b(0));
                }
                Function::Quadratic => {
                    *a = ((b(0) * b(0) - 4.0 * *a * b(1)).sqrt() - b(0)) / (2.0 * *a);
                }
            },
            Operators::LeftParenthesis => {
                unreachable!()
            }
        }
    }
}
pub enum Constants {
    Pi,
    E,
}
impl TryFrom<&str> for Constants {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "pi" => Constants::Pi,
            "e" => Constants::E,
            _ => return Err(()),
        })
    }
}
impl Constants {
    pub fn get(value: &str) -> Option<f64> {
        if let Ok(c) = Self::try_from(value) {
            Some(match c {
                Constants::Pi => std::f64::consts::PI,
                Constants::E => std::f64::consts::E,
            })
        } else {
            None
        }
    }
}
