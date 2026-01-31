use crate::parse::{Function, Operators, ParseError, Parsed};
use crate::parse::{Token, Tokens};
use crate::variable::{Functions, Variables};
use crate::{FunctionVar, InnerVariable, InnerVariables, Variable};
use ucalc_numbers::{Complex, Constant};
macro_rules! assert_teq {
    ($a:expr, $b:expr, $c:expr) => {
        assert_eq!($a, $b);
        assert_eq!($a, $c);
    };
}
macro_rules! assert_correct {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        assert_teq!($a.parsed, $b.parsed, Tokens($c));
        assert_teq!($a.clone_compute(), $b.compute(), $d);
    };
}
fn infix(s: &str) -> Parsed {
    Parsed::infix(s, Variables::default(), Functions::default()).unwrap()
}
fn rpn(s: &str) -> Parsed {
    Parsed::rpn(s, Variables::default(), Functions::default()).unwrap()
}
fn res<T>(f: T) -> Complex
where
    Complex: From<T>,
{
    Complex::from(f)
}
fn num<T>(f: T) -> Token
where
    Complex: From<T>,
{
    res(f).into()
}
#[test]
fn parse_neg() {
    assert_correct!(
        infix("-4"),
        rpn("4 _"),
        vec![num(4), Operators::Negate.into()],
        res(-4)
    );
}
#[test]
fn parse_mul() {
    assert_correct!(
        infix("2*4"),
        rpn("2 4 *"),
        vec![num(2), num(4), Operators::Mul.into()],
        res(8)
    );
}
#[test]
fn parse_add() {
    assert_correct!(
        infix("2+4"),
        rpn("2 4 +"),
        vec![num(2), num(4), Operators::Add.into()],
        res(6)
    );
}
#[test]
fn parse_sub() {
    assert_correct!(
        infix("2-4"),
        rpn("2 4 -"),
        vec![num(2), num(4), Operators::Sub.into()],
        res(-2)
    );
}
#[test]
fn parse_div() {
    assert_correct!(
        infix("2/4"),
        rpn("2 4 /"),
        vec![num(2), num(4), Operators::Div.into()],
        res(0.5)
    );
}
#[test]
fn parse_rem() {
    assert_correct!(
        infix("7%4"),
        rpn("7 4 %"),
        vec![num(7), num(4), Operators::Rem.into()],
        res(3)
    );
    assert_correct!(
        infix("7%4^2"),
        rpn("7 4 % 2 ^"),
        vec![
            num(7),
            num(4),
            Operators::Rem.into(),
            num(2),
            Operators::Pow.into()
        ],
        res(9)
    );
}
#[test]
fn parse_pow() {
    assert_correct!(
        infix("2^4"),
        rpn("2 4 ^"),
        vec![num(2), num(4), Operators::Pow.into()],
        res(16)
    );
    assert_correct!(
        infix("2**4"),
        rpn("2 4 **"),
        vec![num(2), num(4), Operators::Pow.into()],
        res(16)
    );
}
#[test]
fn parse_root() {
    assert_correct!(
        infix("4//2"),
        rpn("4 2 //"),
        vec![num(4), num(2), Operators::Root.into()],
        res(2)
    );
}
#[test]
fn parse_min() {
    assert_correct!(
        infix("min(1,2)"),
        rpn("1 2 min"),
        vec![num(1), num(2), Function::Min.into()],
        res(1)
    );
}
#[test]
fn parse_ln() {
    assert_correct!(
        infix("ln(e)"),
        rpn("e ln"),
        vec![num(Constant::E), Function::Ln.into()],
        res(1)
    );
}
#[test]
fn parse_exp() {
    assert_correct!(
        infix("exp(1)"),
        rpn("1 exp"),
        vec![num(1), Function::Exp.into()],
        Complex::from(Constant::E)
    );
}
#[test]
fn parse_max() {
    assert_correct!(
        infix("max(1,2)"),
        rpn("1 2 max"),
        vec![num(1), num(2), Function::Max.into()],
        res(2)
    );
}
#[test]
fn parse_vars() {
    assert_correct!(
        infix("pi"),
        rpn("pi"),
        vec![num(Constant::Pi)],
        res(Constant::Pi)
    );
    assert_correct!(
        infix("e"),
        rpn("e"),
        vec![num(Constant::E)],
        res(Constant::E)
    );
    assert_correct!(
        infix("tau"),
        rpn("tau"),
        vec![num(Constant::Tau)],
        res(Constant::Tau)
    );
    assert_correct!(
        infix("inf"),
        rpn("inf"),
        vec![num(Constant::Infinity)],
        res(Constant::Infinity)
    );
    assert_correct!(infix("i"), rpn("i"), vec![num((0, 1))], res((0, 1)));
}
#[test]
fn parse_cos() {
    assert_correct!(
        infix("cos(pi/6)"),
        rpn("pi 6 / cos"),
        vec![
            num(Constant::Pi),
            num(6),
            Operators::Div.into(),
            Function::Cos.into()
        ],
        (Complex::from(Constant::Pi) / 6).cos()
    );
}
#[test]
fn parse_acos() {
    assert_correct!(
        infix("acos(3//2/2)"),
        rpn("3 2 // 2 / acos"),
        vec![
            num(3),
            num(2),
            Operators::Root.into(),
            num(2),
            Operators::Div.into(),
            Function::Acos.into()
        ],
        (res(3).sqrt() / 2).acos()
    );
}
#[test]
fn parse_sin() {
    assert_correct!(
        infix("sin(pi/6)"),
        rpn("pi 6 / sin"),
        vec![
            num(Constant::Pi),
            num(6),
            Operators::Div.into(),
            Function::Sin.into()
        ],
        (Complex::from(Constant::Pi) / 6).sin()
    );
}
#[test]
fn parse_asin() {
    assert_correct!(
        infix("asin(1/2)"),
        rpn("1 2 / asin"),
        vec![num(1), num(2), Operators::Div.into(), Function::Asin.into()],
        (res(1) / 2).asin()
    );
}
#[test]
fn parse_atan() {
    assert_correct!(
        infix("atan(1,1)"),
        rpn("1 1 atan"),
        vec![num(1), num(1), Function::Atan.into()],
        res(Constant::Pi) / 4
    );
}
#[test]
fn parse_quadratic() {
    assert_correct!(
        infix("quadratic(1,-2,-1)"),
        rpn("1 2 _ 1 _ quadratic"),
        vec![
            num(1),
            num(2),
            Operators::Negate.into(),
            num(1),
            Operators::Negate.into(),
            Function::Quadratic.into()
        ],
        res(2).sqrt() + 1
    );
    assert_correct!(
        infix("quadratic((4-2)/2,3-2-3,-ln(e))"),
        rpn("4 2 - 2 / 3 2 - 3 - e ln _ quadratic"),
        vec![
            num(4),
            num(2),
            Operators::Sub.into(),
            num(2),
            Operators::Div.into(),
            num(3),
            num(2),
            Operators::Sub.into(),
            num(3),
            Operators::Sub.into(),
            num(Constant::E),
            Function::Ln.into(),
            Operators::Negate.into(),
            Function::Quadratic.into()
        ],
        res(2).sqrt() + 1
    );
}
#[test]
fn parse_number() {
    assert_correct!(infix("0.5"), rpn("0.5"), vec![num(0.5)], res(0.5));
}
#[test]
fn parse_order_of_operations() {
    assert_correct!(
        infix("-2*3+4*7+-2^2^3"),
        rpn("2 _ 3 * 4 7 * + 2 2 3 ^ ^ _ +"),
        vec![
            num(2),
            Operators::Negate.into(),
            num(3),
            Operators::Mul.into(),
            num(4),
            num(7),
            Operators::Mul.into(),
            Operators::Add.into(),
            num(2),
            num(2),
            num(3),
            Operators::Pow.into(),
            Operators::Pow.into(),
            Operators::Negate.into(),
            Operators::Add.into(),
        ],
        res(-234)
    );
    assert_correct!(
        infix("sin(max(2,3)/3*pi)"),
        rpn("2 3 max 3 / pi * sin"),
        vec![
            num(2),
            num(3),
            Function::Max.into(),
            num(3),
            Operators::Div.into(),
            num(Constant::Pi),
            Operators::Mul.into(),
            Function::Sin.into(),
        ],
        Complex::from(Constant::Pi).sin()
    );
}
#[test]
fn test_graph_vars() {
    let vars = Variables(vec![Variable::new("x", res(0), false)]);
    let mut infix = Parsed::infix("x", vars.clone(), Functions::default()).unwrap();
    let mut rpn = Parsed::rpn("x", vars, Functions::default()).unwrap();
    assert_correct!(infix.clone(), rpn.clone(), vec![Token::Var(0)], res(0));
    rpn.vars[0].value = res(1);
    infix.vars[0].value = res(1);
    assert_correct!(infix, rpn, vec![Token::Var(0)], res(1));
}
#[test]
fn test_custom_functions() {
    let funs = Functions(vec![FunctionVar::new(
        "f",
        InnerVariables(vec![InnerVariable::new(res(0)), InnerVariable::new(res(0))]),
        Tokens(vec![
            Token::InnerVar(0),
            Token::InnerVar(1),
            Operators::Sub.into(),
        ]),
    )]);
    assert_correct!(
        Parsed::infix("f(3,4)", Variables::default(), funs.clone()).unwrap(),
        Parsed::rpn("3 4 f", Variables::default(), funs.clone()).unwrap(),
        vec![num(3), num(4), Token::Fun(0)],
        res(-1)
    );
}
#[test]
fn test_err() {
    assert_eq!(
        Parsed::infix("(2+3))", Variables::default(), Functions::default()),
        Err(ParseError::LeftParenthesisNotFound)
    );
    assert_eq!(
        Parsed::infix("((2+3)", Variables::default(), Functions::default()),
        Err(ParseError::RightParenthesisNotFound)
    );
    assert_teq!(
        Parsed::infix("2.3.4", Variables::default(), Functions::default()),
        Parsed::rpn("2.3.4", Variables::default(), Functions::default()),
        Err(ParseError::UnknownToken("2.3.4".to_string()))
    );
    assert_teq!(
        Parsed::infix("abc(2)", Variables::default(), Functions::default()),
        Parsed::rpn("2 abc", Variables::default(), Functions::default()),
        Err(ParseError::UnknownToken("abc".to_string()))
    );
}
