use crate::parse::TokensRef;
use crate::{Functions, Number, Tokens, Variables};
use ucalc_numbers::{Float, FloatTrait};
impl TokensRef<'_> {
    #[allow(clippy::too_many_arguments)]
    pub fn numerical_solve(
        self,
        fun_vars: &mut Vec<Number>,
        vars: &[Number],
        funs: &Functions,
        custom_vars: &Variables,
        stack: &mut Tokens,
        offset: usize,
        point: &Number,
        var: usize,
    ) -> Number {
        fun_vars[var] = point.clone();
        for _ in 0..64 {
            let y = self.compute_buffer_with(fun_vars, vars, funs, custom_vars, stack, offset);
            if y.is_zero() {
                break;
            }
            fun_vars[var] = fun_vars[var].clone()
                - y / self.numerical_derivative(
                    fun_vars,
                    vars,
                    funs,
                    custom_vars,
                    stack,
                    offset,
                    &fun_vars[var].clone(),
                    var,
                );
        }
        fun_vars[var].clone()
    }
    #[allow(clippy::too_many_arguments)]
    pub fn numerical_derivative(
        self,
        fun_vars: &mut Vec<Number>,
        vars: &[Number],
        funs: &Functions,
        custom_vars: &Variables,
        stack: &mut Tokens,
        offset: usize,
        point: &Number,
        var: usize,
    ) -> Number {
        let epsilon = Float::from(2.0f64.powi(-32));
        fun_vars[var] = point.clone() - &epsilon;
        let start = self.compute_buffer_with(fun_vars, vars, funs, custom_vars, stack, offset);
        fun_vars[var] = point.clone() + &epsilon;
        let end = self.compute_buffer_with(fun_vars, vars, funs, custom_vars, stack, offset);
        (end - start) / (Float::from(2) * epsilon)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn numerical_integral(
        self,
        fun_vars: &mut Vec<Number>,
        vars: &[Number],
        funs: &Functions,
        custom_vars: &Variables,
        stack: &mut Tokens,
        offset: usize,
        start: &Number,
        end: &Number,
        var: usize,
    ) -> Number {
        let n = 1024;
        let epsilon = (end.clone() - start) / Float::from(n);
        let mut total = Number::from(0);
        fun_vars[var] = start.clone();
        let mut last = self.compute_buffer_with(fun_vars, vars, funs, custom_vars, stack, offset);
        let mid = epsilon.clone() / Float::from(2);
        for _ in 0..n {
            fun_vars[var] += &epsilon;
            let cur = self.compute_buffer_with(fun_vars, vars, funs, custom_vars, stack, offset);
            total += (last + &cur) * &mid;
            last = cur;
        }
        total
    }
}
