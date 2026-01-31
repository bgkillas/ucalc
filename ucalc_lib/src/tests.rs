use crate::parse::{Function, Operators, ParseError, Parsed};
use crate::parse::{Token, Tokens};
use crate::variable::{Functions, Variables};
use crate::{FunctionVar, InnerVariable, InnerVariables, Variable};
use std::f64::consts::{E, PI};

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
#[test]
fn parse_neg() {
    assert_correct!(
        infix("-4"),
        rpn("4 _"),
        vec![4.0f64.into(), Operators::Negate.into()],
        -4.0
    );
}
#[test]
fn parse_mul() {
    assert_correct!(
        infix("2*4"),
        rpn("2 4 *"),
        vec![2.0f64.into(), 4.0f64.into(), Operators::Mul.into()],
        8.0
    );
}
#[test]
fn parse_add() {
    assert_correct!(
        infix("2+4"),
        rpn("2 4 +"),
        vec![2.0f64.into(), 4.0f64.into(), Operators::Add.into()],
        6.0
    );
}
#[test]
fn parse_sub() {
    assert_correct!(
        infix("2-4"),
        rpn("2 4 -"),
        vec![2.0f64.into(), 4.0f64.into(), Operators::Sub.into()],
        -2.0
    );
}
#[test]
fn parse_div() {
    assert_correct!(
        infix("2/4"),
        rpn("2 4 /"),
        vec![2.0f64.into(), 4.0f64.into(), Operators::Div.into()],
        0.5
    );
}
#[test]
fn parse_pow() {
    assert_correct!(
        infix("2^4"),
        rpn("2 4 ^"),
        vec![2.0f64.into(), 4.0f64.into(), Operators::Pow.into()],
        16.0
    );
    assert_correct!(
        infix("2**4"),
        rpn("2 4 **"),
        vec![2.0f64.into(), 4.0f64.into(), Operators::Pow.into()],
        16.0
    );
}
#[test]
fn parse_root() {
    assert_correct!(
        infix("4//2"),
        rpn("4 2 //"),
        vec![4.0f64.into(), 2.0f64.into(), Operators::Root.into()],
        2.0
    );
}
#[test]
fn parse_min() {
    assert_correct!(
        infix("min(1,2)"),
        rpn("1 2 min"),
        vec![1.0f64.into(), 2.0f64.into(), Function::Min.into()],
        1.0
    );
}
#[test]
fn parse_ln() {
    assert_correct!(
        infix("ln(e)"),
        rpn("e ln"),
        vec![E.into(), Function::Ln.into()],
        1.0
    );
}
#[test]
fn parse_exp() {
    assert_correct!(
        infix("exp(1)"),
        rpn("1 exp"),
        vec![1.0f64.into(), Function::Exp.into()],
        E
    );
}
#[test]
fn parse_max() {
    assert_correct!(
        infix("max(1,2)"),
        rpn("1 2 max"),
        vec![1.0f64.into(), 2.0f64.into(), Function::Max.into()],
        2.0
    );
}
#[test]
fn parse_cos() {
    assert_correct!(
        infix("cos(pi/6)"),
        rpn("pi 6 / cos"),
        vec![
            PI.into(),
            6.0f64.into(),
            Operators::Div.into(),
            Function::Cos.into()
        ],
        (PI / 6.0).cos()
    );
}
#[test]
fn parse_acos() {
    assert_correct!(
        infix("acos(3//2/2)"),
        rpn("3 2 // 2 / acos"),
        vec![
            3.0f64.into(),
            2.0f64.into(),
            Operators::Root.into(),
            2.0f64.into(),
            Operators::Div.into(),
            Function::Acos.into()
        ],
        (3.0f64.sqrt() / 2.0).acos()
    );
}
#[test]
fn parse_sin() {
    assert_correct!(
        infix("sin(pi/6)"),
        rpn("pi 6 / sin"),
        vec![
            PI.into(),
            6.0f64.into(),
            Operators::Div.into(),
            Function::Sin.into()
        ],
        (PI / 6.0).sin()
    );
}
#[test]
fn parse_asin() {
    assert_correct!(
        infix("asin(1/2)"),
        rpn("1 2 / asin"),
        vec![
            1.0f64.into(),
            2.0f64.into(),
            Operators::Div.into(),
            Function::Asin.into()
        ],
        (1.0f64 / 2.0).asin()
    );
}
#[test]
fn parse_atan() {
    assert_correct!(
        infix("atan(1,1)"),
        rpn("1 1 atan"),
        vec![1.0f64.into(), 1.0f64.into(), Function::Atan.into()],
        std::f64::consts::FRAC_PI_4
    );
}
#[test]
fn parse_quadratic() {
    assert_correct!(
        infix("quadratic(1,-2,-1)"),
        rpn("1 2 _ 1 _ quadratic"),
        vec![
            1.0f64.into(),
            2.0f64.into(),
            Operators::Negate.into(),
            1.0f64.into(),
            Operators::Negate.into(),
            Function::Quadratic.into()
        ],
        1.0 + 2.0f64.sqrt()
    );
    assert_correct!(
        infix("quadratic((4-2)/2,3-2-3,-ln(e))"),
        rpn("4 2 - 2 / 3 2 - 3 - e ln _ quadratic"),
        vec![
            4.0f64.into(),
            2.0f64.into(),
            Operators::Sub.into(),
            2.0f64.into(),
            Operators::Div.into(),
            3.0f64.into(),
            2.0f64.into(),
            Operators::Sub.into(),
            3.0f64.into(),
            Operators::Sub.into(),
            E.into(),
            Function::Ln.into(),
            Operators::Negate.into(),
            Function::Quadratic.into()
        ],
        1.0 + 2.0f64.sqrt()
    );
}
#[test]
fn parse_number() {
    assert_correct!(infix("0.5"), rpn("0.5"), vec![0.5f64.into()], 0.5);
}
#[test]
fn parse_order_of_operations() {
    assert_correct!(
        infix("-2*3+4*7+-2^2^3"),
        rpn("2 _ 3 * 4 7 * + 2 2 3 ^ ^ _ +"),
        vec![
            2.0f64.into(),
            Operators::Negate.into(),
            3.0f64.into(),
            Operators::Mul.into(),
            4.0f64.into(),
            7.0f64.into(),
            Operators::Mul.into(),
            Operators::Add.into(),
            2.0f64.into(),
            2.0f64.into(),
            3.0f64.into(),
            Operators::Pow.into(),
            Operators::Pow.into(),
            Operators::Negate.into(),
            Operators::Add.into(),
        ],
        -234.0
    );
    assert_correct!(
        infix("sin(max(2,3)/3*pi)"),
        rpn("2 3 max 3 / pi * sin"),
        vec![
            2.0f64.into(),
            3.0f64.into(),
            Function::Max.into(),
            3.0f64.into(),
            Operators::Div.into(),
            PI.into(),
            Operators::Mul.into(),
            Function::Sin.into(),
        ],
        PI.sin()
    );
}
#[test]
fn test_graph_vars() {
    let vars = Variables(vec![Variable::new("x", 0.0, false)]);
    let mut infix = Parsed::infix("x", vars.clone(), Functions::default()).unwrap();
    let mut rpn = Parsed::rpn("x", vars, Functions::default()).unwrap();
    assert_correct!(infix.clone(), rpn.clone(), vec![Token::Var(0)], 0.0);
    rpn.vars[0].value = 1.0;
    infix.vars[0].value = 1.0;
    assert_correct!(infix, rpn, vec![Token::Var(0)], 1.0);
}
#[test]
fn test_custom_functions() {
    let funs = Functions(vec![FunctionVar::new(
        "f",
        InnerVariables(vec![InnerVariable::new(0.0), InnerVariable::new(0.0)]),
        Tokens(vec![
            Token::InnerVar(0),
            Token::InnerVar(1),
            Operators::Sub.into(),
        ]),
    )]);
    assert_correct!(
        Parsed::infix("f(3,4)", Variables::default(), funs.clone()).unwrap(),
        Parsed::rpn("3 4 f", Variables::default(), funs.clone()).unwrap(),
        vec![3.0f64.into(), 4.0f64.into(), Token::Fun(0)],
        -1.0
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
