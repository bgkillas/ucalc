#[cfg(feature = "float_rand")]
use crate::Rand;
use crate::{Functions, Token, Tokens, Variables, Volatility};
use std::ops::IndexMut;
impl Tokens {
    pub fn simplify<const V: Volatility>(
        &mut self,
        vars: &mut Variables,
        funs: &mut Functions,
        _inputs: u8,
        #[cfg(feature = "float_rand")] rand: &mut Rand,
    ) {
        let mut skips = Vec::with_capacity(self.len());
        let mut i = 0;
        while i < self.len() {
            match self[i] {
                Token::Function(fun, d) => {
                    let inputs = fun.inputs().get();
                    if fun.volatility() < V
                        && self[i - inputs as usize..i]
                            .iter()
                            .all(|n| matches!(n, Token::Number(_)))
                        && d.get() == 0
                    {
                        if fun.has_inner_fn() {
                            //TODO
                            let _ = 0;
                            skips.pop();
                        } else if fun.is_chainable() {
                            //TODO
                        } else {
                            let n = fun
                                .compute_drain(
                                    self.drain(i - inputs as usize..i).map(|t| t.num()),
                                    #[cfg(feature = "float_rand")]
                                    rand,
                                )
                                .into();
                            i -= inputs as usize;
                            for i in skips.iter().copied() {
                                *<Tokens as IndexMut<usize>>::index_mut(self, i).skip_mut() -=
                                    inputs as usize;
                            }
                            self[i] = n;
                        }
                    }
                }
                Token::CustomFun(index, d) => {
                    let fun = &funs[index as usize];
                    let inputs = fun.inputs.get();
                    if fun.volatile < V
                        && !fun.tokens.is_empty()
                        && self[i - inputs as usize..i]
                            .iter()
                            .all(|n| matches!(n, Token::Number(_)))
                        && d.get() == 0
                    {
                        let n = fun.tokens.compute_fun(
                            &[],
                            funs,
                            vars,
                            self.drain(i - inputs as usize..i).map(|t| t.num()),
                            #[cfg(feature = "float_rand")]
                            rand,
                        );
                        i -= inputs as usize;
                        for i in skips.iter().copied() {
                            *<Tokens as IndexMut<usize>>::index_mut(self, i).skip_mut() -=
                                inputs as usize;
                        }
                        self[i] = n.into();
                    }
                }
                Token::CustomVar(index) if vars[index as usize].volatile < V => {
                    self[i] = vars[index as usize].value.clone().into()
                }
                Token::Skip(_) => {
                    if let Some(n) = skips.last().copied()
                        && n + self[n].skip() + 1 == i
                    {
                        skips.pop();
                    }
                    skips.push(i)
                }
                _ => {}
            }
            i += 1;
        }
    }
}
