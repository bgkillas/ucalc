use crate::functions::Function;
use crate::operators::{Bracket, Operators};
use crate::polynomial::Polynomial;
use crate::variable::{Functions, Variables};
use crate::{Number, NumberBase};
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use ucalc_numbers::FloatTrait;
#[derive(Default, PartialEq, Debug, Clone)]
pub struct Tokens(pub Vec<Token>);
#[derive(Debug, Clone)]
pub struct TokensRef<'a>(pub &'a [Token]);
impl<'a> From<&'a Tokens> for TokensRef<'a> {
    fn from(value: &'a Tokens) -> Self {
        Self(value)
    }
}
impl<'a> From<&'a [Token]> for TokensRef<'a> {
    fn from(value: &'a [Token]) -> Self {
        Self(value)
    }
}
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Num(Number),
    Polynomial(Box<Polynomial>),
    InnerVar(usize),
    GraphVar(usize),
    Fun(usize),
    Skip(usize),
    Operator(Operators),
}
impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Num(n) => write!(f, "{}", n),
            Self::Operator(Operators::Function(fun)) => write!(f, "{:?}", fun),
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
        write!(f, "{}", TokensRef(self))
    }
}
impl<'a> Display for TokensRef<'a> {
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
    pub fn rpn(
        value: &str,
        vars: &Variables,
        graph_vars: &[&str],
        funs: &Functions,
    ) -> Result<Self, ParseError> {
        let mut tokens = Tokens(Vec::with_capacity(value.len()));
        let mut inner_vars: Vec<&str> = Vec::with_capacity(value.len());
        for token in value.split(' ') {
            if token.is_empty() {
                continue;
            }
            if let Ok(operator) = Operators::try_from(token) {
                tokens.push(operator.into());
            } else if let Some(n) = NumberBase::parse_radix(token, 10) {
                tokens.push(n.into());
            } else if let Some(i) = funs.iter().position(|v| v.name == token) {
                tokens.push(Token::Fun(i))
            } else if let Ok(fun) = Function::try_from(token) {
                tokens.compact_args(&fun, &mut inner_vars, funs);
                tokens.push(fun.into());
            } else if let Some(i) = inner_vars.iter().position(|v| *v == token) {
                tokens.push(Token::InnerVar(i));
            } else if let Some(v) = vars.iter().find(|v| v.name == token) {
                tokens.push(Token::Num(v.value.clone()));
            } else if let Some(i) = graph_vars.iter().position(|v| v == &token) {
                tokens.push(Token::GraphVar(i));
            } else if token.chars().all(|c| c.is_ascii_alphabetic()) {
                inner_vars.push(token);
            } else {
                return Err(ParseError::UnknownToken(token.to_string()));
            }
        }
        Ok(tokens)
    }
    pub fn infix(
        value: &str,
        vars: &Variables,
        graph_vars: &[&str],
        funs: &Functions,
    ) -> Result<Self, ParseError> {
        let mut tokens = Tokens(Vec::with_capacity(value.len()));
        let mut operator_stack: Vec<Operators> = Vec::with_capacity(value.len());
        let mut inner_vars: Vec<&str> = Vec::with_capacity(value.len());
        let mut chars = value.char_indices();
        let mut negate = true;
        let mut last_abs = false;
        let mut req_input = false;
        let mut expect_var = 0;
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
                    if let Some(i) = funs.iter().position(|v| v.name == s) {
                        operator_stack.push(Function::Custom(i).into());
                    } else if let Ok(fun) = Function::try_from(s) {
                        expect_var = fun.inner_vars();
                        operator_stack.push(fun.into());
                    } else if expect_var > 0 {
                        inner_vars.push(s);
                        expect_var -= 1;
                    } else if let Some(i) = inner_vars.iter().position(|v| *v == s) {
                        tokens.push(Token::InnerVar(i));
                    } else if let Some(v) = vars.iter().find(|v| v.name == s) {
                        tokens.push(Token::Num(v.value.clone()));
                    } else if let Some(i) = graph_vars.iter().position(|v| v == &s) {
                        tokens.push(Token::GraphVar(i));
                    } else {
                        return Err(ParseError::UnknownToken(s.to_string()));
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
                    let Some(float) = NumberBase::parse_radix(s, 10) else {
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
                        return Err(ParseError::UnknownToken(s.to_string()));
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
        let mut t = 0;
        for _ in 0..fun.compact() {
            let to = self.len() - t;
            let last = TokensRef(&self[0..to]).get_last(funs);
            self.insert(last, Token::Skip(to - last));
            t += to - last + 1;
            inner_vars.pop();
        }
    }
}
impl<'a> TokensRef<'a> {
    pub fn get_last(&self, funs: &Functions) -> usize {
        match self.last().unwrap() {
            Token::Fun(i) => {
                let inputs = funs[*i].inputs;
                let mut i = self.len() - 1;
                for _ in 0..inputs {
                    i = TokensRef(&self[..i]).get_last(funs)
                }
                i
            }
            Token::Operator(o) => {
                let inputs = o.inputs();
                let mut i = self.len() - 1;
                for _ in 0..inputs {
                    i = TokensRef(&self[..i]).get_last(funs)
                }
                i
            }
            Token::Skip(_) => TokensRef(&self[..self.len() - 1]).get_last(funs),
            _ => self.len() - 1,
        }
    }
    pub fn get_from_last(&'a self, funs: &Functions) -> (Self, usize) {
        let last = self.get_last(funs);
        (Self(&self[last..]), last)
    }
}
impl From<Number> for Token {
    fn from(value: Number) -> Self {
        Self::Num(value)
    }
}
#[cfg(any(
    feature = "list",
    feature = "vector",
    feature = "matrix",
    feature = "units"
))]
impl From<NumberBase> for Token {
    fn from(value: NumberBase) -> Self {
        Self::Num(value.into())
    }
}
impl From<Operators> for Token {
    fn from(value: Operators) -> Self {
        Self::Operator(value)
    }
}
impl From<Function> for Token {
    fn from(value: Function) -> Self {
        Self::Operator(value.into())
    }
}
impl From<Polynomial> for Token {
    fn from(value: Polynomial) -> Self {
        Self::Polynomial(value.into())
    }
}
impl Token {
    pub fn num(self) -> Number {
        let Token::Num(num) = self else {
            unreachable!()
        };
        num
    }
    pub fn skip(&self) -> usize {
        let Token::Skip(num) = self else {
            unreachable!()
        };
        *num
    }
    pub fn num_ref(&self) -> &Number {
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
    pub fn num_mut(&mut self) -> &mut Number {
        let Token::Num(num) = self else {
            unreachable!()
        };
        num
    }
    pub fn poly_mut(&mut self) -> &mut Polynomial {
        let Token::Polynomial(poly) = self else {
            unreachable!()
        };
        poly
    }
    pub fn poly(self) -> Box<Polynomial> {
        let Token::Polynomial(poly) = self else {
            unreachable!()
        };
        poly
    }
    pub fn poly_ref(&self) -> &Polynomial {
        let Token::Polynomial(poly) = self else {
            unreachable!()
        };
        poly
    }
}
impl Deref for Tokens {
    type Target = Vec<Token>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> Deref for TokensRef<'a> {
    type Target = [Token];
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl DerefMut for Tokens {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
