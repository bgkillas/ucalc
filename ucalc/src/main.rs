use std::env::args;
use std::io::{IsTerminal, stdin};
use ucalc_lib::{Functions, Tokens, Variables};
fn main() {
    let vars = Variables::default();
    let funs = Functions::default();
    let mut infix = true;
    for arg in args().skip(1) {
        run_line(arg.as_str(), &mut infix, &vars, &funs)
    }
    if !stdin().is_terminal() {
        stdin()
            .lines()
            .for_each(|l| run_line(l.unwrap().as_str(), &mut infix, &vars, &funs));
    }
}
fn run_line(line: &str, infix: &mut bool, vars: &Variables, funs: &Functions) {
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
        Ok(tokens) => {
            let compute = tmr(|| tokens.compute(&[], funs));
            println!("{}", compute);
        }
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
