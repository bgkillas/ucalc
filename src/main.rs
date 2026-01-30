use std::env::args;
use std::hint::black_box;
use ucalc::parse::Parsed;
fn main() {
    tmr(|| {
        for _ in 0..1 << 20 {
            black_box(Parsed::infix("-2*3+4*7+-2^2^3").unwrap());
        }
    });
    let p = Parsed::infix("-2*3+4*7+-2*2*3").unwrap();
    tmr(|| {
        for _ in 0..1 << 20 {
            black_box(p.clone().compute());
        }
    });
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
