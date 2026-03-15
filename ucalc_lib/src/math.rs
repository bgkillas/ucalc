use crate::compute::Compute;
use crate::{Number, Tokens};
use ucalc_numbers::{Float, FloatTrait};
impl Compute<'_> {
    pub fn numerical_solve(
        self,
        fun_vars: &mut Vec<Number>,
        stack: &mut Tokens,
        point: &Number,
        var: usize,
    ) -> Number {
        fun_vars[var] = point.clone();
        for _ in 0..64 {
            let y = self.compute_buffer_with(fun_vars, stack);
            if y.is_zero() {
                break;
            }
            fun_vars[var] = fun_vars[var].clone()
                - y / self.numerical_derivative(fun_vars, stack, &fun_vars[var].clone(), var);
        }
        fun_vars[var].clone()
    }
    pub fn numerical_derivative(
        self,
        fun_vars: &mut Vec<Number>,
        stack: &mut Tokens,
        point: &Number,
        var: usize,
    ) -> Number {
        let epsilon = Float::from(2.0f64.powi(-32));
        fun_vars[var] = point.clone() - &epsilon;
        let start = self.compute_buffer_with(fun_vars, stack);
        fun_vars[var] = point.clone() + &epsilon;
        let end = self.compute_buffer_with(fun_vars, stack);
        (end - start) / (Float::from(2) * epsilon)
    }
    pub fn numerical_integral(
        self,
        fun_vars: &mut Vec<Number>,
        stack: &mut Tokens,
        start: &Number,
        end: &Number,
        var: usize,
    ) -> Number {
        let n = 1024;
        let epsilon = (end.clone() - start) / Float::from(n);
        let mut total = Number::from(0);
        fun_vars[var] = start.clone();
        let mut last = self.compute_buffer_with(fun_vars, stack);
        let mid = epsilon.clone() / Float::from(2);
        for _ in 0..n {
            fun_vars[var] += &epsilon;
            let cur = self.compute_buffer_with(fun_vars, stack);
            total += (last + &cur) * &mid;
            last = cur;
        }
        total
    }
    #[allow(clippy::too_many_arguments)]
    pub fn numerical_differential(
        self,
        fun_vars: &mut Vec<Number>,
        stack: &mut Tokens,
        x_0: &Number,
        t_0: &Number,
        t_1: &Number,
        x_var: usize,
        t_var: usize,
    ) -> Number {
        let n = 1024;
        let epsilon = (t_1.clone() - t_0) / Float::from(n);
        fun_vars[x_var] = x_0.clone();
        fun_vars[t_var] = t_0.clone();
        for _ in 0..n {
            let delta = self.compute_buffer_with(fun_vars, stack) * &epsilon;
            fun_vars[x_var] += delta;
            fun_vars[t_var] += &epsilon;
        }
        fun_vars[x_var].clone()
    }
}
