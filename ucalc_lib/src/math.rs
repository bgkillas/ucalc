use crate::compute::Compute;
use crate::{Number, Tokens};
use ucalc_numbers::{Float, FloatTrait, UInteger};
impl Compute<'_> {
    pub fn numerical_solve(
        self,
        fun_vars: &mut Vec<Number>,
        stack: &mut Tokens,
        point: Number,
        var: usize,
    ) -> Number {
        fun_vars[var] = point;
        for _ in 0..64 {
            let y = self.compute_buffer_with(fun_vars, stack);
            if y.is_zero() {
                break;
            }
            let val = self.numerical_derivative(fun_vars, stack, fun_vars[var].clone(), var);
            fun_vars[var] -= y / val;
        }
        fun_vars[var].clone()
    }
    pub fn numerical_derivative(
        self,
        fun_vars: &mut Vec<Number>,
        stack: &mut Tokens,
        point: Number,
        var: usize,
    ) -> Number {
        let epsilon = Float::from(2.0f64.powi(-32));
        fun_vars[var] = point.clone() - &epsilon;
        let start = self.compute_buffer_with(fun_vars, stack);
        fun_vars[var] = point + &epsilon;
        let end = self.compute_buffer_with(fun_vars, stack);
        (end - start) / (Float::from(2) * epsilon)
    }
    pub fn numerical_nth_derivative(
        self,
        fun_vars: &mut Vec<Number>,
        stack: &mut Tokens,
        point: Number,
        var: usize,
        n: u8,
    ) -> Number {
        let e = -32;
        let e = e - e % n as i32;
        let epsilon = Float::from(2.0f64.powi(e / n as i32));
        let mut sum = Number::default();
        fun_vars[var] = point;
        for k in 0..=n {
            let r = Float::from(
                UInteger::from(n as usize)
                    .binomial(UInteger::from(k as usize))
                    .0,
            );
            if (n - k).is_multiple_of(2) {
                sum += self.compute_buffer_with(fun_vars, stack) * r;
            } else {
                sum -= self.compute_buffer_with(fun_vars, stack) * r;
            }
            if k != n {
                fun_vars[var] += &epsilon;
            }
        }
        let epsilon = Float::from(2.0f64.powi(e));
        sum / epsilon
    }
    pub fn numerical_integral(
        self,
        fun_vars: &mut Vec<Number>,
        stack: &mut Tokens,
        start: Number,
        end: Number,
        var: usize,
    ) -> Number {
        let n = 1024;
        let epsilon = (end - &start) / Float::from(n);
        let mut total = Number::from(0);
        fun_vars[var] = start;
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
        x_0: Number,
        t_0: Number,
        t_1: Number,
        x_var: usize,
        t_var: usize,
    ) -> Number {
        let n = 1024;
        let epsilon = (t_1 - &t_0) / Float::from(n);
        fun_vars[x_var] = x_0;
        fun_vars[t_var] = t_0;
        for _ in 0..n {
            let delta = self.compute_buffer_with(fun_vars, stack) * &epsilon;
            fun_vars[x_var] += delta;
            fun_vars[t_var] += &epsilon;
        }
        fun_vars[x_var].clone()
    }
}
