use std::env::args;
use ucalc::parse::Parsed;
fn main() {
    let arg = args().nth(1).unwrap();
    let parsed = Parsed::try_from(arg.as_str()).unwrap();
    println!("{parsed:?}");
    println!("{}", parsed.compute());
}
