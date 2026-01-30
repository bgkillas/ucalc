use std::env::args;
use ucalc::parse::{Parsed, Token};
fn main() {
    fn tmr<T, W>(fun: T) -> W
    where
        T: FnOnce() -> W,
    {
        let tmr = std::time::Instant::now();
        let ret = fun();
        println!("{}", tmr.elapsed().as_nanos());
        ret
    }
    tmr(|| {
        for _ in 0..1 << 20 {
            std::hint::black_box(Parsed::infix("-2*3+4*7+-2^2^3").unwrap());
        }
    });
    let mut p = Parsed::infix("-2*3+4*7+-2*2*3").unwrap();
    let mut b: Vec<f64> = Vec::with_capacity(ucalc::parse::Operators::MAX_INPUT - 1);
    p.parsed.shrink_to_fit();
    tmr(|| {
        let mut v = Parsed {
            parsed: vec![Token::Num(0.0); p.parsed.len()],
        };
        for _ in 0..1 << 20 {
            v.parsed.extend_from_slice(&p.parsed);
            std::hint::black_box(v.compute_buffer(&mut b));
            b.clear();
            v.parsed.clear();
        }
    });
    tmr(|| {
        for _ in 0..1 << 20 {
            std::hint::black_box(p.clone().compute_buffer(&mut b));
            b.clear();
        }
    });
    tmr(|| {
        for _ in 0..1 << 20 {
            std::hint::black_box(p.clone().compute());
        }
    });
    for arg in args().skip(1) {
        let mut parsed = tmr(|| Parsed::infix(arg.as_str()).unwrap());
        let compute = tmr(|| parsed.compute());
        println!("{}", compute);
    }
}
