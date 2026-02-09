use std::env::args;
use std::io::{IsTerminal, stdin};
use ucalc_lib::{Functions, Tokens, Variables};
fn main() {
    let mut vars = Variables::default();
    let mut funs = Functions::default();
    let mut infix = true;
    for arg in args().skip(1) {
        run_line(arg.as_str(), &mut infix, &mut vars, &mut funs)
    }
    if !stdin().is_terminal() {
        stdin()
            .lines()
            .for_each(|l| run_line(l.unwrap().as_str(), &mut infix, &mut vars, &mut funs));
    }
}
fn run_line(line: &str, infix: &mut bool, vars: &mut Variables, funs: &mut Functions) {
    if line == "--rpn" {
        *infix = false;
        return;
    }
    match tmr(|| {
        if *infix {
            Tokens::infix(line, vars, &[], funs)
        } else {
            Tokens::rpn(line, vars, &[], funs)
        }
    }) {
        Ok(Some(tokens)) => {
            let compute = tmr(|| tokens.compute(&[], funs, vars));
            println!("{}", compute);
        }
        Ok(None) => {}
        Err(e) => println!("{e:?}"),
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
