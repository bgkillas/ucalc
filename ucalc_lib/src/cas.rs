use crate::parse::TokensRef;
use ucalc_numbers::Complex;
impl TokensRef<'_> {
    #[allow(unused_variables)]
    pub fn get_inverse(&self, fun_vars: &[Complex], vars: &[Complex]) -> Complex {
        //TODO
        Complex::from(0)
    }
}
