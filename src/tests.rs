use crate::parse::Parsed;
#[test]
fn parse_neg() {
    let infix = Parsed::infix("-4").unwrap().compute();
    let rpn = Parsed::rpn("0 4-").unwrap().compute();
    assert_eq!(infix, rpn);
    assert_eq!(rpn, -4.0);
}
