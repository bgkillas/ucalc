use crate::parse::{Token, Tokens, TokensRef};
use crate::{Functions, Number, Variables};
use std::array;
use ucalc_numbers::{Constant, Float, FloatTrait, RealTrait};
impl Tokens {
    pub fn compute(&self, vars: &[Number], funs: &Functions, custom_vars: &Variables) -> Number {
        TokensRef(self).compute(vars, funs, custom_vars)
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
        TokensRef(self).compute_buffer_with(fun_vars, vars, funs, custom_vars, stack, offset)
    }
    pub fn get_skip<'a, 'b, const N: usize, const K: usize>(
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
    pub fn get_skip_tokens<'a, const N: usize>(&self, tokens: TokensRef<'a>) -> [TokensRef<'a>; N] {
        let len = tokens.len();
        let mut t = len - 1;
        let mut l = 0;
        let mut arr = array::from_fn(|i| {
            let back = self[self.len() - (i + 1)].skip();
            let ret = TokensRef(&tokens.0[back..t]);
            l += t - back + 1;
            t = back.saturating_sub(1);
            ret
        });
        arr.reverse();
        arr
    }
    #[allow(clippy::too_many_arguments)]
    pub fn range(
        &mut self,
        tokens: TokensRef,
        fun_vars: &mut Vec<Number>,
        vars: &[Number],
        funs: &Functions,
        custom_vars: &Variables,
        offset: usize,
        fun: impl FnOnce(&mut dyn Iterator<Item = Number>) -> Token,
    ) {
        let len = self.len();
        let ([start, end], [tokens]) = self.get_skip(tokens);
        let start = start.num_ref().real().clone().into_isize();
        let end = end.num_ref().real().clone().into_isize();
        fun_vars.push(Number::from(start));
        let mut stack = Tokens(Vec::with_capacity(tokens.len()));
        let mut iter = (start..=end).map(|_| {
            let ret =
                tokens.compute_buffer_with(fun_vars, vars, funs, custom_vars, &mut stack, offset);
            *fun_vars.last_mut().unwrap() += Float::from(1);
            ret
        });
        self[len - 3] = fun(&mut iter);
        fun_vars.pop().unwrap();
        self.drain(len - 2..);
    }
}
impl TokensRef<'_> {
    pub fn compute(self, vars: &[Number], funs: &Functions, custom_vars: &Variables) -> Number {
        let mut fun_vars = Vec::with_capacity(self.len());
        let mut stack = Tokens(Vec::with_capacity(self.len()));
        self.compute_buffer(&mut fun_vars, vars, funs, custom_vars, &mut stack)
    }
    pub fn compute_with(
        self,
        vars: &[Number],
        funs: &Functions,
        custom_vars: &Variables,
        offset: usize,
    ) -> Number {
        let mut fun_vars = Vec::with_capacity(self.len());
        let mut stack = Tokens(Vec::with_capacity(self.len()));
        self.compute_buffer_with(&mut fun_vars, vars, funs, custom_vars, &mut stack, offset)
    }
    pub fn compute_buffer(
        self,
        fun_vars: &mut Vec<Number>,
        vars: &[Number],
        funs: &Functions,
        custom_vars: &Variables,
        stack: &mut Tokens,
    ) -> Number {
        self.compute_buffer_with(fun_vars, vars, funs, custom_vars, stack, 0)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn compute_buffer_with(
        self,
        fun_vars: &mut Vec<Number>,
        vars: &[Number],
        funs: &Functions,
        custom_vars: &Variables,
        stack: &mut Tokens,
        offset: usize,
    ) -> Number {
        let mut i = 0;
        while i < self.len() {
            let len = stack.len();
            match &self[i] {
                Token::Function(operator) => {
                    let inputs = operator.inputs() as usize;
                    if operator.has_inner_fn() {
                        operator.compute_var(
                            TokensRef(&self[..=i]),
                            stack,
                            fun_vars,
                            vars,
                            funs,
                            custom_vars,
                            offset,
                        )
                    } else if operator.is_chainable() {
                        let chain = if self.get(i + 1).is_some_and(|o| {
                            if let Token::Function(o) = o {
                                o.is_chainable()
                            } else {
                                false
                            }
                        }) {
                            Some(self[len - inputs].num_ref())
                        } else {
                            None
                        };
                        operator.compute(&mut stack[len - inputs..]);
                        if let Some(b) = chain {
                            let a = stack[len - inputs].num_mut();
                            *a = if a.is_zero() {
                                Number::from(Constant::Nan)
                            } else {
                                b.clone()
                            };
                        }
                        stack.drain(len + 1 - inputs..);
                    } else {
                        operator.compute(&mut stack[len - inputs..]);
                        stack.drain(len + 1 - inputs..);
                    }
                }
                Token::Var(index) => {
                    stack.push(Token::Num(custom_vars[*index as usize].value.clone()))
                }
                Token::Fun(index) => {
                    let inputs = funs[*index as usize].inputs as usize;
                    let end = fun_vars.len();
                    fun_vars.push(stack[len - inputs].num_ref().clone());
                    fun_vars.extend(stack.drain(len + 1 - inputs..).map(|n| n.num()));
                    *stack[len - inputs].num_mut() =
                        funs[*index as usize].tokens.compute_buffer_with(
                            fun_vars,
                            vars,
                            funs,
                            custom_vars,
                            &mut Tokens(Vec::with_capacity(funs[*index as usize].tokens.len())),
                            end,
                        );
                    fun_vars.drain(end..);
                }
                Token::InnerVar(index) => {
                    stack.push(Token::Num(fun_vars[offset + *index as usize].clone()))
                }
                Token::GraphVar(index) => stack.push(Token::Num(vars[*index as usize].clone())),
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
