use crate::parse::TokensRef;
use ucalc_numbers::Complex;
impl TokensRef<'_> {
    pub fn get_inverse(&self, fun_vars: &[Complex], vars: &[Complex]) -> Complex {
        Complex::from(0)
    }
}
