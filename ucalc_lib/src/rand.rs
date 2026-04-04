use ucalc_numbers::rand;
use ucalc_numbers::rand::prelude::ThreadRng;
pub type Rand = ThreadRng;
pub fn rng() -> Rand {
    rand::rng()
}
