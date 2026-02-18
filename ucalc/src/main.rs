use crossterm::cursor::{MoveToColumn, MoveToNextLine, MoveToPreviousLine};
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{ExecutableCommand, event, terminal};
use std::env::args;
use std::io::Write;
use std::io::{BufRead, IsTerminal, stdin, stdout};
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
        terminal::enable_raw_mode().unwrap();
        #[cfg(debug_assertions)]
        let hook = std::panic::take_hook();
        #[cfg(debug_assertions)]
        std::panic::set_hook(Box::new(move |info| {
            _ = terminal::disable_raw_mode();
            println!();
            hook(info);
        }));
        let mut stdout = stdout().lock();
        let mut line = String::new();
        let mut cursor = 0;
        loop {
            let Ok(k) = event::read() else {
                return;
            };
            match k {
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    ..
                }) => {
                    cursor = 0;
                    stdout.execute(MoveToNextLine(2)).unwrap();
                    stdout.execute(MoveToColumn(0)).unwrap();
                    stdout.flush().unwrap();
                    if line == "exit" {
                        terminal::disable_raw_mode().unwrap();
                        return;
                    }
                    line.clear();
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char(c),
                    ..
                }) => {
                    line.push(c);
                    cursor += 1;
                    print!("{}", c);
                    match if infix {
                        Tokens::infix(&line, &mut vars, &mut funs, &[], false)
                    } else {
                        Tokens::rpn(&line, &mut vars, &mut funs, &[], false)
                    } {
                        Ok(Some(tokens)) => {
                            let compute = tokens.compute(&[], &funs, &vars);
                            stdout.execute(MoveToNextLine(1)).unwrap();
                            stdout.execute(MoveToColumn(0)).unwrap();
                            stdout.execute(Clear(ClearType::CurrentLine)).unwrap();
                            print!("{}", compute);
                            stdout.execute(MoveToPreviousLine(1)).unwrap();
                            stdout.execute(MoveToColumn(cursor)).unwrap();
                        }
                        Ok(None) => {}
                        Err(e) => print!("{e:?}"),
                    }
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
