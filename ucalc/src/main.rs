use std::env::args;
use std::io::Write;
use std::io::{BufRead, IsTerminal, stdin, stdout};
use termion::cursor::Left;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use ucalc_lib::{Functions, Tokens, Variables};
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
        let mut stdout = stdout().into_raw_mode().unwrap();
        let mut line = String::new();
        for k in stdin.keys() {
            let Ok(k) = k else {
                return;
            };
            match k {
                Key::Char('\n') => {
                    write!(stdout, "\n{}", Left(u16::MAX)).unwrap();
                    if line != "exit" {
                        run_line(&line, &mut infix, &mut vars, &mut funs);
                        write!(stdout, "\n{}", Left(u16::MAX)).unwrap();
                    }
                    stdout.flush().unwrap();
                    if line == "exit" {
                        return;
                    }
                    line.clear();
                }
                Key::Char(c) => {
                    line.push(c);
                    write!(stdout, "{}", c).unwrap();
                    stdout.flush().unwrap();
                }
                _ => {}
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
