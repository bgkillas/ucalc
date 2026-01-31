use crate::variable::{Functions, Variables};
use std::ops::{Deref, DerefMut, Neg};
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Tokens(pub Vec<Token>);
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Parsed {
    pub parsed: Tokens,
    pub vars: Variables,
    pub funs: Functions,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Num(f64),
    InnerVar(usize),
    Var(usize),
    Fun(usize),
    Operator(Operators),
}
#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnknownToken(String),
    LeftParenthesisNotFound,
    RightParenthesisNotFound,
}
impl Parsed {
    pub fn rpn(value: &str, vars: Variables, funs: Functions) -> Result<Self, ParseError> {
        let mut parsed = Tokens(Vec::with_capacity(value.len()));
        for token in value.split(' ') {
            if token.is_empty() {
                continue;
            }
            if let Ok(operator) = Operators::try_from(token) {
                parsed.push(operator.into());
            } else if let Ok(n) = token.parse::<f64>() {
                parsed.push(n.into());
            } else if let Ok(fun) = Function::try_from(token) {
                parsed.push(fun.into());
            } else if let Some((i, v)) = vars.iter().enumerate().find(|(_, v)| v.name == token) {
                if v.place {
                    parsed.push(Token::Num(v.value));
                } else {
                    parsed.push(Token::Var(i));
                }
            } else if let Some(i) = funs.iter().position(|v| v.name == token) {
                parsed.push(Token::Fun(i))
            } else {
                return Err(ParseError::UnknownToken(token.to_string()));
            }
        }
        Ok(Self { parsed, vars, funs })
    }
    pub fn infix(value: &str, vars: Variables, funs: Functions) -> Result<Self, ParseError> {
        let mut parsed = Tokens(Vec::with_capacity(value.len()));
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
                    if operator_stack.pop() != Some(Operators::LeftParenthesis) {
                        return Err(ParseError::LeftParenthesisNotFound);
                    }
                    if let Some(top) = operator_stack.last() {
                        match top {
                            Operators::Fun(Function::Custom(i)) => {
                                parsed.push(Token::Fun(*i));
                                operator_stack.pop();
                            }
                            Operators::Fun(_) => parsed.push(operator_stack.pop().unwrap().into()),
                            _ => {}
                        }
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
                    let s = &value[i..i + l];
                    if let Ok(fun) = Function::try_from(s) {
                        operator_stack.push(fun.into());
                    } else if let Some((i, v)) = vars.iter().enumerate().find(|(_, v)| v.name == s)
                    {
                        if v.place {
                            parsed.push(Token::Num(v.value));
                        } else {
                            parsed.push(Token::Var(i));
                        }
                    } else if let Some(i) = funs.iter().position(|v| v.name == s) {
                        operator_stack.push(Operators::Fun(Function::Custom(i)));
                    } else {
                        return Err(ParseError::UnknownToken(s.to_string()));
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
                    let s = &value[i..i + l];
                    let Ok(float) = s.parse::<f64>() else {
                        return Err(ParseError::UnknownToken(s.to_string()));
                    };
                    parsed.push(float.into());
                    let _ = chars.advance_by(l - 1);
                    negate = false;
                }
                '(' => {
                    operator_stack.push(Operators::LeftParenthesis);
                    negate = true;
                }
                _ => {
                    let mut l = c.len_utf8();
                    if let Some(next) = value[i + l..].chars().next()
                        && c == next
                    {
                        chars.next();
                        l += next.len_utf8();
                    }
                    let s = &value[i..i + l];
                    if let Ok(mut operator) = Operators::try_from(s) {
                        if negate && Operators::Sub == operator {
                            operator = Operators::Negate;
                        }
                        while let Some(top) = operator_stack.last()
                            && *top != Operators::LeftParenthesis
                            && (top.precedence() > operator.precedence()
                                || (top.precedence() == operator.precedence()
                                    && operator.left_associative()))
                        {
                            parsed.push(operator_stack.pop().unwrap().into());
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
            if operator == Operators::LeftParenthesis {
                return Err(ParseError::RightParenthesisNotFound);
            }
            parsed.push(operator.into());
        }
        Ok(Self { parsed, vars, funs })
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
    Custom(usize),
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
            Function::Custom(_) => unreachable!(),
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
            Function::Custom(_) => unreachable!(),
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
            Function::Custom(_) => unreachable!(),
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
    pub fn num_mut(&mut self) -> &mut f64 {
        let Token::Num(num) = self else {
            unreachable!()
        };
        num
    }
}
impl Deref for Tokens {
    type Target = Vec<Token>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Tokens {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
