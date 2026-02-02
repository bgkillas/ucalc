use std::env::args;
use std::io::stdin;
use ucalc_lib::{Functions, Tokens, Variables};
fn main() {
    let vars = Variables::default();
    let funs = Functions::default();
    let mut infix = true;
    for mut arg in args().skip(1) {
        match arg.as_str() {
            "--rpn" => {
                infix = false;
                continue;
            }
            "-" => {
                arg.clear();
                stdin().read_line(&mut arg).unwrap();
                arg.pop();
            }
            _ => {}
        }
        match tmr(|| {
            if infix {
                Tokens::infix(arg.as_str(), &vars, &[], &funs)
            } else {
                Tokens::rpn(arg.as_str(), &vars, &[], &funs)
            }
        }) {
            Ok(tokens) => {
                println!("{}", tokens);
                let compute = tmr(|| tokens.compute(&[], &funs));
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
