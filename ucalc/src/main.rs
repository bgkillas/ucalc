mod colors;
mod complete;
#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
use crate::colors::{Colors, ToColor, color_brackets};
use crate::complete::Complete;
use readchar::crossterm::QueueableCommand;
use readchar::crossterm::cursor::MoveTo;
use readchar::crossterm::terminal::{Clear, ClearType};
use readchar::{ReadChar, Return};
use std::env::args;
use std::fmt::Write;
use std::io::{BufRead, IsTerminal, stdin, stdout};
use ucalc_lib::{Functions, Number, Tokens, Variable, Variables, get_help};
use ucalc_numbers::FloatTrait;
fn main() {
    let colors = Colors::default();
    let mut vars = Variables::default();
    let mut funs = Functions::default();
    let mut infix = true;
    let mut quit = false;
    let mut base_input = 10;
    let mut base_output = 10;
    for arg in args().skip(1) {
        quit = true;
        run_line(
            arg.as_str(),
            &mut infix,
            &mut base_input,
            &mut base_output,
            &mut vars,
            &mut funs,
        )
    }
    let stdin = stdin().lock();
    if !stdin.is_terminal() {
        stdin.lines().for_each(|l| {
            run_line(
                l.unwrap().as_str(),
                &mut infix,
                &mut base_input,
                &mut base_output,
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
                |line, string| {
                    last = process_line(
                        line,
                        &mut vars,
                        &mut funs,
                        infix,
                        base_input,
                        base_output,
                        string,
                        &colors,
                    )
                },
                ToColor(&colors),
                |readchar, stdout, line| {
                    Ok(match line {
                        "exit" => {
                            readchar.close(stdout)?;
                            Return::Cancel
                        }
                        "clear" => {
                            stdout.queue(Clear(ClearType::Purge))?;
                            stdout.queue(Clear(ClearType::All))?;
                            stdout.queue(MoveTo(0, 0))?;
                            Return::Finish
                        }
                        _ => Return::Finish,
                    })
                },
                Complete(&colors),
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
#[allow(clippy::too_many_arguments)]
fn process_line(
    line: &str,
    vars: &mut Variables,
    funs: &mut Functions,
    infix: bool,
    base_input: u8,
    base_output: u8,
    str: &mut String,
    colors: &Colors,
) -> Option<Number> {
    str.clear();
    match line {
        "" | "exit" | "clear" => None,
        _ if line.starts_with("help") => {
            let arg = line.split_once(' ').map(|(_, a)| a).unwrap_or("");
            write!(str, "{}", color_brackets(get_help(arg), colors)).unwrap();
            None
        }
        _ => {
            match if infix {
                Tokens::infix(line, vars, funs, &[], false, base_input)
            } else {
                Tokens::rpn(line, vars, funs, &[], false, base_input)
            } {
                Ok(Some(tokens)) => {
                    let compute = tokens.compute(&[], funs, vars);
                    write!(str, "{}", compute.to_string_radix(base_output)).unwrap();
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
    base_input: &mut u8,
    base_output: &mut u8,
    vars: &mut Variables,
    funs: &mut Functions,
) {
    if line == "--rpn" {
        *infix = false;
        return;
    }
    if let Some(s) = line.strip_prefix("--base_input=") {
        *base_input = s.parse().unwrap();
        return;
    }
    if let Some(s) = line.strip_prefix("--base_output=") {
        *base_output = s.parse().unwrap();
        return;
    }
    match if *infix {
        Tokens::infix(line, vars, funs, &[], false, *base_input)
    } else {
        Tokens::rpn(line, vars, funs, &[], false, *base_input)
    } {
        Ok(Some(tokens)) => {
            //println!("{tokens:?}");
            //println!("{}", tokens.get_infix(vars, funs, &[]));
            //println!("{}", tokens.get_rpn(vars, funs, &[]));
            let compute = tmr(|| tokens.compute(&[], funs, vars));
            //let compute = tokens.compute(&[], funs, vars);
            //println!("{}", compute.get_closest_fraction());
            println!("{}", compute.to_string_radix(*base_output));
        }
        Ok(None) => {}
        Err(e) => println!("{e:?}"),
    }
}
#[allow(dead_code)]
fn tmr<T, W>(fun: T) -> W
where
    T: FnOnce() -> W,
{
    let tmr = std::time::Instant::now();
    let ret = fun();
    println!("{}", tmr.elapsed().as_nanos());
    ret
}
