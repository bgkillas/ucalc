use crate::parse::{Token, Tokens, TokensRef};
use crate::{Functions, Number, Variables};
use std::array;
use ucalc_numbers::{Constant, FloatTrait};
#[derive(Copy, Clone, Debug)]
pub(crate) struct Compute<'a> {
    pub(crate) tokens: TokensRef<'a>,
    pub(crate) vars: &'a [Number],
    pub(crate) funs: &'a Functions,
    pub(crate) custom_vars: &'a Variables,
    pub(crate) offset: usize,
}
impl Tokens {
    pub fn compute(&self, vars: &[Number], funs: &Functions, custom_vars: &Variables) -> Number {
        let cap = self.len() + funs.iter().map(|c| c.tokens.len()).sum::<usize>();
        let mut fun_vars = Vec::with_capacity(cap);
        let mut stack = Tokens(Vec::with_capacity(cap));
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
    pub(crate) fn get_skip_mut<'a, const N: usize, const K: usize>(
        &mut self,
        tokens: TokensRef<'a>,
    ) -> (&mut Number, [Number; K], [TokensRef<'a>; N]) {
        let tokens = self.get_skip_tokens(tokens);
        let args = self.get_skip_var();
        (self.last_mut().unwrap().num_mut(), args, tokens)
    }
    pub fn get_skip_var<const N: usize>(&mut self) -> [Number; N] {
        let mut arr = array::from_fn(|_| Number::default());
        for i in 0..N {
            arr[N - (i + 1)] = self.pop().unwrap().num();
        }
        arr
    }
    pub(crate) fn get_skip_tokens<'a, const N: usize>(
        &mut self,
        tokens: TokensRef<'a>,
    ) -> [TokensRef<'a>; N] {
        let len = tokens.len();
        let mut t = len - 1;
        let mut arr = array::from_fn(|_| TokensRef(&[]));
        for i in 0..N {
            let back = self.pop().unwrap().skip();
            let ret = TokensRef(&tokens.0[back..t]);
            t = back.saturating_sub(1);
            arr[N - (i + 1)] = ret
        }
        arr
    }
    pub(crate) fn get_skip_tokens_keep_one<'a, const N: usize>(
        &mut self,
        tokens: TokensRef<'a>,
    ) -> [TokensRef<'a>; N] {
        let len = tokens.len();
        let mut t = len - 1;
        let mut arr = array::from_fn(|_| TokensRef(&[]));
        for i in 0..N - 1 {
            let back = self.pop().unwrap().skip();
            let ret = TokensRef(&tokens.0[back..t]);
            t = back.saturating_sub(1);
            arr[N - (i + 1)] = ret
        }
        let back = self[self.len() - 1].skip();
        let ret = TokensRef(&tokens.0[back..t]);
        arr[0] = ret;
        arr
    }
    pub(crate) fn get_skip_tokens_keep_one_vec<'a>(
        &mut self,
        tokens: TokensRef<'a>,
        n: usize,
    ) -> Vec<TokensRef<'a>> {
        let len = tokens.len();
        let mut t = len - 1;
        let mut arr = vec![TokensRef(&[]); n];
        for i in 0..n - 1 {
            let back = self.pop().unwrap().skip();
            let ret = TokensRef(&tokens.0[back..t]);
            t = back.saturating_sub(1);
            arr[n - (i + 1)] = ret
        }
        let back = self[self.len() - 1].skip();
        let ret = TokensRef(&tokens.0[back..t]);
        arr[0] = ret;
        arr
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
        let mut tokens = self.tokens.iter().enumerate();
        while let Some((i, token)) = tokens.next() {
            match token {
                &Token::Function(operator, d) => {
                    if d.get_num() != 0 {
                        todo!()
                    }
                    let inputs = operator.inputs();
                    if operator.has_inner_fn() {
                        operator.compute_var(
                            self.tokens(TokensRef(&self.tokens[..=i])),
                            stack,
                            fun_vars,
                        )
                    } else if operator.is_chainable() {
                        let chain = if self.tokens.get(i + 1).is_some_and(|o| {
                            if let Token::Function(o, _) = o {
                                o.is_chainable()
                            } else {
                                false
                            }
                        }) {
                            Some(self.tokens[stack.len() - inputs.get() as usize].num_ref())
                        } else {
                            None
                        };
                        operator.compute(stack, inputs);
                        if let Some(b) = chain {
                            let len = stack.len();
                            let a = stack[len - 1].num_mut();
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
                &Token::Var(index) => {
                    stack.push(Token::Num(self.custom_vars[index as usize].value.clone()))
                }
                &Token::Fun(index, d) => {
                    let inputs = self.funs[index as usize].inputs.get() as usize;
                    let end = fun_vars.len();
                    let len = stack.len();
                    let compute = self
                        .offset(end)
                        .tokens(TokensRef(&self.funs[index as usize].tokens));
                    if d.get_num() != 0 {
                        if d.is_integral() {
                            if d.is_integral_two_input() {
                                if inputs != 1 {
                                    todo!()
                                }
                                fun_vars.push(Number::default());
                                let end = stack.pop().unwrap().num();
                                *stack.last_mut().unwrap().num_mut() = compute
                                    .numerical_nth_integral(
                                        fun_vars,
                                        stack,
                                        d.get_num(),
                                        stack.last().unwrap().num_ref().clone(),
                                        end,
                                        fun_vars.len() - 1,
                                    );
                            } else {
                                if inputs != 1 {
                                    todo!()
                                }
                                fun_vars.push(Number::default());
                                *stack[len - inputs].num_mut() = compute.numerical_nth_integral(
                                    fun_vars,
                                    stack,
                                    d.get_num(),
                                    Number::default(),
                                    stack[len - inputs].num_ref().clone(),
                                    fun_vars.len() - 1,
                                );
                            }
                        } else {
                            if inputs != 1 {
                                todo!()
                            }
                            fun_vars.push(Number::default());
                            *stack[len - inputs].num_mut() = compute.numerical_nth_derivative(
                                fun_vars,
                                stack,
                                d.get_num(),
                                stack[len - inputs].num_ref().clone(),
                                fun_vars.len() - 1,
                            );
                        }
                    } else {
                        fun_vars.push(stack[len - inputs].num_ref().clone());
                        fun_vars.extend(stack.drain(len + 1 - inputs..).map(|n| n.num()));
                        *stack[len - inputs].num_mut() =
                            compute.compute_buffer_with(fun_vars, stack);
                    }
                    fun_vars.drain(end..);
                }
                &Token::InnerVar(index) => {
                    stack.push(Token::Num(fun_vars[self.offset + index as usize].clone()))
                }
                &Token::GraphVar(index) => {
                    stack.push(Token::Num(self.vars[index as usize].clone()))
                }
                &Token::Skip(to) => {
                    stack.push(Token::Skip(i + 1));
                    tokens.nth(to - 1);
                }
                Token::Num(n) => stack.push(Token::Num(n.clone())),
                Token::Polynomial(_) => unreachable!(),
            }
        }
        stack.pop().unwrap().num()
    }
}
