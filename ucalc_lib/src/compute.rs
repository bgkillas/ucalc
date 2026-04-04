use crate::parse::{Token, Tokens, TokensSlice};
#[cfg(feature = "float_rand")]
use crate::rand::Rand;
use crate::{FunctionVar, Number, Variable};
use std::array;
use ucalc_numbers::{Constant, FloatTrait};
#[derive(Copy, Clone, Debug)]
pub(crate) struct Compute<'a> {
    pub(crate) tokens: &'a TokensSlice,
    pub(crate) graph_vars: &'a [Number],
    pub(crate) custom_funs: &'a [FunctionVar],
    pub(crate) custom_vars: &'a [Variable],
    pub(crate) offset: usize,
}
impl Tokens {
    pub fn compute(
        &self,
        vars: &[Number],
        funs: &[FunctionVar],
        custom_vars: &[Variable],
        #[cfg(feature = "float_rand")] rand: &mut Option<Rand>,
    ) -> Number {
        let cap = self.len() + funs.iter().map(|c| c.tokens.len()).sum::<usize>();
        let mut inner_vars = Vec::with_capacity(cap);
        let mut stack = Tokens(Vec::with_capacity(cap));
        self.compute_buffer(
            &mut inner_vars,
            vars,
            funs,
            custom_vars,
            &mut stack,
            #[cfg(feature = "float_rand")]
            rand,
        )
    }
    pub fn compute_buffer(
        &self,
        inner_vars: &mut Vec<Number>,
        vars: &[Number],
        funs: &[FunctionVar],
        custom_vars: &[Variable],
        stack: &mut Tokens,
        #[cfg(feature = "float_rand")] rand: &mut Option<Rand>,
    ) -> Number {
        self.compute_buffer_with(
            inner_vars,
            vars,
            funs,
            custom_vars,
            stack,
            0,
            #[cfg(feature = "float_rand")]
            rand,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn compute_buffer_with(
        &self,
        inner_vars: &mut Vec<Number>,
        vars: &[Number],
        funs: &[FunctionVar],
        custom_vars: &[Variable],
        stack: &mut Tokens,
        offset: usize,
        #[cfg(feature = "float_rand")] rand: &mut Option<Rand>,
    ) -> Number {
        Compute::new(&self[..], vars, funs, custom_vars, offset).compute_buffer_with(
            inner_vars,
            stack,
            #[cfg(feature = "float_rand")]
            rand,
        )
    }
    pub(crate) fn get_skip_mut<'a, const N: usize, const K: usize>(
        &mut self,
        tokens: &'a TokensSlice,
    ) -> (&mut Number, [Number; K], [&'a TokensSlice; N]) {
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
        tokens: &'a TokensSlice,
    ) -> [&'a TokensSlice; N] {
        let len = tokens.len();
        let mut t = len - 1;
        let mut arr = array::from_fn(|_| &tokens[0..0]);
        for i in 0..N {
            let back = self.pop().unwrap().skip();
            let ret = &tokens[back..t];
            t = back.saturating_sub(1);
            arr[N - (i + 1)] = ret
        }
        arr
    }
    pub(crate) fn get_skip_tokens_keep_one<'a, const N: usize>(
        &mut self,
        tokens: &'a TokensSlice,
    ) -> [&'a TokensSlice; N] {
        let len = tokens.len();
        let mut t = len - 1;
        let mut arr = array::from_fn(|_| &tokens[0..0]);
        for i in 0..N - 1 {
            let back = self.pop().unwrap().skip();
            let ret = &tokens[back..t];
            t = back.saturating_sub(1);
            arr[N - (i + 1)] = ret
        }
        let back = self[self.len() - 1].skip();
        let ret = &tokens[back..t];
        arr[0] = ret;
        arr
    }
    pub(crate) fn get_skip_tokens_keep_one_vec<'a>(
        &mut self,
        tokens: &'a TokensSlice,
        n: usize,
    ) -> Vec<&'a TokensSlice> {
        let len = tokens.len();
        let mut t = len - 1;
        let mut arr = vec![&tokens[0..0]; n];
        for i in 0..n - 1 {
            let back = self.pop().unwrap().skip();
            let ret = &tokens[back..t];
            t = back.saturating_sub(1);
            arr[n - (i + 1)] = ret
        }
        let back = self[self.len() - 1].skip();
        let ret = &tokens[back..t];
        arr[0] = ret;
        arr
    }
}
impl<'a> Compute<'a> {
    pub fn offset(self, offset: usize) -> Self {
        Self {
            tokens: self.tokens,
            graph_vars: self.graph_vars,
            custom_funs: self.custom_funs,
            custom_vars: self.custom_vars,
            offset,
        }
    }
    pub fn tokens<'b: 'a>(self, tokens: &'b TokensSlice) -> Self {
        Self {
            tokens,
            graph_vars: self.graph_vars,
            custom_funs: self.custom_funs,
            custom_vars: self.custom_vars,
            offset: self.offset,
        }
    }
    pub fn new(
        tokens: &'a TokensSlice,
        vars: &'a [Number],
        funs: &'a [FunctionVar],
        custom_vars: &'a [Variable],
        offset: usize,
    ) -> Self {
        Self {
            tokens,
            graph_vars: vars,
            custom_funs: funs,
            custom_vars,
            offset,
        }
    }
    pub fn compute_buffer_with(
        self,
        inner_vars: &mut Vec<Number>,
        stack: &mut Tokens,
        #[cfg(feature = "float_rand")] rand: &mut Option<Rand>,
    ) -> Number {
        let mut tokens = self.tokens.iter().enumerate();
        while let Some((i, token)) = tokens.next() {
            match token {
                &Token::Function(operator, d) => {
                    if d.get() != 0 {
                        todo!()
                    }
                    let inputs = operator.inputs();
                    if operator.has_inner_fn() {
                        operator.compute_var(
                            self.tokens(&self.tokens[..=i]),
                            stack,
                            inner_vars,
                            #[cfg(feature = "float_rand")]
                            rand,
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
                        operator.compute(
                            stack,
                            inputs,
                            #[cfg(feature = "float_rand")]
                            rand,
                        );
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
                        operator.compute(
                            stack,
                            inputs,
                            #[cfg(feature = "float_rand")]
                            rand,
                        );
                    }
                }
                &Token::CustomVar(index) => {
                    stack.push(self.custom_vars[index as usize].value.clone().into())
                }
                &Token::CustomFun(index, d) => {
                    let inputs = self.custom_funs[index as usize].inputs.get() as usize;
                    let end = inner_vars.len();
                    let len = stack.len();
                    let compute = self
                        .offset(end)
                        .tokens(&self.custom_funs[index as usize].tokens[..]);
                    if d.get() != 0 {
                        if d.is_integral() {
                            if d.is_integral_twice_input() {
                                if inputs != 1 {
                                    todo!()
                                }
                                inner_vars.push(Number::default());
                                let end = stack.pop().unwrap().num();
                                *stack.last_mut().unwrap().num_mut() = compute
                                    .numerical_nth_integral(
                                        inner_vars,
                                        stack,
                                        d.get(),
                                        stack.last().unwrap().num_ref().clone(),
                                        end,
                                        inner_vars.len() - 1,
                                        #[cfg(feature = "float_rand")]
                                        rand,
                                    );
                            } else {
                                if inputs != 1 {
                                    todo!()
                                }
                                inner_vars.push(Number::default());
                                *stack[len - inputs].num_mut() = compute.numerical_nth_integral(
                                    inner_vars,
                                    stack,
                                    d.get(),
                                    Number::default(),
                                    stack[len - inputs].num_ref().clone(),
                                    inner_vars.len() - 1,
                                    #[cfg(feature = "float_rand")]
                                    rand,
                                );
                            }
                        } else {
                            if inputs != 1 {
                                todo!()
                            }
                            inner_vars.push(Number::default());
                            *stack[len - inputs].num_mut() = compute.numerical_nth_derivative(
                                inner_vars,
                                stack,
                                d.get(),
                                stack[len - inputs].num_ref().clone(),
                                inner_vars.len() - 1,
                                #[cfg(feature = "float_rand")]
                                rand,
                            );
                        }
                    } else {
                        inner_vars.push(stack[len - inputs].num_ref().clone());
                        inner_vars.extend(stack.drain(len + 1 - inputs..).map(|n| n.num()));
                        *stack[len - inputs].num_mut() = compute.compute_buffer_with(
                            inner_vars,
                            stack,
                            #[cfg(feature = "float_rand")]
                            rand,
                        );
                    }
                    inner_vars.drain(end..);
                }
                &Token::InnerVar(index) => {
                    stack.push(inner_vars[self.offset + index as usize].clone().into())
                }
                &Token::GraphVar(index) => {
                    stack.push(self.graph_vars[index as usize].clone().into())
                }
                &Token::Skip(to) => {
                    stack.push(Token::Skip(i + 1));
                    tokens.nth(to - 1);
                }
                Token::Num(n) => stack.push(n.clone().into()),
                Token::Polynomial(_) => unreachable!(),
            }
        }
        stack.pop().unwrap().num()
    }
}
