#[cfg(feature = "units")]
use crate::UNITS;
use crate::functions::Function;
use crate::operators::{Bracket, Operator};
#[cfg(feature = "float_rand")]
use crate::rand::Rand;
use crate::variable::{Functions, Variables};
use crate::{FunctionVar, Number, NumberBase, Variable};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::marker::ConstParamTy;
use std::num::NonZeroU8;
use std::ops::{
    Deref, DerefMut, Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo,
    RangeToInclusive,
};
use std::{fmt, iter, mem};
#[cfg(feature = "complex")]
use ucalc_numbers::ComplexFunctionsMut;
#[cfg(feature = "units")]
use ucalc_numbers::Units;
use ucalc_numbers::{Float, FloatFunctions, FloatTrait};
#[derive(Default, PartialEq, Debug, Clone)]
#[repr(transparent)]
pub struct Tokens(pub Vec<Token>);
#[derive(Debug)]
#[repr(transparent)]
pub struct TokensSlice(pub [Token]);
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(Number),
    InnerVar(u16),
    GraphVar(u8),
    CustomVar(u16),
    CustomFun(u16, Derivative),
    Function(Function, Derivative),
    Skip(usize),
}
#[derive(Debug, PartialEq, Clone, Copy, Default)]
#[repr(transparent)]
pub struct Derivative(u8);
#[derive(Debug, PartialEq, Clone, Copy, Eq, ConstParamTy)]
pub enum Volatility {
    Constant,
    GraphConstant,
    Volatile,
}
impl PartialOrd for Volatility {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match (self, other) {
            (Self::Constant, Self::Constant) => Ordering::Equal,
            (Self::GraphConstant, Self::GraphConstant) => Ordering::Equal,
            (Self::Volatile, Self::Volatile) => Ordering::Equal,
            (Self::Volatile, _) => Ordering::Greater,
            (Self::Constant, _) => Ordering::Less,
            (Self::GraphConstant, Self::Constant) => Ordering::Greater,
            (Self::GraphConstant, Self::Volatile) => Ordering::Less,
        })
    }
}
#[derive(PartialEq)]
pub(crate) enum NewCustom<'a> {
    Var(&'a str),
    Fun(bool, Option<(u16, Option<Box<str>>)>),
}
#[derive(PartialEq, Debug)]
pub enum ParseReturn {
    Tokens(Tokens),
    Graph(Tokens, Vec<bool>),
    Var,
}
impl ParseReturn {
    pub fn tokens(self) -> Tokens {
        let Self::Tokens(tokens) = self else {
            unreachable!()
        };
        tokens
    }
    pub fn tokens_any(self) -> Tokens {
        match self {
            Self::Tokens(t) => t,
            Self::Graph(t, _) => t,
            Self::Var => unreachable!(),
        }
    }
    pub fn is_var(&self) -> bool {
        matches!(self, Self::Var)
    }
}
impl Tokens {
    #[allow(clippy::too_many_arguments)]
    pub fn parse<'a>(
        value: &'a str,
        vars: &mut Variables,
        funs: &mut Functions,
        graph_vars: &[&str],
        expect_let: bool,
        simplify: bool,
        base: u8,
        rpn: bool,
        #[cfg(feature = "float_rand")] rand: &mut Rand,
    ) -> Result<ParseReturn, ParseError<'a>> {
        if rpn {
            Self::rpn(
                value,
                vars,
                funs,
                graph_vars,
                expect_let,
                simplify,
                base,
                #[cfg(feature = "float_rand")]
                rand,
            )
        } else {
            Self::infix(
                value,
                vars,
                funs,
                graph_vars,
                expect_let,
                simplify,
                base,
                #[cfg(feature = "float_rand")]
                rand,
            )
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn rpn<'a>(
        value: &'a str,
        vars: &mut Variables,
        funs: &mut Functions,
        graph_vars: &[&str],
        mut expect_let: bool,
        simplify: bool,
        base: u8,
        #[cfg(feature = "float_rand")] rand: &mut Rand,
    ) -> Result<ParseReturn, ParseError<'a>> {
        let mut tokens = Tokens(Vec::with_capacity(value.len()));
        let mut has_graph_vars = vec![false; graph_vars.len()];
        let mut inputs = None;
        let inner = try {
            let mut inner_vars: Vec<&str> = Vec::with_capacity(value.len());
            let mut open_inputs: usize = 0;
            for token in value.split(' ') {
                match token {
                    "" | "(" | ")" => {}
                    "let" => expect_let = true,
                    "=" if expect_let => {
                        expect_let = false;
                        let Some(name) = inner_vars.pop() else {
                            return Err(ParseError::VarExpectedName);
                        };
                        if !inner_vars.is_empty() {
                            let (a, b) = funs.add(
                                vars,
                                name,
                                NonZeroU8::new(inner_vars.len() as u8).unwrap(),
                            );
                            inputs = Some(NewCustom::Fun(a, b));
                        } else {
                            inner_vars.pop();
                            if !inner_vars.is_empty() {
                                return Err(ParseError::InnerVarError);
                            }
                            inputs = Some(NewCustom::Var(name));
                        }
                    }
                    "=" => return Err(ParseError::RpnUnsupported),
                    _ if expect_let && token.chars().all(|c| c.is_alphabetic()) => {
                        inner_vars.push(token)
                    }
                    _ if let Ok(operator) = Operator::try_from(token) => {
                        open_inputs = open_inputs
                            .checked_sub(operator.inputs().get() as usize - 1)
                            .ok_or(ParseError::ExtraInput)?;
                        tokens.push(operator.into())
                    }
                    _ if let Some(i) = funs.position(token) => {
                        open_inputs = open_inputs
                            .checked_sub(funs[i as usize].inputs.get() as usize - 1)
                            .ok_or(ParseError::ExtraInput)?;
                        tokens.push(Token::CustomFun(i, Derivative::default()))
                    }
                    _ if let Some(i) = inner_vars.iter().copied().position(|v| v == token) => {
                        open_inputs += 1;
                        tokens.push(Token::InnerVar(i as u16))
                    }
                    _ if let Some(i) = vars.position(token) => {
                        open_inputs += 1;
                        tokens.push(Token::CustomVar(i))
                    }
                    _ if let Some(i) = graph_vars.iter().copied().position(|v| v == token) => {
                        if matches!(inputs, Some(NewCustom::Var(_))) {
                            return Err(ParseError::GraphVarError);
                        }
                        has_graph_vars[i] = true;
                        open_inputs += 1;
                        tokens.push(Token::GraphVar(i as u8))
                    }
                    #[cfg(feature = "complex")]
                    "z" if let Some(i) = graph_vars.iter().copied().position(|v| v == "x")
                        && let Some(j) = graph_vars.iter().copied().position(|v| v == "y") =>
                    {
                        if matches!(inputs, Some(NewCustom::Var(_))) {
                            return Err(ParseError::GraphVarError);
                        }
                        has_graph_vars[i] = true;
                        has_graph_vars[j] = true;
                        open_inputs += 1;
                        tokens.push(Token::GraphVar(i as u8));
                        tokens.push(Token::GraphVar(j as u8));
                        tokens.push(Function::Addi.into());
                    }
                    _ if let Ok(fun) = Function::try_from(token) => {
                        open_inputs = open_inputs
                            .checked_sub(fun.inputs().get() as usize - 1)
                            .ok_or(ParseError::MissingInput)?;
                        tokens.compact_args(fun, &mut inner_vars, funs);
                        tokens.push(fun.into());
                    }
                    _ if let Some(k) = token
                        .rfind(|c: char| c != '\'')
                        .map(|k| token.len() - (k + 1))
                        && let Some(j) = token
                            .rfind(|c: char| c != '`')
                            .map(|j| token.len() - (j + 1))
                        && ((k > 0 && j == 0) || (j > 0 && k == 0) || (k == 0 && j == 0))
                        && let Ok(fun) = Function::try_from(&token[..token.len() - (j + k)]) =>
                    {
                        open_inputs = open_inputs
                            .checked_sub(fun.inputs().get() as usize - 1)
                            .ok_or(ParseError::MissingInput)?;
                        let mut d = Derivative::from((j + k) as u8)?;
                        tokens.compact_args(fun, &mut inner_vars, funs);
                        if j > 0 {
                            d.set_integral()
                        }
                        tokens.push(Token::Function(fun, d));
                    }
                    _ if let Some(k) = token
                        .rfind(|c: char| c != '\'')
                        .map(|k| token.len() - (k + 1))
                        && let Some(j) = token
                            .rfind(|c: char| c != '`')
                            .map(|j| token.len() - (j + 1))
                        && ((k > 0 && j == 0) || (j > 0 && k == 0) || (k == 0 && j == 0))
                        && let Some(i) =
                            token[..token.len() - (j + k)].rfind(|c: char| !c.is_ascii_digit())
                        && let Ok(mut fun) = Function::try_from(&token[..=i]) =>
                    {
                        let inputs = token[i + 1..token.len() - (j + k)].parse().unwrap();
                        fun.set_inputs(inputs);
                        open_inputs = open_inputs
                            .checked_sub(fun.inputs().get() as usize - 1)
                            .ok_or(ParseError::MissingInput)?;
                        let mut d = Derivative::from((j + k) as u8)?;
                        tokens.compact_args(fun, &mut inner_vars, funs);
                        if j > 0 {
                            if inputs.get() == 2 * fun.inputs().get() {
                                d.set_integral_twice_input()
                            } else {
                                d.set_integral()
                            }
                        }
                        tokens.push(Token::Function(fun, d));
                    }
                    _ if token.chars().all(|c| c.is_alphabetic()) => inner_vars.push(token),
                    #[cfg(feature = "units")]
                    _ if let Some(i) = UNITS.iter().copied().position(|s| s == token) => {
                        open_inputs += 1;
                        tokens.push(Units::from(i).into())
                    }
                    _ if token.starts_with("0b")
                        && let Some(n) = NumberBase::parse_radix(&token[2..], 2) =>
                    {
                        open_inputs += 1;
                        tokens.push(n.into())
                    }
                    _ if token.starts_with("0o")
                        && let Some(n) = NumberBase::parse_radix(&token[2..], 8) =>
                    {
                        open_inputs += 1;
                        tokens.push(n.into())
                    }
                    _ if token.starts_with("0x")
                        && let Some(n) = NumberBase::parse_radix(&token[2..], 16) =>
                    {
                        open_inputs += 1;
                        tokens.push(n.into())
                    }
                    _ if let Some(n) = NumberBase::parse_radix(token, base) => {
                        open_inputs += 1;
                        tokens.push(n.into())
                    }
                    _ => return Err(ParseError::UnknownToken(token)),
                }
            }
            if open_inputs > 1 || (open_inputs == 0 && !tokens.is_empty()) {
                return Err(ParseError::MissingInput);
            }
        };
        if let Err(e) = inner {
            if let Some(NewCustom::Fun(b, v)) = inputs {
                if b {
                    funs.pop();
                }
                if let Some((v, n)) = v {
                    funs[v as usize].name = n
                }
            }
            return Err(e);
        }
        Ok(tokens.end(
            inputs,
            simplify,
            vars,
            funs,
            has_graph_vars,
            #[cfg(feature = "float_rand")]
            rand,
        ))
    }
    #[allow(clippy::too_many_arguments)]
    pub fn infix<'a>(
        value: &'a str,
        vars: &mut Variables,
        funs: &mut Functions,
        graph_vars: &[&str],
        mut expect_let: bool,
        simplify: bool,
        base: u8,
        #[cfg(feature = "float_rand")] rand: &mut Rand,
    ) -> Result<ParseReturn, ParseError<'a>> {
        let mut inputs = None;
        let mut tokens = Tokens(Vec::with_capacity(value.len()));
        let mut has_graph_vars = vec![false; graph_vars.len()];
        let inner = try {
            let mut operator_stack: Vec<Operator> = Vec::with_capacity(value.len());
            let mut inner_vars: Vec<&str> = Vec::with_capacity(value.len());
            let mut fn_inputs: Vec<NonZeroU8> = Vec::with_capacity(value.len());
            let mut inner_vars_count: Vec<u8> = Vec::with_capacity(value.len());
            let mut chars = value.char_indices();
            let mut no_input_left = true;
            let mut last_open = false;
            let mut req_input = false;
            let mut open_input = false;
            let mut last_mul = false;
            let mut expect_expr = false;
            let mut abs = 0;
            let mut needs_bracket = false;
            while let Some((i, c)) = chars.next() {
                if needs_bracket && c != '(' {
                    return Err(ParseError::NeedsBracket);
                }
                needs_bracket = false;
                match c {
                    ' ' => {}
                    '@' if let Some(i) = vars.position("@") => {
                        tokens.last_mul(&mut operator_stack, no_input_left, &mut last_mul, true);
                        tokens.push(Token::CustomVar(i));
                        open_input = true;
                        no_input_left = false;
                        last_open = false;
                        req_input = false;
                        expect_expr = false;
                    }
                    '0' if base <= 10
                        && let Some(c) = value[i + 1..].chars().next()
                        && matches!(c, 'b' | 'o' | 'x') =>
                    {
                        let s;
                        let mut l = 0;
                        let Some(float) = (match c {
                            'b' => {
                                for t in value[i + 2..].chars() {
                                    if matches!(t, '0'..='1' | '.') {
                                        l += 1;
                                    } else {
                                        break;
                                    }
                                }
                                s = &value[i + 2..i + 2 + l];
                                NumberBase::parse_radix(s, 2)
                            }
                            'o' => {
                                for t in value[i + 2..].chars() {
                                    if matches!(t, '0'..='7' | '.') {
                                        l += 1;
                                    } else {
                                        break;
                                    }
                                }
                                s = &value[i + 2..i + 2 + l];
                                NumberBase::parse_radix(s, 8)
                            }
                            'x' => {
                                for t in value[i + 2..].chars() {
                                    if matches!(t, '0'..='9' | 'a'..='f' | '.') {
                                        l += 1;
                                    } else {
                                        break;
                                    }
                                }
                                s = &value[i + 2..i + 2 + l];
                                NumberBase::parse_radix(s, 16)
                            }
                            _ => unreachable!(),
                        }) else {
                            return Err(ParseError::UnknownToken(s));
                        };
                        tokens.last_mul(&mut operator_stack, no_input_left, &mut last_mul, true);
                        tokens.push(float.into());
                        chars.advance_by(l + 1).unwrap();
                        no_input_left = false;
                        last_open = false;
                        req_input = false;
                        open_input = true;
                        expect_expr = false;
                    }
                    _ if if base > 10 {
                        c.is_alphabetic() || c.is_ascii_digit()
                    } else {
                        c.is_alphabetic()
                    } =>
                    {
                        let mut l = c.len_utf8();
                        let mut count = 1;
                        for t in value[i + l..].chars() {
                            if if base > 10 {
                                t.is_alphabetic() || t.is_ascii_digit() || t == '.'
                            } else {
                                t.is_alphabetic()
                            } || t == '_'
                            {
                                l += t.len_utf8();
                                count += 1;
                            } else {
                                break;
                            }
                        }
                        loop {
                            let s = &value[i..i + l];
                            if i == 0 && s == "let" {
                                expect_let = true;
                                open_input = false;
                            } else if expect_let && s.chars().all(|c| c.is_alphabetic()) {
                                inner_vars.push(s);
                                open_input = true;
                            } else if let Some(i) = funs.position(s) {
                                if funs[i as usize].inputs.get() > 1 {
                                    needs_bracket = true;
                                }
                                tokens.last_mul(
                                    &mut operator_stack,
                                    no_input_left,
                                    &mut last_mul,
                                    false,
                                );
                                operator_stack.push(Operator::Custom(i, Derivative::default()));
                                open_input = false;
                                fn_inputs.push(NonZeroU8::new(1).unwrap());
                            } else if let Some(i) = inner_vars.iter().copied().position(|v| v == s)
                            {
                                tokens.last_mul(
                                    &mut operator_stack,
                                    no_input_left,
                                    &mut last_mul,
                                    true,
                                );
                                tokens.push(Token::InnerVar(i as u16));
                                open_input = true;
                            } else if let Some(i) = vars.position(s) {
                                tokens.last_mul(
                                    &mut operator_stack,
                                    no_input_left,
                                    &mut last_mul,
                                    true,
                                );
                                tokens.push(Token::CustomVar(i));
                                open_input = true;
                            } else if let Some(i) = graph_vars.iter().copied().position(|v| v == s)
                            {
                                if matches!(inputs, Some(NewCustom::Var(_))) {
                                    return Err(ParseError::GraphVarError);
                                }
                                has_graph_vars[i] = true;
                                tokens.last_mul(
                                    &mut operator_stack,
                                    no_input_left,
                                    &mut last_mul,
                                    true,
                                );
                                tokens.push(Token::GraphVar(i as u8));
                                open_input = true;
                            } else if cfg!(feature = "complex")
                                && s == "z"
                                && let Some(i) = graph_vars.iter().copied().position(|v| v == "x")
                                && let Some(j) = graph_vars.iter().copied().position(|v| v == "y")
                            {
                                if matches!(inputs, Some(NewCustom::Var(_))) {
                                    return Err(ParseError::GraphVarError);
                                }
                                has_graph_vars[i] = true;
                                has_graph_vars[j] = true;
                                tokens.last_mul(
                                    &mut operator_stack,
                                    no_input_left,
                                    &mut last_mul,
                                    true,
                                );
                                tokens.push(Token::GraphVar(i as u8));
                                tokens.push(Token::GraphVar(j as u8));
                                #[cfg(feature = "complex")]
                                tokens.push(Function::Addi.into());
                                open_input = true;
                            } else if let Ok(fun) = Function::try_from(s) {
                                if fun.inputs().get() > 1 {
                                    needs_bracket = true;
                                }
                                if fun.first_expected_var(NonZeroU8::new(1).unwrap()) {
                                    inner_vars
                                        .extend(iter::repeat_n("", fun.inner_vars() as usize));
                                }
                                if fun.has_var() {
                                    inner_vars_count.push(fun.inner_vars());
                                }
                                tokens.last_mul(
                                    &mut operator_stack,
                                    no_input_left,
                                    &mut last_mul,
                                    false,
                                );
                                operator_stack.push(Operator::Function(fun, Derivative::default()));
                                open_input = false;
                                fn_inputs.push(NonZeroU8::new(1).unwrap());
                            } else if count == 1
                                && s.chars().all(|c| c.is_alphabetic())
                                && !inner_vars_count.is_empty()
                                && let Some(n) = get_var_position(
                                    &mut inner_vars_count,
                                    &fn_inputs,
                                    &operator_stack,
                                    inner_vars.len(),
                                )
                            {
                                tokens.last_mul(
                                    &mut operator_stack,
                                    no_input_left,
                                    &mut last_mul,
                                    true,
                                );
                                tokens.push(Token::InnerVar(n as u16));
                                inner_vars[n] = s;
                                open_input = true;
                            } else if let Some(_i) = {
                                #[cfg(feature = "units")]
                                {
                                    UNITS.iter().position(|v| *v == s)
                                }
                                #[cfg(not(feature = "units"))]
                                {
                                    None::<()>
                                }
                            } {
                                #[cfg(feature = "units")]
                                tokens.push(Units::from(_i).into())
                            } else if let Some(f) = NumberBase::parse_radix(s, base) {
                                tokens.last_mul(
                                    &mut operator_stack,
                                    no_input_left,
                                    &mut last_mul,
                                    true,
                                );
                                tokens.push(f.into());
                                open_input = true;
                            } else if count != 1 {
                                count -= 1;
                                l -= value[i..i + l].chars().last().unwrap().len_utf8();
                                continue;
                            } else {
                                tokens.last_mul(
                                    &mut operator_stack,
                                    no_input_left,
                                    &mut last_mul,
                                    true,
                                );
                                tokens.push(Token::InnerVar(inner_vars.len() as u16));
                                inner_vars.push(&value[i..i + l]);
                                open_input = true;
                            }
                            break;
                        }
                        chars.advance_by(count - 1).unwrap();
                        no_input_left = false;
                        last_open = false;
                        req_input = false;
                        expect_expr = !open_input;
                    }
                    '\'' => {
                        let d = match operator_stack.last_mut() {
                            Some(Operator::Custom(_, d)) => d,
                            Some(Operator::Function(_, d)) => d,
                            _ => return Err(ParseError::DerivativeError),
                        };
                        if d.is_integral() {
                            return Err(ParseError::MixedError);
                        }
                        d.increment()?;
                        open_input = false;
                        no_input_left = false;
                        last_open = false;
                        req_input = false;
                        expect_expr = true;
                    }
                    '`' => {
                        let d = match operator_stack.last_mut() {
                            Some(Operator::Custom(_, d)) => d,
                            Some(Operator::Function(_, d)) => d,
                            _ => return Err(ParseError::IntegralError),
                        };
                        if d.is_derivative() {
                            if d.get() == 0 {
                                d.set_integral()
                            } else {
                                return Err(ParseError::MixedError);
                            }
                        }
                        d.increment()?;
                        open_input = false;
                        no_input_left = false;
                        last_open = false;
                        req_input = false;
                        expect_expr = true;
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
                        tokens.last_mul(&mut operator_stack, no_input_left, &mut last_mul, true);
                        tokens.push(float.into());
                        chars.advance_by(l - 1).unwrap();
                        no_input_left = false;
                        last_open = false;
                        req_input = false;
                        open_input = true;
                        expect_expr = false;
                    }
                    '↉' | '⅐' | '⅑' | '⅒' | '⅓' | '⅔' | '⅕' | '⅖' | '⅗' | '⅘' | '⅙' | '⅚' | '⅛'
                    | '⅜' | '⅝' | '⅞' => {
                        tokens.last_mul(&mut operator_stack, no_input_left, &mut last_mul, true);
                        tokens.push(
                            match c {
                                '↉' => Number::default(),
                                '⅐' => Number::from(7).recip(),
                                '⅑' => Number::from(9).recip(),
                                '⅒' => Number::from(10).recip(),
                                '⅓' => Number::from(3).recip(),
                                '⅔' => Number::from(2) / Float::from(3),
                                '⅕' => Number::from(5).recip(),
                                '⅖' => Number::from(2) / Float::from(5),
                                '⅗' => Number::from(3) / Float::from(5),
                                '⅘' => Number::from(4) / Float::from(5),
                                '⅙' => Number::from(6).recip(),
                                '⅚' => Number::from(5) / Float::from(6),
                                '⅛' => Number::from(8).recip(),
                                '⅜' => Number::from(3) / Float::from(8),
                                '⅝' => Number::from(5) / Float::from(8),
                                '⅞' => Number::from(7) / Float::from(8),
                                _ => unreachable!(),
                            }
                            .into(),
                        );
                        no_input_left = false;
                        last_open = false;
                        req_input = false;
                        open_input = true;
                        expect_expr = false;
                    }
                    '⅟' => {
                        tokens.last_mul(&mut operator_stack, no_input_left, &mut last_mul, true);
                        tokens.push(Number::from(1).into());
                        tokens.pop_stack(
                            &mut operator_stack,
                            &mut inner_vars,
                            funs,
                            Operator::Div,
                            no_input_left,
                        )?;
                        req_input = false;
                        open_input = true;
                        expect_expr = false;
                        no_input_left = true;
                        last_open = false;
                        last_mul = false;
                    }
                    '⁰' | '¹' | '²' | '³' | '⁴' | '⁵' | '⁶' | '⁷' | '⁸' | '⁹' | 'ⁱ' | '⁻'
                        if base <= 10 =>
                    {
                        let mut str = String::with_capacity(value.len());
                        let mut imag = false;
                        if c == '⁻' {
                            str.push('-');
                        } else if c == 'i' {
                            str.push('1');
                        } else {
                            str.push(match c {
                                '⁰' => '0',
                                '¹' => '1',
                                '²' => '2',
                                '³' => '3',
                                '⁴' => '4',
                                '⁵' => '5',
                                '⁶' => '6',
                                '⁷' => '7',
                                '⁸' => '8',
                                '⁹' => '9',
                                _ => unreachable!(),
                            })
                        }
                        let mut n = 1;
                        let mut l = c.len_utf8();
                        for t in value[i + l..].chars() {
                            if matches!(
                                t,
                                '⁰' | '¹'
                                    | '²'
                                    | '³'
                                    | '⁴'
                                    | '⁵'
                                    | '⁶'
                                    | '⁷'
                                    | '⁸'
                                    | '⁹'
                                    | 'ⁱ'
                            ) {
                                l += t.len_utf8();
                                n += 1;
                                if t == 'ⁱ' {
                                    imag = true;
                                    break;
                                } else {
                                    str.push(match t {
                                        '⁰' => '0',
                                        '¹' => '1',
                                        '²' => '2',
                                        '³' => '3',
                                        '⁴' => '4',
                                        '⁵' => '5',
                                        '⁶' => '6',
                                        '⁷' => '7',
                                        '⁸' => '8',
                                        '⁹' => '9',
                                        _ => unreachable!(),
                                    })
                                }
                            } else {
                                break;
                            }
                        }
                        #[allow(unused_mut)]
                        let Some(mut float) = NumberBase::parse_radix(&str, base) else {
                            return Err(ParseError::UnknownToken(&value[i..i + l]));
                        };
                        if imag {
                            #[cfg(feature = "complex")]
                            float.mul_i_mut(false);
                        }
                        tokens.push(float.into());
                        tokens.pop_stack(
                            &mut operator_stack,
                            &mut inner_vars,
                            funs,
                            Operator::Pow,
                            no_input_left,
                        )?;
                        chars.advance_by(n - 1).unwrap();
                        last_mul = true;
                        no_input_left = false;
                        last_open = false;
                        req_input = false;
                        open_input = true;
                        expect_expr = false;
                    }
                    ',' => {
                        if req_input || expect_expr {
                            return Err(ParseError::MissingInput);
                        }
                        while let Some(top) =
                            operator_stack.pop_if(|top| !matches!(top, Operator::Bracket(_)))
                        {
                            tokens.push_operator(top, &mut inner_vars, &operator_stack, funs)?;
                        }
                        if let Some(last) = fn_inputs.last_mut() {
                            *last = last.checked_add(1).unwrap();
                            if operator_stack.len() < 2 {
                                return Err(ParseError::CommaError);
                            }
                            match operator_stack[operator_stack.len() - 2] {
                                Operator::Custom(_, _) => {}
                                Operator::Function(fun, _) => {
                                    if fun.first_expected_var(*last) {
                                        inner_vars
                                            .extend(iter::repeat_n("", fun.inner_vars() as usize));
                                    }
                                }
                                _ => return Err(ParseError::CommaError),
                            }
                        } else if !expect_let {
                            return Err(ParseError::CommaError);
                        }
                        no_input_left = true;
                        last_open = false;
                        last_mul = false;
                        open_input = false;
                        expect_expr = true;
                    }
                    ')' => {
                        if req_input || expect_expr {
                            return Err(ParseError::MissingInput);
                        }
                        while let Some(top) =
                            operator_stack.pop_if(|top| !matches!(top, Operator::Bracket(_)))
                        {
                            tokens.push_operator(top, &mut inner_vars, &operator_stack, funs)?;
                        }
                        if matches!(
                            operator_stack.last(),
                            Some(Operator::Bracket(Bracket::Absolute))
                        ) {
                            return Err(ParseError::AbsoluteBracketFailed);
                        }
                        operator_stack
                            .pop_if(|top| matches!(top, Operator::Bracket(Bracket::Parenthesis)));
                        if tokens.close_off_bracket(
                            &mut operator_stack,
                            &mut inner_vars,
                            &mut inner_vars_count,
                            funs,
                            &mut fn_inputs,
                        )? {
                            last_mul = false;
                            open_input = false;
                            req_input = false;
                            expect_expr = true;
                        } else {
                            last_mul = true;
                            open_input = true;
                            expect_expr = false;
                        }
                        last_open = false;
                        no_input_left = false;
                    }
                    '(' => {
                        tokens.last_mul(&mut operator_stack, no_input_left, &mut last_mul, true);
                        operator_stack.push(Bracket::Parenthesis.into());
                        no_input_left = true;
                        last_open = true;
                        req_input = false;
                        last_mul = false;
                        open_input = false;
                        expect_expr = true;
                    }
                    '|' => {
                        if abs == 0 || last_open || req_input {
                            operator_stack.push(Bracket::Absolute.into());
                            abs += 1;
                            no_input_left = true;
                            last_open = true;
                            req_input = true;
                            last_mul = false;
                            open_input = false;
                            expect_expr = true;
                        } else {
                            while let Some(top) =
                                operator_stack.pop_if(|top| !matches!(top, Operator::Bracket(_)))
                            {
                                tokens.push_operator(
                                    top,
                                    &mut inner_vars,
                                    &operator_stack,
                                    funs,
                                )?;
                            }
                            if !matches!(
                                operator_stack.pop(),
                                Some(Operator::Bracket(Bracket::Absolute))
                            ) {
                                return Err(ParseError::AbsoluteBracketFailed);
                            }
                            tokens.push(Function::Abs.into());
                            if tokens.close_off_bracket(
                                &mut operator_stack,
                                &mut inner_vars,
                                &mut inner_vars_count,
                                funs,
                                &mut fn_inputs,
                            )? {
                                last_mul = false;
                                open_input = false;
                                req_input = false;
                                expect_expr = true;
                            } else {
                                last_mul = true;
                                open_input = true;
                                expect_expr = false;
                            }
                            last_open = false;
                            no_input_left = false;
                            abs -= 1;
                        }
                    }
                    _ => {
                        let mut l = c.len_utf8();
                        if let Some(next) = value[i + l..].chars().next()
                            && Operator::try_from(&value[i..i + l + next.len_utf8()]).is_ok()
                        {
                            chars.next();
                            l += next.len_utf8();
                        }
                        let s = &value[i..i + l];
                        if let Ok(mut operator) = Operator::try_from(s) {
                            if expect_let && operator == Operator::Solve {
                                open_input = false;
                                expect_let = false;
                                let Some(name) = inner_vars.try_remove(0) else {
                                    return Err(ParseError::VarExpectedName);
                                };
                                if !inner_vars.is_empty() {
                                    let (a, b) = funs.add(
                                        vars,
                                        name,
                                        NonZeroU8::new(inner_vars.len() as u8).unwrap(),
                                    );
                                    inputs = Some(NewCustom::Fun(a, b));
                                } else {
                                    inner_vars.pop();
                                    if !inner_vars.is_empty() {
                                        return Err(ParseError::InnerVarError);
                                    }
                                    inputs = Some(NewCustom::Var(name));
                                }
                            } else {
                                if no_input_left && let Some(op) = operator.get_unary_left() {
                                    if op == Operator::Add {
                                        req_input = true;
                                        last_open = false;
                                        continue;
                                    } else {
                                        operator = op;
                                    }
                                }
                                if !operator.is_unary() {
                                    req_input = true;
                                    if !open_input {
                                        return Err(ParseError::MissingInput);
                                    }
                                } else {
                                    req_input = operator.unary_left();
                                }
                                tokens.pop_stack(
                                    &mut operator_stack,
                                    &mut inner_vars,
                                    funs,
                                    operator,
                                    no_input_left,
                                )?;
                                no_input_left = !operator.unary_right();
                                last_open = false;
                            }
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
                if let Operator::Bracket(bracket) = operator {
                    match bracket {
                        Bracket::Absolute => {
                            tokens.push(Function::Abs.into());
                        }
                        Bracket::Parenthesis => {}
                    };
                    tokens.close_off_bracket(
                        &mut operator_stack,
                        &mut inner_vars,
                        &mut inner_vars_count,
                        funs,
                        &mut fn_inputs,
                    )?;
                } else {
                    tokens.push_operator(operator, &mut inner_vars, &operator_stack, funs)?;
                }
            }
            if !inner_vars.is_empty() && matches!(inputs, None | Some(NewCustom::Var(_))) {
                return Err(ParseError::InnerVarError);
            }
        };
        if let Err(e) = inner {
            if let Some(NewCustom::Fun(b, v)) = inputs {
                if b {
                    funs.pop();
                }
                if let Some((v, n)) = v {
                    funs[v as usize].name = n
                }
            }
            return Err(e);
        }
        Ok(tokens.end(
            inputs,
            simplify,
            vars,
            funs,
            has_graph_vars,
            #[cfg(feature = "float_rand")]
            rand,
        ))
    }
    pub fn last_mul(
        &mut self,
        operator_stack: &mut Vec<Operator>,
        negate: bool,
        last_mul: &mut bool,
        new: bool,
    ) {
        if *last_mul {
            self.pop_stack(operator_stack, &mut Vec::new(), &[], Operator::Mul, negate)
                .unwrap();
        }
        *last_mul = new;
    }
    pub fn push_operator(
        &mut self,
        operator: Operator,
        inner_vars: &mut Vec<&str>,
        operator_stack: &[Operator],
        custom_funs: &[FunctionVar],
    ) -> Result<(), ParseError<'static>> {
        if operator == Operator::Solve {
            let count = operator_stack.iter().map(|a| a.inner_vars() as usize).sum();
            if inner_vars.len() == count {
                return Err(ParseError::InnerVarError);
            }
            if inner_vars.len() > count + 1 {
                #[cfg(not(all(feature = "vector", feature = "matrix")))]
                {
                    return Err(ParseError::VecMatNotEnabled);
                }
                #[cfg(all(feature = "vector", feature = "matrix"))]
                {
                    todo!()
                }
            }
            if operator_stack.contains(&Operator::Solve) {
                #[cfg(not(all(feature = "vector", feature = "matrix")))]
                {
                    return Err(ParseError::VecMatNotEnabled);
                }
                #[cfg(all(feature = "vector", feature = "matrix"))]
                {
                    todo!()
                }
            }
            self.push(Function::Sub.into());
            self.compact_args(Function::Solve, inner_vars, custom_funs);
            self.push(Function::Solve.into());
            Ok(())
        } else {
            self.push(operator.into());
            Ok(())
        }
    }
    pub fn pop_stack(
        &mut self,
        operator_stack: &mut Vec<Operator>,
        inner_vars: &mut Vec<&str>,
        custom_funs: &[FunctionVar],
        operator: Operator,
        negate: bool,
    ) -> Result<(), ParseError<'static>> {
        while let Some(top) = operator_stack.pop_if(|top| {
            !matches!(top, Operator::Bracket(_))
                && (top.precedence() > operator.precedence()
                    || (top.precedence() == operator.precedence() && operator.left_associative()))
                && !(negate && operator == Operator::Negate && *top == Operator::Pow)
        }) {
            self.push_operator(top, inner_vars, operator_stack, custom_funs)?;
        }
        operator_stack.push(operator);
        Ok(())
    }
    pub fn close_off_bracket(
        &mut self,
        operator_stack: &mut Vec<Operator>,
        inner_vars: &mut Vec<&str>,
        inner_vars_count: &mut Vec<u8>,
        custom_funs: &[FunctionVar],
        fn_inputs: &mut Vec<NonZeroU8>,
    ) -> Result<bool, ParseError<'static>> {
        Ok(
            if let Some(top) = operator_stack
                .pop_if(|l| matches!(l, Operator::Function(_, _) | Operator::Custom(_, _)))
            {
                match top {
                    Operator::Custom(i, mut d) => {
                        let inputs = fn_inputs.pop().unwrap();
                        let normal = custom_funs.get(i as usize).unwrap().inputs;
                        match normal.cmp(&inputs) {
                            Ordering::Greater => return Err(ParseError::MissingInput),
                            Ordering::Less
                                if d.is_derivative() || inputs.get() != normal.get() * 2 =>
                            {
                                return Err(ParseError::ExtraInput);
                            }
                            Ordering::Less => d.set_integral_twice_input(),
                            _ => {}
                        }
                        self.push(Token::CustomFun(i, d));
                        false
                    }
                    Operator::Function(mut fun, mut d) => {
                        if fun.has_var() {
                            inner_vars_count.pop().unwrap();
                        }
                        let mut inputs = fn_inputs.pop().unwrap();
                        fun.set_inputs(inputs);
                        if fun.inputs().get() + 1 < inputs.get() + fun.inner_vars() {
                            let last = self[..].get_last(custom_funs);
                            let mut t = last;
                            for _ in fun.inputs().get()..inputs.get() {
                                let last = self[..t].get_last(custom_funs);
                                if t - 1 == last && matches!(self[last], Token::InnerVar(_)) {
                                    self.remove(last);
                                } else {
                                    return Err(ParseError::InnerVarError);
                                }
                                t = last;
                            }
                            inputs = fun.inputs();
                        }
                        let normal = fun.inputs();
                        match normal.cmp(&inputs) {
                            Ordering::Greater => return Err(ParseError::MissingInput),
                            Ordering::Less
                                if d.is_derivative() || inputs.get() != normal.get() * 2 =>
                            {
                                return Err(ParseError::ExtraInput);
                            }
                            Ordering::Less => d.set_integral_twice_input(),
                            _ => {}
                        }
                        self.compact_args(fun, inner_vars, custom_funs);
                        self.push(Token::Function(fun, d));
                        false
                    }
                    _ => {
                        unreachable!()
                    }
                }
            } else {
                false
            },
        )
    }
    pub fn compact_args(
        &mut self,
        fun: Function,
        inner_vars: &mut Vec<&str>,
        custom_funs: &[FunctionVar],
    ) {
        let mut t = self.len();
        for _ in 0..fun.compact() {
            let last = self[..t].get_last(custom_funs);
            self.insert(last, Token::Skip(t - last));
            t = last;
        }
        for _ in 0..fun.inner_vars() {
            inner_vars.pop().unwrap();
        }
    }
    pub fn get_infix(
        &self,
        custom_vars: &[Variable],
        custom_funs: &[FunctionVar],
        graph_vars: &[&str],
    ) -> impl Display {
        self[..].get_infix(custom_vars, custom_funs, graph_vars)
    }
    pub fn get_rpn(
        &self,
        custom_vars: &[Variable],
        custom_funs: &[FunctionVar],
        graph_vars: &[&str],
    ) -> impl Display {
        self[..].get_rpn(custom_vars, custom_funs, graph_vars)
    }
    fn end(
        mut self,
        inputs: Option<NewCustom>,
        simplify: bool,
        vars: &mut Variables,
        funs: &mut Functions,
        has_graph_vars: Vec<bool>,
        #[cfg(feature = "float_rand")] rand: &mut Rand,
    ) -> ParseReturn {
        if self.is_empty() {
            self.push(Number::default().into());
        }
        if let Some(name) = inputs {
            if let NewCustom::Var(name) = name {
                if simplify {
                    self.simplify::<{ Volatility::Volatile }>(
                        vars,
                        funs,
                        0,
                        #[cfg(feature = "float_rand")]
                        rand,
                    );
                }
                let val = self.compute(
                    &[],
                    funs,
                    vars,
                    #[cfg(feature = "float_rand")]
                    rand,
                );
                if let Some(v) = vars.position(name) {
                    vars[v as usize].value = val;
                    vars[v as usize].volatile = Volatility::GraphConstant;
                } else {
                    vars.push(Variable::new(name, val, Volatility::GraphConstant));
                }
                funs.iter_mut().for_each(|v| {
                    if v.name.as_ref().is_some_and(|n| n.as_ref() == name) {
                        v.name = None;
                    }
                });
            } else {
                let inputs = funs.last().unwrap().inputs.get();
                if simplify {
                    self.simplify::<{ Volatility::GraphConstant }>(
                        vars,
                        funs,
                        inputs,
                        #[cfg(feature = "float_rand")]
                        rand,
                    );
                }
                let volatile = self[..].volatility(funs, vars, inputs);
                let Some(v) = funs.last_mut() else {
                    unreachable!()
                };
                v.volatile = volatile;
                v.tokens = self;
            }
            ParseReturn::Var
        } else if has_graph_vars.iter().copied().any(|a| a) {
            if simplify {
                self.simplify::<{ Volatility::Volatile }>(
                    vars,
                    funs,
                    0,
                    #[cfg(feature = "float_rand")]
                    rand,
                );
            }
            ParseReturn::Graph(self, has_graph_vars)
        } else {
            if simplify {
                self.simplify::<{ Volatility::Volatile }>(
                    vars,
                    funs,
                    0,
                    #[cfg(feature = "float_rand")]
                    rand,
                );
            }
            ParseReturn::Tokens(self)
        }
    }
}
pub(crate) fn get_var_position(
    inner_vars_count: &mut [u8],
    fn_inputs: &[NonZeroU8],
    operator_stack: &[Operator],
    mut inner_vars: usize,
) -> Option<usize> {
    let mut n = inner_vars_count.len();
    let mut inputs = fn_inputs.iter();
    let mut last = None;
    if operator_stack
        .iter()
        .rfind(|la| {
            let f = match la {
                Operator::Function(f, _) => f,
                Operator::Custom(_, _) => {
                    inputs.next_back();
                    return false;
                }
                _ => return false,
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
        Some(inner_vars)
    } else {
        None
    }
}
impl TokensSlice {
    pub fn volatility(
        &self,
        custom_funs: &[FunctionVar],
        custom_vars: &[Variable],
        inputs: u8,
    ) -> Volatility {
        let mut volatility = Volatility::Constant;
        for token in self.iter() {
            match *token {
                Token::Function(fun, _) if fun.volatility() > volatility => {
                    if fun.volatility() == Volatility::Volatile {
                        return Volatility::Volatile;
                    }
                    volatility = fun.volatility()
                }
                Token::CustomVar(index) if custom_vars[index as usize].volatile > volatility => {
                    if custom_vars[index as usize].volatile == Volatility::Volatile {
                        return Volatility::Volatile;
                    }
                    volatility = custom_vars[index as usize].volatile;
                }
                Token::CustomFun(index, _) if custom_funs[index as usize].volatile > volatility => {
                    if custom_funs[index as usize].volatile == Volatility::Volatile {
                        return Volatility::Volatile;
                    }
                    volatility = custom_funs[index as usize].volatile;
                }
                Token::InnerVar(index)
                    if index < inputs as u16 && Volatility::GraphConstant > volatility =>
                {
                    volatility = Volatility::GraphConstant;
                }
                Token::GraphVar(_) => return Volatility::Volatile,
                _ => {}
            }
        }
        volatility
    }
    pub fn get_last_with_end(&self, custom_funs: &[FunctionVar], mut end: usize) -> usize {
        let mut inputs = 1;
        while inputs != 0 {
            inputs -= 1;
            end -= 1;
            match self[end] {
                Token::CustomFun(j, d) if d.is_integral_twice_input() => {
                    inputs += 2 * custom_funs[j as usize].inputs.get()
                }
                Token::Function(o, d) if d.is_integral_twice_input() => {
                    inputs += 2 * o.inputs().get()
                }
                Token::CustomFun(j, _) => inputs += custom_funs[j as usize].inputs.get(),
                Token::Function(o, _) => inputs += o.inputs().get(),
                Token::Skip(_) => inputs += 1,
                _ => {}
            }
        }
        if end != 0 && matches!(self[end - 1], Token::Skip(_)) {
            end -= 1;
        }
        end
    }
    pub fn get_last(&self, custom_funs: &[FunctionVar]) -> usize {
        self.get_last_with_end(custom_funs, self.len())
    }
    pub fn get_from_last_with_end(
        &self,
        custom_funs: &[FunctionVar],
        end: usize,
    ) -> (&Self, usize) {
        let last = self.get_last_with_end(custom_funs, end);
        if matches!(self[end - 1], Token::Skip(_)) {
            (&self[last..end - 1], last)
        } else {
            (&self[last..end], last)
        }
    }
    pub fn get_from_last(&self, custom_funs: &[FunctionVar]) -> (&Self, usize) {
        self.get_from_last_with_end(custom_funs, self.len())
    }
    pub fn get_lasts(&self, custom_funs: &[FunctionVar]) -> Vec<&Self> {
        let inputs = match *self.last().unwrap() {
            Token::CustomFun(j, d) if d.is_integral_twice_input() => {
                2 * custom_funs[j as usize].inputs.get()
            }
            Token::Function(o, d) if d.is_integral_twice_input() => 2 * o.inputs().get(),
            Token::CustomFun(j, _) => custom_funs[j as usize].inputs.get(),
            Token::Function(o, _) => o.inputs().get(),
            _ => unreachable!(),
        };
        let mut ret = vec![&self[0..0]; inputs as usize];
        let mut end = self.len() - 1;
        for j in (0..inputs).rev() {
            (ret[j as usize], end) = self.get_from_last_with_end(custom_funs, end);
        }
        ret
    }
}
impl Derivative {
    pub fn get(self) -> u8 {
        self.0 & 0b0011_1111
    }
    pub fn set_integral(&mut self) {
        self.0 |= 0b1000_0000;
    }
    pub fn set_integral_twice_input(&mut self) {
        self.0 |= 0b1100_0000;
    }
    pub fn set_derivative(&mut self) {
        self.0 &= 0b0011_1111;
    }
    pub fn is_integral(self) -> bool {
        self.0 & 0b1000_0000 == 0b1000_0000
    }
    pub fn is_integral_twice_input(self) -> bool {
        self.0 & 0b1100_0000 == 0b1100_0000
    }
    pub fn is_derivative(self) -> bool {
        self.0 & 0b1000_0000 == 0
    }
    pub fn from(n: u8) -> Result<Self, ParseError<'static>> {
        if n > 0b0011_1111 {
            Err(ParseError::TooManyDerivatives)
        } else {
            Ok(Self(n))
        }
    }
    pub fn increment(&mut self) -> Result<(), ParseError<'static>> {
        if self.get() == 0b0011_1111 {
            Err(ParseError::TooManyDerivatives)
        } else {
            self.0 += 1;
            Ok(())
        }
    }
}
impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{}", n),
            &Self::Function(fun, _) => {
                if let Ok(o) = Operator::try_from(fun) {
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
    NeedsBracket,
    InnerVarError,
    VarExpectedName,
    CommaError,
    DerivativeError,
    IntegralError,
    MixedError,
    TooManyDerivatives,
    RpnUnsupported,
    GraphVarError,
    #[cfg(not(all(feature = "vector", feature = "matrix")))]
    VecMatNotEnabled,
}
impl Display for Tokens {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self[..])
    }
}
impl Display for TokensSlice {
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
    fn greater_precedence(&self, o: Operator) -> bool {
        match self {
            Token::Number(_) => true,
            Token::InnerVar(_) => true,
            Token::GraphVar(_) => true,
            Token::CustomFun(_, _) => true,
            Token::CustomVar(_) => true,
            Token::Skip(_) => {
                unreachable!()
            }
            &Token::Function(f, _) => {
                if let Ok(f) = Operator::try_from(f) {
                    (f == o && f.left_associative()) || f.precedence() > o.precedence()
                } else {
                    true
                }
            }
        }
    }
}
impl TokensSlice {
    pub fn get_infix(
        &self,
        custom_vars: &[Variable],
        custom_funs: &[FunctionVar],
        graph_vars: &[&str],
    ) -> impl Display {
        fmt::from_fn(move |fmt| match self.last().unwrap() {
            Token::Number(n) => write!(fmt, "{}", n),
            &Token::InnerVar(i) => write!(fmt, "{}", (b'n' + i as u8) as char),
            &Token::GraphVar(i) => write!(fmt, "{}", graph_vars[i as usize]),
            &Token::CustomFun(i, d) => {
                let lasts = self.get_lasts(custom_funs);
                let mut first = true;
                write!(fmt, "{}", custom_funs[i as usize].name.as_ref().unwrap())?;
                write_commas(fmt, d)?;
                for arg in lasts {
                    let arg = arg.get_infix(custom_vars, custom_funs, graph_vars);
                    if first {
                        first = false;
                        write!(fmt, "{arg}")?;
                    } else {
                        write!(fmt, ",{arg}")?;
                    }
                }
                write!(fmt, ")")
            }
            &Token::CustomVar(i) => {
                write!(fmt, "{}", custom_vars[i as usize].name.as_ref().unwrap())
            }
            Token::Skip(_) => Ok(()),
            &Token::Function(f, d) => {
                let l = self.len() - 1;
                let last = self[..l].get_last(custom_funs);
                if let Ok(o) = Operator::try_from(f) {
                    let arg = self[last..l].get_infix(custom_vars, custom_funs, graph_vars);
                    let arg = if self[l - 1].greater_precedence(o)
                        || (f.is_chainable()
                            && if let Token::Function(f, _) = self[l - 1] {
                                f.is_chainable()
                            } else {
                                false
                            }) {
                        format_args!("{arg}")
                    } else {
                        format_args!("({arg})")
                    };
                    if o.inputs().get() == 2 {
                        let arg1 = self[..last].get_infix(custom_vars, custom_funs, graph_vars);
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
                    let lasts = self.get_lasts(custom_funs);
                    let mut first = true;
                    write!(fmt, "{f}(")?;
                    write_commas(fmt, d)?;
                    for arg in lasts {
                        let arg = arg.get_infix(custom_vars, custom_funs, graph_vars);
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
    pub fn get_rpn(
        &self,
        custom_vars: &[Variable],
        custom_funs: &[FunctionVar],
        graph_vars: &[&str],
    ) -> impl Display {
        fmt::from_fn(move |fmt| {
            let mut first = true;
            for token in self.iter() {
                if !first {
                    write!(fmt, " ")?;
                }
                first = false;
                match token {
                    Token::Number(n) => write!(fmt, "{}", n)?,
                    &Token::InnerVar(i) => write!(fmt, "{}", (b'n' + i as u8) as char)?,
                    &Token::GraphVar(i) => write!(fmt, "{}", graph_vars[i as usize])?,
                    &Token::CustomFun(i, d) => {
                        write!(fmt, "{}", custom_funs[i as usize].name.as_ref().unwrap())?;
                        write_commas(fmt, d)?;
                    }
                    &Token::CustomVar(i) => {
                        write!(fmt, "{}", custom_vars[i as usize].name.as_ref().unwrap())?
                    }
                    &Token::Function(fun, d) => {
                        if let Ok(o) = Operator::try_from(fun) {
                            write!(fmt, "{o}")?;
                        } else {
                            write!(fmt, "{fun}")?;
                            write_commas(fmt, d)?;
                        }
                    }
                    Token::Skip(_) => first = true,
                }
            }
            Ok(())
        })
    }
}
fn write_commas(fmt: &mut Formatter, d: Derivative) -> fmt::Result {
    if d.is_derivative() {
        for _ in 0..d.get() {
            write!(fmt, "\'")?;
        }
    } else {
        for _ in 0..d.get() {
            write!(fmt, "`")?;
        }
    }
    Ok(())
}
impl From<Number> for Token {
    fn from(value: Number) -> Self {
        Self::Number(value)
    }
}
impl From<Operator> for Token {
    fn from(value: Operator) -> Self {
        Self::Function(value.into(), Derivative::default())
    }
}
impl From<Function> for Token {
    fn from(value: Function) -> Self {
        Self::Function(value, Derivative::default())
    }
}
impl Token {
    pub fn num(self) -> Number {
        let Self::Number(num) = self else {
            unreachable!()
        };
        num
    }
    pub fn skip(&self) -> usize {
        let &Self::Skip(num) = self else {
            unreachable!()
        };
        num
    }
    pub fn skip_mut(&mut self) -> &mut usize {
        let Self::Skip(num) = self else {
            unreachable!()
        };
        num
    }
    pub fn num_ref(&self) -> &Number {
        let Self::Number(num) = self else {
            unreachable!()
        };
        num
    }
    pub fn inner_var(self) -> u16 {
        let Self::InnerVar(n) = self else {
            unreachable!()
        };
        n
    }
    pub fn inner_var_ref(&self) -> u16 {
        let &Self::InnerVar(n) = self else {
            unreachable!()
        };
        n
    }
    pub fn num_mut(&mut self) -> &mut Number {
        let Self::Number(num) = self else {
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
impl Deref for TokensSlice {
    type Target = [Token];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for TokensSlice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
macro_rules! impl_range {
    ($ty:ty, $range:path) => {
        impl Index<$range> for $ty {
            type Output = TokensSlice;
            fn index(&self, index: $range) -> &Self::Output {
                unsafe { mem::transmute(&self.0[index]) }
            }
        }
        impl IndexMut<$range> for $ty {
            fn index_mut(&mut self, index: $range) -> &mut Self::Output {
                unsafe { mem::transmute(&mut self.0[index]) }
            }
        }
    };
}
impl_range!(TokensSlice, RangeInclusive<usize>);
impl_range!(TokensSlice, Range<usize>);
impl_range!(TokensSlice, RangeFrom<usize>);
impl_range!(TokensSlice, RangeTo<usize>);
impl_range!(TokensSlice, RangeToInclusive<usize>);
impl_range!(TokensSlice, RangeFull);
impl_range!(Tokens, RangeInclusive<usize>);
impl_range!(Tokens, Range<usize>);
impl_range!(Tokens, RangeFrom<usize>);
impl_range!(Tokens, RangeTo<usize>);
impl_range!(Tokens, RangeToInclusive<usize>);
impl_range!(Tokens, RangeFull);
impl Index<usize> for Tokens {
    type Output = Token;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl Index<usize> for TokensSlice {
    type Output = Token;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl IndexMut<usize> for Tokens {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
impl IndexMut<usize> for TokensSlice {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
