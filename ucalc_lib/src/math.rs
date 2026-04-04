use crate::compute::Compute;
use crate::{Number, Tokens};
use ucalc_numbers::{Float, FloatFunctions, FloatTrait, UInteger};
impl Compute<'_> {
    pub fn numerical_solve(
        self,
        inner_vars: &mut Vec<Number>,
        stack: &mut Tokens,
        point: Number,
        var: usize,
    ) -> Number {
        inner_vars[var] = point;
        for _ in 0..64 {
            let y = self.compute_buffer_with(inner_vars, stack);
            if y.is_zero() {
                break;
            }
            let val = self.numerical_derivative(inner_vars, stack, inner_vars[var].clone(), var);
            inner_vars[var] -= val.recip() * y;
        }
        inner_vars[var].clone()
    }
    pub fn numerical_derivative(
        self,
        inner_vars: &mut Vec<Number>,
        stack: &mut Tokens,
        point: Number,
        var: usize,
    ) -> Number {
        let epsilon = Float::from(2.0f64.powi(-32));
        inner_vars[var] = point.clone() - &epsilon;
        let start = self.compute_buffer_with(inner_vars, stack);
        inner_vars[var] = point + &epsilon;
        let end = self.compute_buffer_with(inner_vars, stack);
        (end - start) / (Float::from(2) * epsilon)
    }
    pub fn numerical_nth_derivative(
        self,
        inner_vars: &mut Vec<Number>,
        stack: &mut Tokens,
        n: u8,
        point: Number,
        var: usize,
    ) -> Number {
        let e = -32;
        let e = e - e % n as i32;
        let epsilon = Float::from(2.0f64.powi(e / n as i32));
        let mut sum = Number::default();
        inner_vars[var] = point;
        for k in 0..=n {
            let r = Float::from(
                UInteger::from(n as usize)
                    .binomial(UInteger::from(k as usize))
                    .0,
            );
            if (n - k).is_multiple_of(2) {
                sum += self.compute_buffer_with(inner_vars, stack) * r;
            } else {
                sum -= self.compute_buffer_with(inner_vars, stack) * r;
            }
            if k != n {
                inner_vars[var] += &epsilon;
            }
        }
        let epsilon = Float::from(2.0f64.powi(e));
        sum / epsilon
    }
    pub fn numerical_integral(
        self,
        inner_vars: &mut Vec<Number>,
        stack: &mut Tokens,
        start: Number,
        end: Number,
        var: usize,
    ) -> Number {
        let k = 1024;
        let epsilon = (end - &start) / Float::from(k);
        let mut total = Number::from(0);
        inner_vars[var] = start;
        let mut last = self.compute_buffer_with(inner_vars, stack);
        let mid = epsilon.clone() / Float::from(2);
        for _ in 1..=k {
            inner_vars[var] += &epsilon;
            let cur = self.compute_buffer_with(inner_vars, stack);
            total += (last + &cur) * &mid;
            last = cur;
        }
        total
    }
    pub fn numerical_nth_integral(
        self,
        inner_vars: &mut Vec<Number>,
        stack: &mut Tokens,
        n: u8,
        start: Number,
        end: Number,
        var: usize,
    ) -> Number {
        if n == 0 {
            inner_vars[var] = end;
            let end = self.compute_buffer_with(inner_vars, stack);
            inner_vars[var] = start;
            let start = self.compute_buffer_with(inner_vars, stack);
            return end - start;
        }
        let k = 1024;
        let kf = Float::from(k);
        let fact = Float::from(UInteger::from((n - 1) as usize).factorial().0);
        let epsilon = (end - &start) / &kf;
        let mut total = Number::from(0);
        inner_vars[var] = start;
        let mut last = self.compute_buffer_with(inner_vars, stack);
        for _ in 1..n {
            last *= &epsilon;
            last *= &kf;
        }
        last /= &fact;
        let mid = epsilon.clone() / Float::from(2);
        for i in 1..=k {
            inner_vars[var] += &epsilon;
            let mut cur = self.compute_buffer_with(inner_vars, stack);
            let kf = Float::from(k - i);
            for _ in 1..n {
                cur *= &epsilon;
                cur *= &kf;
            }
            cur /= &fact;
            total += (last + &cur) * &mid;
            last = cur;
        }
        total
    }
    #[allow(clippy::too_many_arguments)]
    pub fn numerical_differential(
        self,
        inner_vars: &mut Vec<Number>,
        stack: &mut Tokens,
        x_0: Number,
        t_0: Number,
        t_1: Number,
        x_var: usize,
        t_var: usize,
    ) -> Number {
        let n = 1024;
        let epsilon = (t_1 - &t_0) / Float::from(n);
        inner_vars[x_var] = x_0;
        inner_vars[t_var] = t_0;
        for _ in 1..=n {
            let delta = self.compute_buffer_with(inner_vars, stack) * &epsilon;
            inner_vars[x_var] += delta;
            inner_vars[t_var] += &epsilon;
        }
        inner_vars[x_var].clone()
    }
}
