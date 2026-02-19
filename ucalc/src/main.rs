use concurrent_stdin::Out;
use std::env::args;
use std::io::{BufRead, IsTerminal, stdin, stdout};
use ucalc_lib::{Functions, Number, Tokens, Variable, Variables};
fn main() {
    let mut vars = Variables::default();
    let mut funs = Functions::default();
    let mut infix = true;
    let mut quit = false;
    for arg in args().skip(1) {
        quit = true;
        run_line(arg.as_str(), &mut infix, &mut vars, &mut funs)
    }
    let stdin = stdin().lock();
    if !stdin.is_terminal() {
        stdin
            .lines()
            .for_each(|l| run_line(l.unwrap().as_str(), &mut infix, &mut vars, &mut funs));
    } else if !quit {
        let mut out = Out::default();
        let mut stdout = stdout().lock();
        vars.push(Variable::new("@", Number::default()));
        loop {
            let mut n = None;
            out.read(
                &mut stdout,
                |line| match if infix {
                    Tokens::infix(line, &mut vars, &mut funs, &[], false)
                } else {
                    Tokens::rpn(line, &mut vars, &mut funs, &[], false)
                } {
                    Ok(Some(tokens)) => {
                        let compute = tokens.compute(&[], &funs, &vars);
                        print!("{}", compute);
                        Some(Some(compute))
                    }
                    Ok(None) => Some(None),
                    Err(e) => {
                        print!("{e:?}");
                        None
                    }
                },
                |num| n = Some(num),
            );
            if let Some(num) = n {
                vars.get_mut("@").value = num;
            }
        }
    }
}
fn run_line(line: &str, infix: &mut bool, vars: &mut Variables, funs: &mut Functions) {
    if line == "--rpn" {
        *infix = false;
        return;
    }
    match tmr(|| {
        if *infix {
            Tokens::infix(line, vars, funs, &[], false)
        } else {
            Tokens::rpn(line, vars, funs, &[], false)
        }
    }) {
        Ok(Some(tokens)) => {
            println!("{tokens:?}");
            println!("{}", tokens.get_infix(vars, funs, &[]));
            println!("{}", tokens.get_rpn(vars, funs, &[]));
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
