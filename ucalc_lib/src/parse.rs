use crate::variable::{Functions, Variables};
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut, Neg};
use ucalc_numbers::{Complex, Pow};
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
    Num(Complex),
    InnerVar(usize),
    Var(usize),
    Fun(usize),
    Tokens(Tokens),
    Operator(Operators),
}
impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Num(n) => write!(f, "{}", n),
            Token::Tokens(t) => write!(f, "[{}]", t),
            Token::Operator(Operators::Fun(fun)) => write!(f, "{:?}", fun),
            _ => write!(f, "{:?}", self),
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnknownToken(String),
    LeftParenthesisNotFound,
    RightParenthesisNotFound,
}
impl Display for Tokens {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for token in self.iter() {
            if !first {
                write!(f, " ")?;
            }
            write!(f, "{}", token)?;
            first = false
        }
        Ok(())
    }
}
impl Tokens {
    pub fn get_last(&self, funs: &Functions) -> usize {
        fn inner(tokens: &[Token], funs: &Functions) -> usize {
            match tokens.last() {
                //TODO
                Some(Token::Fun(i)) => {
                    let inputs = funs[*i].vars.len();
                    let mut i = tokens.len() - 1;
                    for _ in 0..inputs {
                        i = inner(&tokens[..i], funs)
                    }
                    i
                }
                Some(Token::Operator(o)) => {
                    let inputs = o.inputs();
                    let mut i = tokens.len() - 1;
                    for _ in 0..inputs {
                        i = inner(&tokens[..i], funs)
                    }
                    i
                }
                _ => tokens.len() - 1,
            }
        }
        inner(self, funs)
    }
}
impl Parsed {
    pub fn rpn(value: &str, vars: Variables, funs: Functions) -> Result<Self, ParseError> {
        let mut parsed = Tokens(Vec::with_capacity(value.len()));
        let mut inner_vars: Vec<&str> = Vec::with_capacity(value.len());
        for token in value.split(' ') {
            if token.is_empty() {
                continue;
            }
            if let Ok(operator) = Operators::try_from(token) {
                parsed.push(operator.into());
            } else if let Ok(n) = Complex::parse_radix(token, 10) {
                parsed.push(n.into());
            } else if let Ok(fun) = Function::try_from(token) {
                if fun.has_var() {
                    let last = parsed.get_last(&funs);
                    let tokens = Tokens(parsed.drain(last..).collect());
                    parsed.push(Token::Tokens(tokens));
                    inner_vars.pop();
                }
                parsed.push(fun.into());
            } else if let Some((i, v)) = vars.iter().enumerate().find(|(_, v)| v.name == token) {
                if v.place {
                    parsed.push(Token::Num(v.value));
                } else {
                    parsed.push(Token::Var(i));
                }
            } else if let Some(i) = funs.iter().position(|v| v.name == token) {
                parsed.push(Token::Fun(i))
            } else if let Some(i) = inner_vars.iter().position(|v| *v == token) {
                parsed.push(Token::InnerVar(i));
            } else {
                inner_vars.push(token);
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
                    let Ok(float) = Complex::parse_radix(s, 10) else {
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
    Rem,
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
    Sqrt,
    Sum,
    Prod,
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
            "sqrt" => Function::Sqrt,
            "sum" => Function::Sum,
            "prod" => Function::Prod,
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
            "%" => Operators::Rem,
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
            | Function::Exp
            | Function::Sqrt => 1,
            Function::Atan | Function::Max | Function::Min => 2,
            Function::Quadratic | Function::Sum | Function::Prod => 3,
            Function::Custom(_) => unreachable!(),
        }
    }
    pub fn has_var(self) -> bool {
        match self {
            Function::Cos
            | Function::Sin
            | Function::Ln
            | Function::Acos
            | Function::Asin
            | Function::Exp
            | Function::Sqrt
            | Function::Custom(_)
            | Function::Atan
            | Function::Max
            | Function::Min
            | Function::Quadratic => false,
            Function::Sum | Function::Prod => true,
        }
    }
    pub fn compute(self, a: &mut Complex, b: &[Complex]) {
        match self {
            Function::Sin => a.sin_mut(),
            Function::Ln => a.ln_mut(),
            Function::Cos => a.cos_mut(),
            Function::Acos => a.acos_mut(),
            Function::Asin => a.asin_mut(),
            Function::Exp => a.exp_mut(),
            Function::Sqrt => a.sqrt_mut(),
            Function::Atan => {
                a.atan2_mut(&b[0]);
            }
            Function::Max => {
                a.max_mut(&b[0]);
            }
            Function::Min => {
                a.min_mut(&b[0]);
            }
            Function::Quadratic => {
                *a = ((b[0] * b[0] - *a * b[1] * 4).sqrt() - b[0]) / (*a * 2);
            }
            Function::Custom(_) | Function::Sum | Function::Prod => unreachable!(),
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
            Function::Max
            | Function::Min
            | Function::Quadratic
            | Function::Atan
            | Function::Sqrt
            | Function::Sum
            | Function::Prod => return None,
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
            Operators::LeftParenthesis | Operators::Rem => return None,
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
            | Operators::Root
            | Operators::Rem => 2,
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
            Operators::Rem => 4,
            Operators::LeftParenthesis | Operators::Fun(_) => unreachable!(),
        }
    }
    pub fn left_associative(self) -> bool {
        match self {
            Operators::Add | Operators::Sub | Operators::Mul | Operators::Div | Operators::Rem => {
                true
            }
            Operators::Pow | Operators::Root | Operators::Negate => false,
            Operators::LeftParenthesis | Operators::Fun(_) => unreachable!(),
        }
    }
    pub fn is_operator(self) -> bool {
        !matches!(self, Operators::Fun(_) | Operators::LeftParenthesis)
    }
    pub fn compute(self, a: &mut Complex, b: &[Complex]) {
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
            Operators::Rem => {
                *a %= b[0];
            }
            Operators::Pow => {
                *a = a.pow(b[0]);
            }
            Operators::Root => {
                *a = a.pow(b[0].recip());
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
impl From<Complex> for Token {
    fn from(value: Complex) -> Self {
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
    pub fn num(self) -> Complex {
        let Token::Num(num) = self else {
            unreachable!()
        };
        num
    }
    pub fn inner_var(self) -> usize {
        let Token::InnerVar(n) = self else {
            unreachable!()
        };
        n
    }
    pub fn tokens(self) -> Tokens {
        let Token::Tokens(t) = self else {
            unreachable!()
        };
        t
    }
    pub fn tokens_mut(&mut self) -> &mut Tokens {
        let Token::Tokens(t) = self else {
            unreachable!()
        };
        t
    }
    pub fn num_mut(&mut self) -> &mut Complex {
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
