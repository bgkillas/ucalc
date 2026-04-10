use crate::parse::{Token, Tokens, TokensSlice};
use crate::polynomial::Polynomial;
#[cfg(feature = "float_rand")]
use crate::rand::Rand;
use crate::{FunctionVar, Number, Variable};
use std::array;
use ucalc_numbers::{Constant, FloatTrait};
#[derive(Clone, Debug)]
pub struct Compute<'a> {
    pub(crate) tokens: &'a TokensSlice,
    pub(crate) graph_vars: &'a [Number],
    pub(crate) custom_funs: &'a [FunctionVar],
    pub(crate) custom_vars: &'a [Variable],
    pub(crate) offset: usize,
}
impl Tokens {
    pub fn compute(
        &self,
        graph_vars: &[Number],
        custom_funs: &[FunctionVar],
        custom_vars: &[Variable],
        #[cfg(feature = "float_rand")] rand: &mut Rand,
    ) -> Number {
        let cap = self.len() + custom_funs.iter().map(|c| c.tokens.len()).sum::<usize>();
        let mut inner_vars = Vec::with_capacity(cap);
        let mut stack = Vec::with_capacity(cap);
        self.compute_buffer(
            &mut inner_vars,
            graph_vars,
            custom_funs,
            custom_vars,
            &mut stack,
            #[cfg(feature = "float_rand")]
            rand,
        )
    }
    pub fn compute_fun(
        &self,
        graph_vars: &[Number],
        custom_funs: &[FunctionVar],
        custom_vars: &[Variable],
        drain: impl Iterator<Item = Number>,
        #[cfg(feature = "float_rand")] rand: &mut Rand,
    ) -> Number {
        let cap = self.len() + custom_funs.iter().map(|c| c.tokens.len()).sum::<usize>();
        let mut inner_vars = Vec::with_capacity(cap);
        inner_vars.extend(drain);
        let mut stack = Vec::with_capacity(cap);
        self.compute_buffer(
            &mut inner_vars,
            graph_vars,
            custom_funs,
            custom_vars,
            &mut stack,
            #[cfg(feature = "float_rand")]
            rand,
        )
    }
    pub fn compute_buffer(
        &self,
        inner_vars: &mut Vec<Number>,
        graph_vars: &[Number],
        custom_funs: &[FunctionVar],
        custom_vars: &[Variable],
        stack: &mut Vec<StackToken>,
        #[cfg(feature = "float_rand")] rand: &mut Rand,
    ) -> Number {
        self.compute_buffer_with(
            inner_vars,
            graph_vars,
            custom_funs,
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
        graph_vars: &[Number],
        custom_funs: &[FunctionVar],
        custom_vars: &[Variable],
        stack: &mut Vec<StackToken>,
        offset: usize,
        #[cfg(feature = "float_rand")] rand: &mut Rand,
    ) -> Number {
        Compute::new(&self[..], graph_vars, custom_funs, custom_vars, offset).compute(
            inner_vars,
            stack,
            #[cfg(feature = "float_rand")]
            rand,
        )
    }
}
pub fn get_skip_var<const N: usize>(stack: &mut Vec<StackToken>) -> [Number; N] {
    let mut arr = array::from_fn(|_| Number::default());
    for i in 0..N {
        arr[N - (i + 1)] = stack.pop().unwrap().num();
    }
    arr
}
impl TokensSlice {
    pub(crate) fn get_skip_mut<'a, const N: usize, const K: usize>(
        &self,
        stack: &'a mut Vec<StackToken>,
    ) -> (&'a mut Number, [Number; K], [&TokensSlice; N]) {
        let tokens = self.get_skip_tokens(stack);
        let args = get_skip_var(stack);
        (stack.last_mut().unwrap().num_mut(), args, tokens)
    }
    pub(crate) fn get_skip_tokens<const N: usize>(
        &self,
        stack: &mut Vec<StackToken>,
    ) -> [&TokensSlice; N] {
        let len = self.len();
        let mut t = len - 1;
        let mut arr = array::from_fn(|_| &self[0..0]);
        for i in 0..N {
            let back = stack.pop().unwrap().skip();
            let ret = &self[back..t];
            t = back.saturating_sub(1);
            arr[N - (i + 1)] = ret
        }
        arr
    }
    pub(crate) fn get_skip_tokens_keep_one<const N: usize>(
        &self,
        stack: &mut Vec<StackToken>,
    ) -> [&TokensSlice; N] {
        let len = self.len();
        let mut t = len - 1;
        let mut arr = array::from_fn(|_| &self[0..0]);
        for i in 0..N - 1 {
            let back = stack.pop().unwrap().skip();
            let ret = &self[back..t];
            t = back.saturating_sub(1);
            arr[N - (i + 1)] = ret
        }
        let back = stack[stack.len() - 1].skip();
        let ret = &self[back..t];
        arr[0] = ret;
        arr
    }
    pub(crate) fn get_skip_tokens_keep_one_vec(
        &self,
        stack: &mut Vec<StackToken>,
        n: usize,
    ) -> Vec<&TokensSlice> {
        let len = self.len();
        let mut t = len - 1;
        let mut arr = vec![&self[0..0]; n];
        for i in 0..n - 1 {
            let back = stack.pop().unwrap().skip();
            let ret = &self[back..t];
            t = back.saturating_sub(1);
            arr[n - (i + 1)] = ret
        }
        let back = stack[stack.len() - 1].skip();
        let ret = &self[back..t];
        arr[0] = ret;
        arr
    }
}
impl<'a> Compute<'a> {
    pub fn offset(&self, offset: usize) -> Self {
        Self {
            tokens: self.tokens,
            graph_vars: self.graph_vars,
            custom_funs: self.custom_funs,
            custom_vars: self.custom_vars,
            offset,
        }
    }
    pub fn tokens<'b: 'a>(&self, tokens: &'b TokensSlice) -> Self {
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
        graph_vars: &'a [Number],
        custom_funs: &'a [FunctionVar],
        custom_vars: &'a [Variable],
        offset: usize,
    ) -> Self {
        Self {
            tokens,
            graph_vars,
            custom_funs,
            custom_vars,
            offset,
        }
    }
    pub fn compute(
        &self,
        inner_vars: &mut Vec<Number>,
        stack: &mut Vec<StackToken>,
        #[cfg(feature = "float_rand")] rand: &mut Rand,
    ) -> Number {
        let mut tokens = self.tokens.iter().enumerate();
        while let Some((i, token)) = tokens.next() {
            match token {
                &Token::Function(fun, d) => {
                    if d.get() != 0 {
                        todo!()
                    }
                    let inputs = fun.inputs();
                    if fun.has_inner_fn() {
                        fun.compute_var(
                            self.tokens(&self.tokens[..=i]),
                            stack,
                            inner_vars,
                            #[cfg(feature = "float_rand")]
                            rand,
                        )
                    } else if fun.is_chainable() {
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
                        fun.compute(
                            stack,
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
                        fun.compute(
                            stack,
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
                        *stack[len - inputs].num_mut() = compute.compute(
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
                    stack.push(StackToken::Skip(i + 1));
                    tokens.nth(to - 1);
                }
                Token::Number(n) => stack.push(n.clone().into()),
            }
        }
        stack.pop().unwrap().num()
    }
}
pub enum StackToken {
    Number(Number),
    Polynomial(Box<Polynomial>),
    Skip(usize),
}
impl StackToken {
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
    pub fn num_ref(&self) -> &Number {
        let Self::Number(num) = self else {
            unreachable!()
        };
        num
    }
    pub fn num_mut(&mut self) -> &mut Number {
        let Self::Number(num) = self else {
            unreachable!()
        };
        num
    }
    pub fn poly_mut(&mut self) -> &mut Polynomial {
        let Self::Polynomial(poly) = self else {
            unreachable!()
        };
        poly
    }
    pub fn poly(self) -> Box<Polynomial> {
        let Self::Polynomial(poly) = self else {
            unreachable!()
        };
        poly
    }
    pub fn poly_ref(&self) -> &Polynomial {
        let Self::Polynomial(poly) = self else {
            unreachable!()
        };
        poly
    }
}
impl From<Number> for StackToken {
    fn from(value: Number) -> Self {
        Self::Number(value)
    }
}
impl From<Polynomial> for StackToken {
    fn from(value: Polynomial) -> Self {
        Self::Polynomial(value.into())
    }
}
