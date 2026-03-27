use crate::parse::{Token, Tokens, TokensRef};
use crate::{Functions, Number, Variables};
use std::array;
use ucalc_numbers::{Constant, Float, FloatTrait, RealTrait};
#[derive(Copy, Clone)]
pub(crate) struct Compute<'a> {
    pub(crate) tokens: TokensRef<'a>,
    pub(crate) vars: &'a [Number],
    pub(crate) funs: &'a Functions,
    pub(crate) custom_vars: &'a Variables,
    pub(crate) offset: usize,
}
impl Tokens {
    pub fn compute(&self, vars: &[Number], funs: &Functions, custom_vars: &Variables) -> Number {
        let mut fun_vars = Vec::with_capacity(self.len());
        let mut stack = Tokens(Vec::with_capacity(
            self.len() + funs.iter().map(|c| c.tokens.len()).sum::<usize>(),
        ));
        self.compute_buffer(&mut fun_vars, vars, funs, custom_vars, &mut stack)
    }
    pub fn compute_buffer(
        &self,
        fun_vars: &mut Vec<Number>,
        vars: &[Number],
        funs: &Functions,
        custom_vars: &Variables,
        stack: &mut Tokens,
    ) -> Number {
        self.compute_buffer_with(fun_vars, vars, funs, custom_vars, stack, 0)
    }
    pub fn compute_buffer_with(
        &self,
        fun_vars: &mut Vec<Number>,
        vars: &[Number],
        funs: &Functions,
        custom_vars: &Variables,
        stack: &mut Tokens,
        offset: usize,
    ) -> Number {
        Compute::new(TokensRef(self), vars, funs, custom_vars, offset)
            .compute_buffer_with(fun_vars, stack)
    }
    pub(crate) fn get_skip<'a, 'b, const N: usize, const K: usize>(
        &'b self,
        tokens: TokensRef<'a>,
    ) -> ([&'b Token; K], [TokensRef<'a>; N]) {
        let tokens = self.get_skip_tokens(tokens);
        let args = self.get_skip_var(N);
        (args, tokens)
    }
    pub fn get_skip_var<const N: usize>(&self, end: usize) -> [&Token; N] {
        let end = self.len() - (end + 1);
        array::from_fn(|i| &self[end - (N - (i + 1))])
    }
    pub(crate) fn get_skip_tokens<'a, const N: usize>(
        &self,
        tokens: TokensRef<'a>,
    ) -> [TokensRef<'a>; N] {
        let len = tokens.len();
        let mut t = len - 1;
        let mut arr = array::from_fn(|_| TokensRef(&[]));
        for i in 0..N {
            let back = self[self.len() - (i + 1)].skip();
            let ret = TokensRef(&tokens.0[back..t]);
            t = back.saturating_sub(1);
            arr[N - (i + 1)] = ret
        }
        arr
    }
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn range(
        &mut self,
        compute: Compute,
        fun_vars: &mut Vec<Number>,
        fun: impl FnOnce(&mut dyn Iterator<Item = Number>) -> Token,
    ) {
        let len = self.len();
        let ([start, end], [tokens]) = self.get_skip(compute.tokens);
        let start = start.num_ref().real().clone().into_isize();
        let end = end.num_ref().real().clone().into_isize();
        fun_vars.push(Number::from(start));
        let mut iter = (start..=end).map(|_| {
            let ret = compute.tokens(tokens).compute_buffer_with(fun_vars, self);
            *fun_vars.last_mut().unwrap() += Float::from(1);
            ret
        });
        self[len - 3] = fun(&mut iter);
        fun_vars.pop().unwrap();
        self.drain(len - 2..);
    }
}
impl<'a> Compute<'a> {
    pub fn offset(self, offset: usize) -> Self {
        Self {
            tokens: self.tokens,
            vars: self.vars,
            funs: self.funs,
            custom_vars: self.custom_vars,
            offset,
        }
    }
    pub fn tokens<'b: 'a>(self, tokens: TokensRef<'b>) -> Self {
        Self {
            tokens,
            vars: self.vars,
            funs: self.funs,
            custom_vars: self.custom_vars,
            offset: self.offset,
        }
    }
    pub fn new(
        tokens: TokensRef<'a>,
        vars: &'a [Number],
        funs: &'a Functions,
        custom_vars: &'a Variables,
        offset: usize,
    ) -> Self {
        Self {
            tokens,
            vars,
            funs,
            custom_vars,
            offset,
        }
    }
    pub fn compute_buffer_with(self, fun_vars: &mut Vec<Number>, stack: &mut Tokens) -> Number {
        let mut i = 0;
        while i < self.tokens.len() {
            let len = stack.len();
            match &self.tokens[i] {
                Token::Function(operator) => {
                    let inputs = operator.inputs() as usize;
                    if operator.has_inner_fn() {
                        operator.compute_var(
                            self.tokens(TokensRef(&self.tokens[..=i])),
                            stack,
                            fun_vars,
                        )
                    } else if operator.is_chainable() {
                        let chain = if self.tokens.get(i + 1).is_some_and(|o| {
                            if let Token::Function(o) = o {
                                o.is_chainable()
                            } else {
                                false
                            }
                        }) {
                            Some(self.tokens[len - inputs].num_ref())
                        } else {
                            None
                        };
                        operator.compute(stack, inputs);
                        if let Some(b) = chain {
                            let a = stack[len - inputs].num_mut();
                            *a = if a.is_zero() {
                                Number::from(Constant::Nan)
                            } else {
                                b.clone()
                            };
                        }
                    } else {
                        operator.compute(stack, inputs);
                    }
                }
                Token::Var(index) => {
                    stack.push(Token::Num(self.custom_vars[*index as usize].value.clone()))
                }
                Token::Fun(index) => {
                    let inputs = self.funs[*index as usize].inputs as usize;
                    let end = fun_vars.len();
                    fun_vars.push(stack[len - inputs].num_ref().clone());
                    fun_vars.extend(stack.drain(len + 1 - inputs..).map(|n| n.num()));
                    *stack[len - inputs].num_mut() =
                        self.funs[*index as usize].tokens.compute_buffer_with(
                            fun_vars,
                            self.vars,
                            self.funs,
                            self.custom_vars,
                            stack,
                            end,
                        );
                    fun_vars.drain(end..);
                }
                Token::InnerVar(index) => {
                    stack.push(Token::Num(fun_vars[self.offset + *index as usize].clone()))
                }
                Token::GraphVar(index) => {
                    stack.push(Token::Num(self.vars[*index as usize].clone()))
                }
                Token::Skip(to) => {
                    stack.push(Token::Skip(i + 1));
                    i += to;
                }
                Token::Num(n) => stack.push(Token::Num(n.clone())),
                Token::Polynomial(_) => unreachable!(),
            }
            i += 1;
        }
        stack.pop().map(|t| t.num()).unwrap()
    }
}
