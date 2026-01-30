use std::env::args;
use ucalc::parse::Parsed;
fn main() {
    for arg in args().skip(1) {
        let mut parsed = tmr(|| Parsed::infix(arg.as_str()).unwrap());
        let compute = tmr(|| parsed.compute());
        println!("{}", compute);
    }
}
fn tmr<T, W>(fun: T) -> W
where
    T: FnOnce() -> W,
{
    let tmr = std::time::Instant::now();
    let ret = fun();
    println!("{}", tmr.elapsed().as_nanos());
    ret
}
