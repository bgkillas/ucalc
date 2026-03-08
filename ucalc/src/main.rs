#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
use readchar::crossterm::QueueableCommand;
use readchar::crossterm::cursor::MoveTo;
use readchar::crossterm::terminal::{Clear, ClearType};
use readchar::{ReadChar, Return};
use std::env::args;
use std::fmt::Write;
use std::io::{BufRead, IsTerminal, stdin, stdout};
use ucalc_lib::{FUNCTION_LIST, Functions, Number, Tokens, Variable, Variables};
fn main() {
    let mut vars = Variables::default();
    let mut funs = Functions::default();
    let mut infix = true;
    let mut quit = false;
    let mut base = 10;
    for arg in args().skip(1) {
        quit = true;
        run_line(arg.as_str(), &mut infix, &mut base, &mut vars, &mut funs)
    }
    let stdin = stdin().lock();
    if !stdin.is_terminal() {
        stdin.lines().for_each(|l| {
            run_line(
                l.unwrap().as_str(),
                &mut infix,
                &mut base,
                &mut vars,
                &mut funs,
            )
        });
    } else if !quit {
        let mut readchar = ReadChar::default();
        let mut stdout = stdout().lock();
        vars.push(Variable::new("@", Number::default()));
        readchar.init(&mut stdout).unwrap();
        let mut string = String::with_capacity(64);
        let mut last = None;
        loop {
            match readchar.read(
                &mut stdout,
                &mut string,
                |line, string| last = process_line(line, &mut vars, &mut funs, infix, base, string),
                |readchar, stdout, line| match line {
                    "exit" => {
                        readchar.close(stdout).unwrap();
                        Return::Cancel
                    }
                    "clear" => {
                        stdout.queue(Clear(ClearType::Purge)).unwrap();
                        stdout.queue(Clear(ClearType::All)).unwrap();
                        stdout.queue(MoveTo(0, 0)).unwrap();
                        Return::Finish
                    }
                    _ => Return::Finish,
                },
                Some(complete),
            ) {
                Ok(Return::Finish) => {
                    if let Some(n) = last.take() {
                        vars.get_mut("@").value = n;
                    }
                }
                Ok(Return::Cancel) => return,
                Ok(Return::None) => {}
                Err(e) => {
                    drop(readchar);
                    println!("\n{e:?}");
                    return;
                }
            }
        }
    }
}
fn complete(mut line: &str) -> Vec<String> {
    if line.ends_with(',') {
        let mut bracket = 0;
        for (i, c) in line.char_indices().rev() {
            if c == ')' {
                bracket += 1;
            } else if c == '(' {
                if bracket == 0 {
                    line = &line[..i];
                    break;
                } else {
                    bracket -= 1;
                }
            }
        }
    }
    if line.ends_with(['(', '{', '[', '|']) {
        line = &line[..line.len() - 1];
    }
    let word = if let Some(idx) = line.rfind(|c: char| !c.is_ascii_alphabetic()) {
        if idx + 1 == line.len() {
            return Vec::new();
        }
        &line[idx + 1..]
    } else {
        line
    };
    let mut ret = Vec::new();
    for w in FUNCTION_LIST {
        if w.starts_with(word) {
            ret.push(w.to_string())
        }
    }
    ret
}
fn process_line(
    line: &str,
    vars: &mut Variables,
    funs: &mut Functions,
    infix: bool,
    base: u32,
    str: &mut String,
) -> Option<Number> {
    str.clear();
    match line {
        "" | "exit" | "clear" => None,
        _ => {
            match if infix {
                Tokens::infix(line, vars, funs, &[], false, base)
            } else {
                Tokens::rpn(line, vars, funs, &[], false, base)
            } {
                Ok(Some(tokens)) => {
                    let compute = tokens.compute(&[], funs, vars);
                    write!(str, "{}", compute).unwrap();
                    Some(compute)
                }
                Ok(None) => None,
                Err(e) => {
                    write!(str, "{e:?}").unwrap();
                    None
                }
            }
        }
    }
}
fn run_line(
    line: &str,
    infix: &mut bool,
    base: &mut u32,
    vars: &mut Variables,
    funs: &mut Functions,
) {
    if line == "--rpn" {
        *infix = false;
        return;
    }
    if let Some(s) = line.strip_prefix("--base=") {
        *base = s.parse().unwrap();
    }
    match tmr(|| {
        if *infix {
            Tokens::infix(line, vars, funs, &[], false, *base)
        } else {
            Tokens::rpn(line, vars, funs, &[], false, *base)
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
