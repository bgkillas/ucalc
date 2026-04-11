use crate::colors::{Colors, ToColor, color_brackets};
use crate::complete::Complete;
use readchar::crossterm::QueueableCommand;
use readchar::crossterm::cursor::MoveTo;
use readchar::crossterm::terminal::{Clear, ClearType};
use readchar::{History, ReadChar, Return};
use std::env::args;
use std::fmt;
use std::fmt::Write;
use std::hint::black_box;
use std::io::{BufRead, IsTerminal, stdin, stdout};
use ucalc_lib::{Compute, Functions, Number, Tokens, Variable, Variables, Volatility, get_help};
#[cfg(feature = "float_rand")]
use ucalc_lib::{Rand, rng};
use ucalc_numbers::{FloatTrait, RealTrait};
pub fn cli() {
    let colors = Colors::default();
    let mut vars = Variables::default();
    let mut funs = Functions::default();
    let mut quit = false;
    let mut options = Options::default();
    #[cfg(feature = "float_rand")]
    let mut rand = rng();
    for arg in args().skip(1) {
        run_line(
            arg.as_str(),
            &mut options,
            &mut vars,
            &mut funs,
            &mut quit,
            #[cfg(feature = "float_rand")]
            &mut rand,
        )
    }
    let stdin = stdin().lock();
    if !stdin.is_terminal() {
        stdin.lines().for_each(|l| {
            run_line(
                l.unwrap().as_str(),
                &mut options,
                &mut vars,
                &mut funs,
                &mut false,
                #[cfg(feature = "float_rand")]
                &mut rand,
            )
        });
    } else if !quit {
        let mut readchar = ReadChar::new(History::new(None).unwrap());
        let mut stdout = stdout().lock();
        vars.push(Variable::new("@", Number::default(), Volatility::Constant));
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
                        options,
                        string,
                        &colors,
                        #[cfg(feature = "float_rand")]
                        &mut rand,
                    )
                    .unwrap()
                },
                ToColor(&colors),
                |readchar, stdout, line| {
                    Ok(match line {
                        "exit" => {
                            readchar.close(stdout)?;
                            Return::Cancel
                        }
                        "clear" => {
                            write!(Clear(ClearType::Purge))?;
                            write!(Clear(ClearType::All))?;
                            write!(MoveTo(0, 0))?;
                            Return::Finish
                        }
                        _ => Return::Finish,
                    })
                },
                to_alt,
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
fn run_line(
    line: &str,
    options: &mut Options,
    vars: &mut Variables,
    funs: &mut Functions,
    quit: &mut bool,
    #[cfg(feature = "float_rand")] rand: &mut Rand,
) {
    if line == "--rpn" {
        options.rpn = true;
        return;
    }
    if line == "--perf" {
        options.perf = true;
        return;
    }
    if line == "--benchmark_simplify" {
        options.benchmark_simplify = true;
        return;
    }
    let mut get = |s: &str| -> isize {
        let tokens = Tokens::parse(
            s,
            vars,
            funs,
            &[],
            false,
            true,
            options.base_input,
            options.rpn,
            #[cfg(feature = "float_rand")]
            rand,
        )
        .unwrap()
        .unwrap();
        let num = tokens.compute(
            &[],
            funs,
            vars,
            #[cfg(feature = "float_rand")]
            rand,
        );
        num.to_real().into_isize()
    };
    if let Some(s) = line.strip_prefix("--base_input=") {
        options.base_input = get(s).try_into().unwrap();
        return;
    }
    if let Some(s) = line.strip_prefix("--base_output=") {
        options.base_output = get(s).try_into().unwrap();
        return;
    }
    if let Some(s) = line.strip_prefix("--benchmark=") {
        options.benchmark = get(s).try_into().unwrap();
        return;
    }
    *quit = true;
    match tmr(
        || {
            Tokens::parse(
                line,
                vars,
                funs,
                &[],
                false,
                true,
                options.base_input,
                options.rpn,
                #[cfg(feature = "float_rand")]
                rand,
            )
        },
        options.perf,
    ) {
        Ok(Some(tokens)) => {
            let compute = tmr(
                || {
                    tokens.compute(
                        &[],
                        funs,
                        vars,
                        #[cfg(feature = "float_rand")]
                        rand,
                    )
                },
                options.perf,
            );
            print!("{}", compute.get_closest_fraction(options.base_output));
            println!("{}", compute.to_string_radix(options.base_output));
            if options.benchmark > 0 {
                if options.benchmark_simplify {
                    benchmark(
                        tokens,
                        options.benchmark,
                        vars,
                        funs,
                        #[cfg(feature = "float_rand")]
                        rand,
                    );
                }
                let tokens = Tokens::parse(
                    line,
                    vars,
                    funs,
                    &[],
                    false,
                    false,
                    options.base_input,
                    options.rpn,
                    #[cfg(feature = "float_rand")]
                    rand,
                )
                .unwrap()
                .unwrap();
                benchmark(
                    tokens,
                    options.benchmark,
                    vars,
                    funs,
                    #[cfg(feature = "float_rand")]
                    rand,
                );
            }
        }
        Ok(None) => {}
        Err(e) => println!("{e:?}"),
    }
}
fn benchmark(
    tokens: Tokens,
    n: usize,
    vars: &mut Variables,
    funs: &mut Functions,
    #[cfg(feature = "float_rand")] rand: &mut Rand,
) {
    let cap = tokens.len() + funs.iter().map(|c| c.tokens.len()).sum::<usize>();
    let mut inner_vars = Vec::with_capacity(cap);
    let mut stack = Vec::with_capacity(cap);
    let compute = Compute::new(&tokens[..], &[], funs, vars, 0);
    for _ in 0..n {
        black_box(compute.compute(
            &mut inner_vars,
            &mut stack,
            #[cfg(feature = "float_rand")]
            rand,
        ));
    }
    let tmr = std::time::Instant::now();
    for _ in 0..n {
        black_box(compute.compute(
            &mut inner_vars,
            &mut stack,
            #[cfg(feature = "float_rand")]
            rand,
        ));
    }
    let time = tmr.elapsed().as_nanos();
    let mean = time as f64 / n as f64;
    println!("Total: {time}ns, Mean: {mean}ns")
}
fn tmr<T, W>(fun: T, perf: bool) -> W
where
    T: FnOnce() -> W,
{
    if perf {
        let tmr = std::time::Instant::now();
        let ret = fun();
        println!("{}", tmr.elapsed().as_nanos());
        ret
    } else {
        fun()
    }
}
