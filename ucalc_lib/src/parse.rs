use crate::functions::Function;
use crate::operators::{Bracket, Operators};
use crate::polynomial::Polynomial;
use crate::variable::{Functions, Variables};
use crate::{Number, NumberBase, Variable};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::{fmt, iter};
use ucalc_numbers::FloatTrait;
#[derive(Default, PartialEq, Debug, Clone)]
pub struct Tokens(pub Vec<Token>);
#[derive(Debug, Clone, Copy)]
pub(crate) struct TokensRef<'a>(pub &'a [Token]);
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
    InnerVar(u16),
    GraphVar(u8),
    Fun(u16),
    Var(u16),
    Skip(usize),
    Function(Function),
}
impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Num(n) => write!(f, "{}", n.real()),
            Self::Function(fun) => {
                if let Ok(o) = Operators::try_from(*fun) {
                    write!(f, "{o}")
                } else {
                    write!(f, "{fun}")
                }
            }
            _ => write!(f, "{self:?}"),
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum ParseError<'a> {
    UnknownToken(&'a str),
    LeftParenthesisNotFound,
    RightParenthesisNotFound,
    AbsoluteBracketFailed,
    MissingInput,
    ExtraInput,
    InnerVarError,
    VarExpectedName,
}
impl Display for Tokens {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", TokensRef(self))
    }
}
impl<'a> Display for TokensRef<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
impl Token {
    fn greater_precedence(&self, o: Operators) -> bool {
        match self {
            Token::Num(_) => true,
            Token::Polynomial(_) => {
                unreachable!()
            }
            Token::InnerVar(_) => true,
            Token::GraphVar(_) => true,
            Token::Fun(_) => true,
            Token::Var(_) => true,
            Token::Skip(_) => {
                unreachable!()
            }
            Token::Function(f) => {
                if let Ok(f) = Operators::try_from(*f) {
                    (f == o && f.left_associative()) || f.precedence() > o.precedence()
                } else {
                    true
                }
            }
        }
    }
}
impl<'a> TokensRef<'a> {
    pub fn get_infix(
        self,
        vars: &Variables,
        funs: &Functions,
        graph_vars: &[&str],
    ) -> impl Display {
        fmt::from_fn(move |fmt| match self.last().unwrap() {
            Token::Num(n) => write!(fmt, "{}", n.real()),
            Token::Polynomial(_) => unreachable!(),
            Token::InnerVar(i) => write!(fmt, "{}", (b'n' + *i as u8) as char),
            Token::GraphVar(i) => write!(fmt, "{}", graph_vars[*i as usize]),
            Token::Fun(i) => write!(fmt, "{}", funs[*i as usize].name.as_ref().unwrap()),
            Token::Var(i) => write!(fmt, "{}", vars[*i as usize].name.as_ref().unwrap()),
            Token::Skip(_) => Ok(()),
            Token::Function(f) => {
                let l = self.len() - 1;
                let last = TokensRef(&self[..l]).get_last(funs);
                if let Ok(o) = Operators::try_from(*f) {
                    let arg = TokensRef(&self[last..l]).get_infix(vars, funs, graph_vars);
                    let arg = if self[l - 1].greater_precedence(o)
                        || (f.is_chainable()
                            && if let Token::Function(f) = self[l - 1] {
                                f.is_chainable()
                            } else {
                                false
                            }) {
                        format_args!("{arg}")
                    } else {
                        format_args!("({arg})")
                    };
                    if o.inputs() == 2 {
                        let arg1 = TokensRef(&self[..last]).get_infix(vars, funs, graph_vars);
                        let arg1 = if self[last - 1].greater_precedence(o) {
                            format_args!("{arg1}")
                        } else {
                            format_args!("({arg1})")
                        };
                        write!(fmt, "{arg1}{o}{arg}")
                    } else if o.unary_left() {
                        write!(fmt, "{o}{arg}")
                    } else {
                        write!(fmt, "{arg}{o}")
                    }
                } else {
                    let lasts = self.get_lasts(funs);
                    let mut first = true;
                    write!(fmt, "{f}(")?;
                    for arg in lasts {
                        let arg = arg.get_infix(vars, funs, graph_vars);
                        if first {
                            first = false;
                            write!(fmt, "{arg}")?;
                        } else {
                            write!(fmt, ",{arg}")?;
                        }
                    }
                    write!(fmt, ")")
                }
            }
        })
    }
    pub fn get_rpn(self, vars: &Variables, funs: &Functions, graph_vars: &[&str]) -> impl Display {
        fmt::from_fn(move |fmt| {
            let mut first = true;
            for token in self.iter() {
                if !first {
                    write!(fmt, " ")?;
                }
                first = false;
                match token {
                    Token::Num(n) => write!(fmt, "{}", n.real())?,
                    Token::Polynomial(_) => unreachable!(),
                    Token::InnerVar(i) => write!(fmt, "{}", (b'n' + *i as u8) as char)?,
                    Token::GraphVar(i) => write!(fmt, "{}", graph_vars[*i as usize])?,
                    Token::Fun(i) => write!(fmt, "{}", funs[*i as usize].name.as_ref().unwrap())?,
                    Token::Var(i) => write!(fmt, "{}", vars[*i as usize].name.as_ref().unwrap())?,
                    Token::Function(fun) => {
                        if let Ok(o) = Operators::try_from(*fun) {
                            write!(fmt, "{o}")?
                        } else {
                            write!(fmt, "{fun}")?
                        }
                    }
                    Token::Skip(_) => first = true,
                }
            }
            Ok(())
        })
    }
}
impl Tokens {
    pub fn get_infix(
        &self,
        vars: &Variables,
        funs: &Functions,
        graph_vars: &[&str],
    ) -> impl Display {
        TokensRef(self).get_infix(vars, funs, graph_vars)
    }
    pub fn get_rpn(&self, vars: &Variables, funs: &Functions, graph_vars: &[&str]) -> impl Display {
        TokensRef(self).get_rpn(vars, funs, graph_vars)
    }
    fn end(
        mut self,
        inputs: Option<(&str, bool)>,
        vars: &mut Variables,
        funs: &mut Functions,
    ) -> Option<Self> {
        if self.is_empty() {
            self.push(Token::Num(Number::default()));
        }
        if let Some((name, is_fun)) = inputs {
            if !is_fun {
                let val = self.compute(&[], funs, vars);
                if let Some(v) = vars.position(name) {
                    vars[v as usize].value = val;
                } else {
                    vars.push(Variable::new(name, val));
                }
                funs.iter_mut().for_each(|v| {
                    if v.name.as_ref().is_some_and(|n| n.as_ref() == name) {
                        v.name = None;
                    }
                });
            } else if let Some(v) = funs.position(name) {
                funs[v as usize].tokens = self;
            } else {
                unreachable!()
            }
            None
        } else {
            Some(self)
        }
    }
    pub fn rpn<'a>(
        value: &'a str,
        vars: &mut Variables,
        funs: &mut Functions,
        graph_vars: &[&str],
        mut expect_let: bool,
        base: u8,
    ) -> Result<Option<Self>, ParseError<'a>> {
        let mut tokens = Tokens(Vec::with_capacity(value.len()));
        let mut inner_vars: Vec<&str> = Vec::with_capacity(value.len());
        let mut inputs = None;
        for token in value.split(' ') {
            match token {
                "" => {}
                "let" => expect_let = true,
                "=" if expect_let => {
                    expect_let = false;
                    let Some(name) = inner_vars.pop() else {
                        return Err(ParseError::VarExpectedName);
                    };
                    if !inner_vars.is_empty() {
                        funs.add(vars, name, inner_vars.len().try_into().unwrap());
                        inputs = Some((name, true));
                    } else {
                        inputs = Some((name, false));
                    }
                }
                _ if expect_let && token.chars().all(|c| c.is_ascii_alphabetic()) => {
                    inner_vars.push(token)
                }
                _ if let Ok(operator) = Operators::try_from(token) => tokens.push(operator.into()),
                _ if let Some(i) = funs.position(token) => tokens.push(Token::Fun(i)),
                _ if let Some(i) = inner_vars.iter().position(|v| *v == token) => {
                    tokens.push(Token::InnerVar(i as u16))
                }
                _ if let Some(i) = vars.position(token) => tokens.push(Token::Var(i)),
                _ if let Some(i) = graph_vars.iter().position(|v| v == &token) => {
                    tokens.push(Token::GraphVar(i as u8))
                }
                _ if let Ok(fun) = Function::try_from(token) => {
                    tokens.compact_args(fun, &mut inner_vars, funs);
                    tokens.push(fun.into());
                }
                _ if let Some(i) = token.rfind(|c: char| !c.is_ascii_digit())
                    && let Ok(mut fun) = Function::try_from(&token[..=i]) =>
                {
                    let inputs = token[i + 1..].parse().unwrap();
                    tokens.compact_args(fun, &mut inner_vars, funs);
                    fun.set_inputs(inputs);
                    tokens.push(fun.into());
                }
                _ if token.chars().all(|c| c.is_ascii_alphabetic()) => inner_vars.push(token),
                _ if let Some(n) = NumberBase::parse_radix(token, base) => tokens.push(n.into()),
                _ => return Err(ParseError::UnknownToken(token)),
            }
        }
        Ok(tokens.end(inputs, vars, funs))
    }
    pub fn infix<'a>(
        value: &'a str,
        vars: &mut Variables,
        funs: &mut Functions,
        graph_vars: &[&str],
        mut expect_let: bool,
        base: u8,
    ) -> Result<Option<Self>, ParseError<'a>> {
        let mut tokens = Tokens(Vec::with_capacity(value.len()));
        let mut operator_stack: Vec<Operators> = Vec::with_capacity(value.len());
        let mut inner_vars: Vec<&str> = Vec::with_capacity(value.len());
        let mut fn_inputs: Vec<u8> = Vec::with_capacity(value.len());
        let mut inner_vars_count: Vec<u8> = Vec::with_capacity(value.len());
        let mut chars = value.char_indices();
        let mut inputs = None;
        let mut negate = true;
        let mut last_abs = false;
        let mut req_input = false;
        let mut open_input = false;
        let mut last_mul = false;
        let mut expect_expr = false;
        let mut abs = 0;
        while let Some((i, c)) = chars.next() {
            match c {
                ' ' => {}
                '@' if let Some(i) = vars.position("@") => {
                    tokens.last_mul(&mut operator_stack, negate, &mut last_mul, true);
                    tokens.push(Token::Var(i));
                    open_input = true;
                    negate = false;
                    last_abs = false;
                    req_input = false;
                    expect_expr = false;
                }
                _ if if base > 10 {
                    c.is_ascii_alphanumeric()
                } else {
                    c.is_ascii_alphabetic()
                } =>
                {
                    let mut l = c.len_utf8();
                    let mut count = 1;
                    for t in value[i + l..].chars() {
                        if if base > 10 {
                            t.is_ascii_alphanumeric() || t == '.'
                        } else {
                            t.is_ascii_alphabetic()
                        } || t == '_'
                        {
                            l += t.len_utf8();
                            count += 1;
                        } else {
                            break;
                        }
                    }
                    let o = l;
                    loop {
                        let s = &value[i..i + l];
                        if i == 0 && s == "let" {
                            expect_let = true;
                            open_input = false;
                        } else if expect_let && s.chars().all(|c| c.is_ascii_alphabetic()) {
                            inner_vars.push(s);
                            open_input = true;
                        } else if let Some(i) = funs.position(s) {
                            tokens.last_mul(&mut operator_stack, negate, &mut last_mul, false);
                            operator_stack.push(Operators::Function(Function::Custom(i)));
                            open_input = false;
                            fn_inputs.push(1);
                        } else if let Some(i) = inner_vars.iter().position(|v| *v == s) {
                            tokens.last_mul(&mut operator_stack, negate, &mut last_mul, true);
                            tokens.push(Token::InnerVar(i as u16));
                            open_input = true;
                        } else if let Some(i) = vars.position(s) {
                            tokens.last_mul(&mut operator_stack, negate, &mut last_mul, true);
                            tokens.push(Token::Var(i));
                            open_input = true;
                        } else if let Some(i) = graph_vars.iter().position(|v| v == &s) {
                            tokens.last_mul(&mut operator_stack, negate, &mut last_mul, true);
                            tokens.push(Token::GraphVar(i as u8));
                            open_input = true;
                        } else if let Ok(fun) = Function::try_from(s) {
                            if fun.first_expected_var(1) {
                                inner_vars.extend(iter::repeat_n("", fun.inner_vars() as usize));
                            }
                            if fun.has_var() {
                                inner_vars_count.push(fun.inner_vars());
                            }
                            tokens.last_mul(&mut operator_stack, negate, &mut last_mul, false);
                            operator_stack.push(Operators::Function(fun));
                            open_input = false;
                            fn_inputs.push(1);
                        } else if s.chars().all(|c| c.is_ascii_alphabetic())
                            && !inner_vars_count.is_empty()
                            && let Some(n) = Tokens::get_var_position(
                                &mut inner_vars_count,
                                &fn_inputs,
                                &operator_stack,
                                l,
                                inner_vars.len(),
                            )?
                        {
                            tokens.last_mul(&mut operator_stack, negate, &mut last_mul, true);
                            tokens.push(Token::InnerVar(n as u16));
                            inner_vars[n] = s;
                            open_input = true;
                        } else if let Some(f) = NumberBase::parse_radix(s, base) {
                            tokens.last_mul(&mut operator_stack, negate, &mut last_mul, true);
                            tokens.push(f.into());
                            let _ = chars.advance_by(l - 1);
                            open_input = true;
                        } else if count != 1 {
                            count -= 1;
                            l -= value[i..i + l].chars().last().unwrap().len_utf8();
                            continue;
                        } else {
                            return Err(ParseError::UnknownToken(&value[i..i + o]));
                        }
                        break;
                    }
                    let _ = chars.advance_by(count - 1);
                    negate = false;
                    last_abs = false;
                    req_input = false;
                    expect_expr = !open_input;
                }
                '0'..='9' if base <= 10 => {
                    let mut l = 1;
                    for t in value[i + 1..].chars() {
                        if t.is_ascii_digit() || t == '.' {
                            l += 1;
                        } else {
                            break;
                        }
                    }
                    let s = &value[i..i + l];
                    let Some(float) = NumberBase::parse_radix(s, base) else {
                        return Err(ParseError::UnknownToken(s));
                    };
                    tokens.last_mul(&mut operator_stack, negate, &mut last_mul, true);
                    tokens.push(float.into());
                    let _ = chars.advance_by(l - 1);
                    negate = false;
                    last_abs = false;
                    req_input = false;
                    open_input = true;
                    expect_expr = false;
                }
                ',' => {
                    if req_input || expect_expr {
                        return Err(ParseError::MissingInput);
                    }
                    while let Some(top) = operator_stack.last()
                        && !matches!(top, Operators::Bracket(_))
                    {
                        tokens.push(operator_stack.pop().unwrap().into());
                    }
                    if let Some(last) = fn_inputs.last_mut() {
                        *last += 1;
                        let Operators::Function(fun) = operator_stack[operator_stack.len() - 2]
                        else {
                            unreachable!()
                        };
                        if fun.first_expected_var(*last) {
                            inner_vars.extend(iter::repeat_n("", fun.inner_vars() as usize));
                        }
                    }
                    negate = true;
                    last_abs = false;
                    last_mul = false;
                    open_input = false;
                    expect_expr = true;
                }
                ')' => {
                    if req_input || expect_expr {
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
                    tokens.close_off_bracket(
                        &mut operator_stack,
                        &mut inner_vars,
                        &mut inner_vars_count,
                        funs,
                        &mut fn_inputs,
                    )?;
                    last_mul = true;
                    negate = false;
                    last_abs = false;
                    open_input = true;
                    expect_expr = false;
                }
                '(' => {
                    tokens.last_mul(&mut operator_stack, negate, &mut last_mul, true);
                    operator_stack.push(Bracket::Parenthesis.into());
                    negate = true;
                    last_abs = false;
                    req_input = false;
                    last_mul = false;
                    open_input = false;
                    expect_expr = true;
                }
                '|' => {
                    if abs == 0 || last_abs || req_input {
                        operator_stack.push(Bracket::Absolute.into());
                        abs += 1;
                        negate = true;
                        last_abs = true;
                        req_input = false;
                        last_mul = false;
                        open_input = false;
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
                        tokens.push(Function::Abs.into());
                        tokens.close_off_bracket(
                            &mut operator_stack,
                            &mut inner_vars,
                            &mut inner_vars_count,
                            funs,
                            &mut fn_inputs,
                        )?;
                        abs -= 1;
                        last_mul = true;
                        negate = false;
                        last_abs = false;
                        open_input = true;
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
                    if expect_let && s == "=" {
                        open_input = false;
                        expect_let = false;
                        let Some(name) = inner_vars.try_remove(0) else {
                            return Err(ParseError::VarExpectedName);
                        };
                        if !inner_vars.is_empty() {
                            funs.add(vars, name, inner_vars.len().try_into().unwrap());
                            inputs = Some((name, true));
                        } else {
                            inputs = Some((name, false));
                        }
                    } else if let Ok(mut operator) = Operators::try_from(s) {
                        if negate {
                            match operator {
                                Operators::Sub => operator = Operators::Negate,
                                Operators::Factorial => operator = Operators::SubFactorial,
                                _ => {}
                            }
                        }
                        if operator.inputs() == 2 {
                            req_input = true;
                            if !open_input {
                                return Err(ParseError::MissingInput);
                            }
                        } else if operator.unary_left() {
                            req_input = true;
                        }
                        tokens.pop_stack(&mut operator_stack, operator, negate);
                        negate = operator != Operators::Factorial;
                        last_abs = false;
                    } else {
                        return Err(ParseError::UnknownToken(s));
                    }
                    last_mul = false;
                }
            }
        }
        if req_input || expect_expr {
            return Err(ParseError::MissingInput);
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
        Ok(tokens.end(inputs, vars, funs))
    }
    pub(crate) fn get_var_position(
        inner_vars_count: &mut [u8],
        fn_inputs: &[u8],
        operator_stack: &[Operators],
        l: usize,
        mut inner_vars: usize,
    ) -> Result<Option<usize>, ParseError<'static>> {
        let mut n = inner_vars_count.len();
        let mut inputs = fn_inputs.iter();
        let mut last = None;
        if operator_stack
            .iter()
            .rfind(|la| {
                let Operators::Function(f) = la else {
                    return false;
                };
                last = inputs.next_back();
                if !f.has_var() {
                    return false;
                }
                n -= 1;
                if !f.expected_var(*last.unwrap()) {
                    return false;
                }
                if inner_vars_count[n] != 0 {
                    inner_vars -= inner_vars_count[n] as usize;
                    true
                } else {
                    inner_vars -= f.inner_vars() as usize;
                    false
                }
            })
            .is_some()
        {
            inner_vars_count[n] -= 1;
            Ok(Some(inner_vars))
        } else if l == 1 {
            Err(ParseError::InnerVarError)
        } else {
            Ok(None)
        }
    }
    pub fn last_mul(
        &mut self,
        operator_stack: &mut Vec<Operators>,
        negate: bool,
        last_mul: &mut bool,
        new: bool,
    ) {
        if *last_mul {
            self.pop_stack(operator_stack, Operators::Mul, negate);
        }
        *last_mul = new;
    }
    pub fn pop_stack(
        &mut self,
        operator_stack: &mut Vec<Operators>,
        operator: Operators,
        negate: bool,
    ) {
        while let Some(top) = operator_stack.last()
            && !matches!(top, Operators::Bracket(_))
            && (top.precedence() > operator.precedence()
                || (top.precedence() == operator.precedence() && operator.left_associative()))
            && !(negate && operator == Operators::Negate && *top == Operators::Pow)
        {
            self.push(operator_stack.pop().unwrap().into());
        }
        operator_stack.push(operator);
    }
    pub fn close_off_bracket(
        &mut self,
        operator_stack: &mut Vec<Operators>,
        inner_vars: &mut Vec<&str>,
        inner_vars_count: &mut Vec<u8>,
        funs: &Functions,
        fn_inputs: &mut Vec<u8>,
    ) -> Result<(), ParseError<'static>> {
        if let Some(top) = operator_stack.pop_if(|l| matches!(l, Operators::Function(_))) {
            match top {
                Operators::Function(Function::Custom(i)) => {
                    let inputs = fn_inputs.pop().unwrap();
                    match funs.get(i as usize).unwrap().inputs.cmp(&inputs) {
                        Ordering::Greater => return Err(ParseError::MissingInput),
                        Ordering::Less => return Err(ParseError::ExtraInput),
                        _ => {}
                    }
                    self.push(Token::Fun(i));
                }
                Operators::Function(mut fun) => {
                    if fun.has_var() {
                        inner_vars_count.pop().unwrap();
                    }
                    let mut inputs = fn_inputs.pop().unwrap();
                    if fun.inputs() + 1 - fun.inner_vars() < inputs {
                        let last = TokensRef(self).get_last(funs);
                        let mut t = last;
                        for _ in fun.inputs()..inputs {
                            let last = TokensRef(&self[0..t]).get_last(funs);
                            if t - 1 == last && matches!(self[last], Token::InnerVar(_)) {
                                self.remove(last);
                            } else {
                                return Err(ParseError::InnerVarError);
                            }
                            t = last;
                        }
                        inputs = fun.inputs();
                    }
                    fun.set_inputs(inputs);
                    match fun.inputs().cmp(&inputs) {
                        Ordering::Greater => return Err(ParseError::MissingInput),
                        Ordering::Less => return Err(ParseError::ExtraInput),
                        _ => {}
                    }
                    self.compact_args(fun, inner_vars, funs);
                    self.push(fun.into());
                }
                _ => {}
            }
        }
        Ok(())
    }
    pub fn compact_args(&mut self, fun: Function, inner_vars: &mut Vec<&str>, funs: &Functions) {
        let mut t = self.len();
        for _ in 0..fun.compact() {
            let last = TokensRef(&self[0..t]).get_last(funs);
            self.insert(last, Token::Skip(t - last));
            t = last;
        }
        for _ in 0..fun.inner_vars() {
            inner_vars.pop().unwrap();
        }
    }
}
impl<'a> TokensRef<'a> {
    pub fn get_last_with_end(self, funs: &Functions, mut end: usize) -> usize {
        let mut inputs = 1;
        while inputs != 0 {
            inputs -= 1;
            end -= 1;
            match self[end] {
                Token::Fun(j) => inputs += funs[j as usize].inputs,
                Token::Function(o) => inputs += o.inputs(),
                Token::Skip(_) => inputs += 1,
                _ => {}
            }
        }
        end
    }
    pub fn get_last(self, funs: &Functions) -> usize {
        self.get_last_with_end(funs, self.len())
    }
    pub fn get_from_last_with_end(self, funs: &Functions, end: usize) -> (Self, usize) {
        let last = self.get_last_with_end(funs, end);
        if matches!(self[end - 1], Token::Skip(_)) {
            (Self(&self.0[last..end - 1]), last)
        } else {
            (Self(&self.0[last..end]), last)
        }
    }
    pub fn get_from_last(self, funs: &Functions) -> (Self, usize) {
        self.get_from_last_with_end(funs, self.len())
    }
    pub fn get_lasts(self, funs: &Functions) -> Vec<Self> {
        let inputs = match self.last().unwrap() {
            Token::Fun(j) => funs[*j as usize].inputs,
            Token::Function(o) => o.inputs(),
            _ => unreachable!(),
        };
        let mut ret = vec![TokensRef(&[]); inputs as usize];
        let mut end = self.len() - 1;
        for j in (0..inputs).rev() {
            (ret[j as usize], end) = self.get_from_last_with_end(funs, end);
        }
        ret
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
impl From<Operators> for Token {
    fn from(value: Operators) -> Self {
        Self::Function(value.into())
    }
}
impl From<Function> for Token {
    fn from(value: Function) -> Self {
        Self::Function(value)
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
    pub fn inner_var(self) -> u16 {
        let Token::InnerVar(n) = self else {
            unreachable!()
        };
        n
    }
    pub fn inner_var_ref(&self) -> u16 {
        let Token::InnerVar(n) = self else {
            unreachable!()
        };
        *n
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
