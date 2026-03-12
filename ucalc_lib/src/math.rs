use crate::parse::TokensRef;
use crate::{Functions, Number, Tokens, Variables};
use ucalc_numbers::Float;
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
            fun_vars[var] = fun_vars[var].clone()
                - self.compute_buffer_with(fun_vars, vars, funs, custom_vars, stack, offset)
                    / self.numerical_derivative(
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
        fun_vars[var] = point.clone();
        let epsilon = Float::from(2.0f64.powi(-32));
        let start = self.compute_buffer_with(fun_vars, vars, funs, custom_vars, stack, offset);
        fun_vars[var] += &epsilon;
        let end = self.compute_buffer_with(fun_vars, vars, funs, custom_vars, stack, offset);
        (end - start) / epsilon
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
        for _ in 0..n {
            fun_vars[var] += &epsilon;
            let cur = self.compute_buffer_with(fun_vars, vars, funs, custom_vars, stack, offset);
            total += (last + &cur) / Float::from(2) * &epsilon;
            last = cur;
        }
        total
    }
}
