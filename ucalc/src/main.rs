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
use std::fmt;
use std::fmt::Write;
use std::hint::black_box;
use std::io::{BufRead, IsTerminal, stdin, stdout};
use ucalc_lib::{Functions, Number, Tokens, Variable, Variables, Volatility, get_help};
#[cfg(feature = "float_rand")]
use ucalc_lib::{Rand, rng};
use ucalc_numbers::{FloatTrait, RealTrait};
#[derive(Clone, Copy)]
pub struct Options {
    rpn: bool,
    perf: bool,
    base_input: u8,
    base_output: u8,
    benchmark: usize,
}
impl Default for Options {
    fn default() -> Self {
        Self {
            rpn: false,
            perf: false,
            base_input: 10,
            base_output: 10,
            benchmark: 0,
        }
    }
}
fn main() {
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
        let mut readchar = ReadChar::default();
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
                            stdout.queue(Clear(ClearType::Purge))?;
                            stdout.queue(Clear(ClearType::All))?;
                            stdout.queue(MoveTo(0, 0))?;
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
fn process_line(
    line: &str,
    vars: &mut Variables,
    funs: &mut Functions,
    options: Options,
    str: &mut String,
    colors: &Colors,
    #[cfg(feature = "float_rand")] rand: &mut Rand,
) -> Result<Option<Number>, fmt::Error> {
    str.clear();
    Ok(match line {
        "" | "exit" | "clear" => None,
        _ if line.starts_with("help") => {
            let arg = line.split_once(' ').map(|(_, a)| a).unwrap_or("");
            write!(str, "{}", color_brackets(get_help(arg), colors))?;
            None
        }
        _ => {
            match tmr_write(
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
                str,
                options.perf,
            ) {
                Ok(Some(tokens)) => {
                    let compute = tmr_write(
                        || {
                            tokens.compute(
                                &[],
                                funs,
                                vars,
                                #[cfg(feature = "float_rand")]
                                rand,
                            )
                        },
                        str,
                        options.perf,
                    );
                    write!(str, "{}", compute.get_closest_fraction(options.base_output))?;
                    write!(str, "{}", compute.to_string_radix(options.base_output))?;
                    Some(compute)
                }
                Ok(None) => None,
                Err(e) => {
                    write!(str, "{e:?}")?;
                    None
                }
            }
        }
    })
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
                benchmark(
                    tokens,
                    options.benchmark,
                    vars,
                    funs,
                    #[cfg(feature = "float_rand")]
                    rand,
                );
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
    for _ in 0..n {
        black_box(tokens.compute_buffer(
            &mut inner_vars,
            &[],
            funs,
            vars,
            &mut stack,
            #[cfg(feature = "float_rand")]
            rand,
        ));
    }
    let tmr = std::time::Instant::now();
    for _ in 0..n {
        black_box(tokens.compute_buffer(
            &mut inner_vars,
            &[],
            funs,
            vars,
            &mut stack,
            #[cfg(feature = "float_rand")]
            rand,
        ));
    }
    let time = tmr.elapsed().as_nanos();
    let mean = time as f64 / n as f64;
    println!("Total: {time}ns, Mean: {mean}ns")
}
fn tmr_write<T, W>(fun: T, str: &mut impl Write, perf: bool) -> W
where
    T: FnOnce() -> W,
{
    if perf {
        let tmr = std::time::Instant::now();
        let ret = fun();
        writeln!(str, "{}", tmr.elapsed().as_nanos()).unwrap();
        ret
    } else {
        fun()
    }
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
fn to_alt(c: char) -> Option<char> {
    Some(match c {
        'a' => 'α',
        'A' => 'Α',
        'b' => 'β',
        'B' => 'Β',
        'c' => 'ξ',
        'C' => 'Ξ',
        'd' => 'Δ',
        'D' => 'δ',
        'e' => 'ε',
        'E' => 'Ε',
        'f' => 'φ',
        'F' => 'Φ',
        'g' => 'γ',
        'G' => 'Γ',
        'h' => 'η',
        'H' => 'Η',
        'i' => 'ι',
        'I' => 'Ι',
        'k' => 'κ',
        'K' => 'Κ',
        'l' => 'λ',
        'L' => 'Λ',
        'm' => 'μ',
        'M' => 'Μ',
        'n' => 'ν',
        'N' => 'Ν',
        'o' => 'ο',
        'O' => 'Ο',
        'p' => 'π',
        'P' => 'Π',
        'q' => 'θ',
        'Q' => 'Θ',
        'r' => 'ρ',
        'R' => 'Ρ',
        's' => 'σ',
        'S' => 'Σ',
        't' => 'τ',
        'T' => 'Τ',
        'u' => 'υ',
        'U' => 'Υ',
        'w' => 'ω',
        'W' => 'Ω',
        'y' => 'ψ',
        'Y' => 'Ψ',
        'x' => 'χ',
        'X' => 'Χ',
        'z' => 'ζ',
        'Z' => 'Ζ',
        '9' => '⁹',
        '8' => '⁸',
        '7' => '⁷',
        '6' => '⁶',
        '5' => '⁵',
        '4' => '⁴',
        '3' => '³',
        '2' => '²',
        '1' => '¹',
        '0' => '⁰',
        ')' => '₀',
        '!' => '₁',
        '@' => '₂',
        '#' => '₃',
        '$' => '₄',
        '%' => '₅',
        '^' => '₆',
        '&' => '₇',
        '*' => '₈',
        '(' => '₉',
        ';' => '°',
        '-' => '⁻',
        '+' => '⁺',
        '`' => 'ⁱ',
        '=' => '±',
        '_' => '∞',
        _ => return None,
    })
}
