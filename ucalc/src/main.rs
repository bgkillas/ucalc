use std::env::args;
use ucalc_lib::{Functions, Parsed, Variables};
fn main() {
    for arg in args().skip(1) {
        let vars = Variables::default();
        let funs = Functions::default();
        match tmr(|| Parsed::infix(arg.as_str(), vars, funs)) {
            Ok(mut parsed) => {
                let compute = tmr(|| parsed.compute());
                println!("{}", compute);
            }
            Err(e) => println!("{e:?}"),
        }
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
