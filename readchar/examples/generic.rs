use readchar::ReadChar;
use std::env::args;
use std::io::stdout;
use std::process::Command;
fn main() {
    let mut args = args();
    let cmd = args.nth(1).unwrap();
    let args: Vec<String> = args.collect();
    let mut readchar: ReadChar = ReadChar::default();
    let mut stdout = stdout().lock();
    readchar.init(&mut stdout).unwrap();
    let mut string = String::with_capacity(64);
    loop {
        readchar
            .read(&mut stdout, &mut string, |line, string| {
                string.clear();
                if !line.is_empty() {
                    let out = Command::new(&cmd).args(&args).arg(line).output().unwrap();
                    string.push_str(str::from_utf8(&out.stdout).unwrap().trim_end_matches("\n"));
                }
            })
            .unwrap();
    }
}
