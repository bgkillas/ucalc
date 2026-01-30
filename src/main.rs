use std::env::args;
use ucalc::parse::Parsed;
fn main() {
    let arg = args().nth(1).unwrap();
    let parsed = tmr(|| Parsed::infix(arg.as_str()).unwrap());
    println!("{parsed:?}");
    let compute = tmr(|| parsed.compute());
    println!("{}", compute);
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
