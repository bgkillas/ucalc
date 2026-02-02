use crate::functions::Function;
use crate::operators::{Bracket, Operators};
use crate::variable::{Functions, Variables};
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use ucalc_numbers::Complex;
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Tokens(pub Vec<Token>);
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
            Token::Operator(Operators::Function(fun)) => write!(f, "{:?}", fun),
            _ => write!(f, "{:?}", self),
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnknownToken(String),
    LeftParenthesisNotFound,
    RightParenthesisNotFound,
    AbsoluteBracketFailed,
    MissingInput,
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
    pub fn rpn(value: &str, vars: &Variables, funs: &Functions) -> Result<Self, ParseError> {
        let mut tokens = Tokens(Vec::with_capacity(value.len()));
        let mut inner_vars: Vec<&str> = Vec::with_capacity(value.len());
        for token in value.split(' ') {
            if token.is_empty() {
                continue;
            }
            if let Ok(operator) = Operators::try_from(token) {
                tokens.push(operator.into());
            } else if let Ok(n) = Complex::parse_radix(token, 10) {
                tokens.push(n.into());
            } else if let Ok(fun) = Function::try_from(token) {
                tokens.compact_args(&fun, &mut inner_vars, funs);
                tokens.push(fun.into());
            } else if let Some((i, v)) = vars.iter().enumerate().find(|(_, v)| v.name == token) {
                if v.place {
                    tokens.push(Token::Num(v.value));
                } else {
                    tokens.push(Token::Var(i));
                }
            } else if let Some(i) = funs.iter().position(|v| v.name == token) {
                tokens.push(Token::Fun(i))
            } else if let Some(i) = inner_vars.iter().position(|v| *v == token) {
                tokens.push(Token::InnerVar(i));
            } else if token.chars().all(|c| c.is_ascii_alphabetic()) {
                inner_vars.push(token);
            } else {
                return Err(ParseError::UnknownToken(token.to_string()));
            }
        }
        Ok(tokens)
    }
    pub fn infix(value: &str, vars: &Variables, funs: &Functions) -> Result<Self, ParseError> {
        let mut tokens = Tokens(Vec::with_capacity(value.len()));
        let mut operator_stack: Vec<Operators> = Vec::with_capacity(value.len());
        let mut inner_vars: Vec<&str> = Vec::with_capacity(value.len());
        let mut chars = value.char_indices();
        let mut negate = true;
        let mut last_abs = false;
        let mut req_input = false;
        let mut abs = 0;
        while let Some((i, c)) = chars.next() {
            match c {
                ' ' => {}
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
                            tokens.push(Token::Num(v.value));
                        } else {
                            tokens.push(Token::Var(i));
                        }
                    } else if let Some(i) = funs.iter().position(|v| v.name == s) {
                        operator_stack.push(Operators::Function(Function::Custom(i)));
                    } else if let Some(i) = inner_vars.iter().position(|v| *v == s) {
                        tokens.push(Token::InnerVar(i));
                    } else {
                        inner_vars.push(s);
                    }
                    let _ = chars.advance_by(count - 1);
                    negate = false;
                    last_abs = false;
                    req_input = false;
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
                    tokens.push(float.into());
                    let _ = chars.advance_by(l - 1);
                    negate = false;
                    last_abs = false;
                    req_input = false;
                }
                ',' => {
                    while let Some(top) = operator_stack.last()
                        && !matches!(top, Operators::Bracket(_))
                    {
                        tokens.push(operator_stack.pop().unwrap().into());
                    }
                    negate = true;
                    last_abs = false;
                }
                ')' => {
                    if req_input {
                        return Err(ParseError::MissingInput);
                    }
                    while let Some(top) = operator_stack.last()
                        && !matches!(top, Operators::Bracket(_))
                    {
                        tokens.push(operator_stack.pop().unwrap().into());
                    }
                    if !matches!(
                        operator_stack.pop(),
                        Some(Operators::Bracket(Bracket::Parenthesis))
                    ) {
                        return Err(ParseError::LeftParenthesisNotFound);
                    }
                    tokens.close_off_bracket(&mut operator_stack, &mut inner_vars, funs);
                    negate = false;
                    last_abs = false;
                }
                '(' => {
                    operator_stack.push(Bracket::Parenthesis.into());
                    negate = true;
                    last_abs = false;
                    req_input = false;
                }
                '|' => {
                    if abs == 0 || last_abs || req_input {
                        operator_stack.push(Bracket::Absolute.into());
                        abs += 1;
                        negate = true;
                        last_abs = true;
                        req_input = false;
                    } else {
                        while let Some(top) = operator_stack.last()
                            && !matches!(top, Operators::Bracket(_))
                        {
                            tokens.push(operator_stack.pop().unwrap().into());
                        }
                        if !matches!(
                            operator_stack.pop(),
                            Some(Operators::Bracket(Bracket::Absolute))
                        ) {
                            return Err(ParseError::AbsoluteBracketFailed);
                        }
                        tokens.close_off_bracket(&mut operator_stack, &mut inner_vars, funs);
                        tokens.push(Function::Abs.into());
                        abs -= 1;
                        negate = false;
                        last_abs = false;
                    }
                }
                _ => {
                    let mut l = c.len_utf8();
                    if let Some(next) = value[i + l..].chars().next()
                        && Operators::try_from(&value[i..i + l + next.len_utf8()]).is_ok()
                    {
                        chars.next();
                        l += next.len_utf8();
                    }
                    let s = &value[i..i + l];
                    if let Ok(mut operator) = Operators::try_from(s) {
                        if negate {
                            match operator {
                                Operators::Sub => operator = Operators::Negate,
                                Operators::Factorial => operator = Operators::SubFactorial,
                                _ => {}
                            }
                        }
                        while let Some(top) = operator_stack.last()
                            && !matches!(top, Operators::Bracket(_))
                            && (top.precedence() > operator.precedence()
                                || (top.precedence() == operator.precedence()
                                    && operator.left_associative()))
                        {
                            tokens.push(operator_stack.pop().unwrap().into());
                        }
                        operator_stack.push(operator);
                        if operator.inputs() == 2 {
                            req_input = true;
                        }
                        negate = operator != Operators::Factorial;
                        last_abs = false;
                    } else {
                        unreachable!()
                    }
                }
            }
        }
        while let Some(operator) = operator_stack.pop() {
            if let Operators::Bracket(bracket) = operator {
                return match bracket {
                    Bracket::Absolute => Err(ParseError::AbsoluteBracketFailed),
                    Bracket::Parenthesis => Err(ParseError::RightParenthesisNotFound),
                };
            }
            tokens.push(operator.into());
        }
        Ok(tokens)
    }
}
impl Tokens {
    pub fn get_last(tokens: &[Token], funs: &Functions) -> usize {
        match tokens.last() {
            Some(Token::Fun(i)) => {
                let inputs = funs[*i].vars.len();
                let mut i = tokens.len() - 1;
                for _ in 0..inputs {
                    i = Tokens::get_last(&tokens[..i], funs)
                }
                i
            }
            Some(Token::Operator(o)) => {
                let inputs = o.inputs();
                let mut i = tokens.len() - 1;
                for _ in 0..inputs {
                    i = Tokens::get_last(&tokens[..i], funs)
                }
                i
            }
            _ => tokens.len() - 1,
        }
    }
    pub fn close_off_bracket(
        &mut self,
        operator_stack: &mut Vec<Operators>,
        inner_vars: &mut Vec<&str>,
        funs: &Functions,
    ) {
        if let Some(top) = operator_stack.last() {
            match top {
                Operators::Function(Function::Custom(i)) => {
                    self.push(Token::Fun(*i));
                    operator_stack.pop();
                }
                Operators::Function(fun) => {
                    self.compact_args(fun, inner_vars, funs);
                    self.push(operator_stack.pop().unwrap().into())
                }
                _ => {}
            }
        }
    }
    pub fn compact_args(&mut self, fun: &Function, inner_vars: &mut Vec<&str>, funs: &Functions) {
        for i in 0..fun.compact() {
            let to = self.len() - i;
            let last = Tokens::get_last(&self[0..to], funs);
            let tokens = Tokens(self.drain(last..to).collect());
            let to = self.len() - i;
            self.insert(to, Token::Tokens(tokens));
            inner_vars.pop();
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
impl From<Tokens> for Token {
    fn from(value: Tokens) -> Self {
        Self::Tokens(value)
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
    pub fn num_ref(&self) -> Complex {
        let Token::Num(num) = self else {
            unreachable!()
        };
        *num
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
