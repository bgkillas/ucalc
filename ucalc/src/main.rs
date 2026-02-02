use std::env::args;
use ucalc_lib::{Functions, Tokens, Variables};
fn main() {
    let mut infix = true;
    for arg in args().skip(1) {
        if arg == "--rpn" {
            infix = false;
            continue;
        }
        let vars = Variables::default();
        let funs = Functions::default();
        match tmr(|| {
            if infix {
                Tokens::infix(arg.as_str(), &vars, &funs)
            } else {
                Tokens::rpn(arg.as_str(), &vars, &funs)
            }
        }) {
            Ok(parsed) => {
                println!("{}", parsed);
                let compute = tmr(|| parsed.compute(&vars, &funs));
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
