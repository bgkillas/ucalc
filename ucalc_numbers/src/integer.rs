pub type U = usize;
pub type I = isize;
#[derive(Clone, Default, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Integer(pub I);
#[derive(Clone, Default, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct UInteger(pub U);
impl Integer {
    pub fn binomial(self, k: Self) -> Self {
        if k.0 < 0 || self.0 < 0 {
            return Self(0);
        }
        let n = self.0 as U;
        let k = k.0 as U;
        Self(UInteger(n).binomial(UInteger(k)).0 as I)
    }
    pub fn factorial(self) -> Self {
        Self((1..=self.0).product())
    }
}
impl UInteger {
    pub const fn binomial(self, k: Self) -> Self {
        if k.0 > self.0 {
            return Self(0);
        }
        let n = self.0;
        let mut k = k.0;
        if k > n - k {
            k = n - k;
        }
        let r = n - k;
        const fn gcd(mut a: U, mut b: U) -> U {
            while b != 0 {
                (a, b) = (b, a % b);
            }
            a
        }
        let mut result = 1;
        let mut i = 1;
        while i <= k {
            let num = r + i;
            let den = i;
            let g = gcd(num, den);
            result *= g;
            result /= den;
            result *= num;
            result /= g;
            i += 1;
        }
        Self(result as U)
    }
    pub fn factorial(self) -> Self {
        Self((1..=self.0).product())
    }
}
impl const From<U> for UInteger {
    fn from(value: U) -> Self {
        Self(value)
    }
}
impl const From<I> for Integer {
    fn from(value: I) -> Self {
        Self(value)
    }
}
