use crate::parse::{Function, Operators, Parsed};
use std::f64::consts::{E, PI};
macro_rules! assert_teq {
    ($a:expr, $b:expr, $c:expr) => {
        assert_eq!($a, $b);
        assert_eq!($a, $c);
    };
}
macro_rules! assert_correct {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        assert_teq!($a.parsed, $b.parsed, $c);
        assert_teq!($a.compute(), $b.compute(), $d);
    };
}
#[test]
fn parse_neg() {
    assert_correct!(
        Parsed::infix("-4").unwrap(),
        Parsed::rpn("4 _").unwrap(),
        vec![4.0f64.into(), Operators::Negate.into()],
        -4.0
    );
}
#[test]
fn parse_mul() {
    assert_correct!(
        Parsed::infix("2*4").unwrap(),
        Parsed::rpn("2 4 *").unwrap(),
        vec![2.0f64.into(), 4.0f64.into(), Operators::Mul.into()],
        8.0
    );
}
#[test]
fn parse_add() {
    assert_correct!(
        Parsed::infix("2+4").unwrap(),
        Parsed::rpn("2 4 +").unwrap(),
        vec![2.0f64.into(), 4.0f64.into(), Operators::Add.into()],
        6.0
    );
}
#[test]
fn parse_sub() {
    assert_correct!(
        Parsed::infix("2-4").unwrap(),
        Parsed::rpn("2 4 -").unwrap(),
        vec![2.0f64.into(), 4.0f64.into(), Operators::Sub.into()],
        -2.0
    );
}
#[test]
fn parse_div() {
    assert_correct!(
        Parsed::infix("2/4").unwrap(),
        Parsed::rpn("2 4 /").unwrap(),
        vec![2.0f64.into(), 4.0f64.into(), Operators::Div.into()],
        0.5
    );
}
#[test]
fn parse_pow() {
    assert_correct!(
        Parsed::infix("2^4").unwrap(),
        Parsed::rpn("2 4 ^").unwrap(),
        vec![2.0f64.into(), 4.0f64.into(), Operators::Pow.into()],
        16.0
    );
    assert_correct!(
        Parsed::infix("2**4").unwrap(),
        Parsed::rpn("2 4 **").unwrap(),
        vec![2.0f64.into(), 4.0f64.into(), Operators::Pow.into()],
        16.0
    );
}
#[test]
fn parse_root() {
    assert_correct!(
        Parsed::infix("4//2").unwrap(),
        Parsed::rpn("4 2 //").unwrap(),
        vec![4.0f64.into(), 2.0f64.into(), Operators::Root.into()],
        2.0
    );
}
#[test]
fn parse_min() {
    assert_correct!(
        Parsed::infix("min(1,2)").unwrap(),
        Parsed::rpn("1 2 min").unwrap(),
        vec![1.0f64.into(), 2.0f64.into(), Function::Min.into()],
        1.0
    );
}
#[test]
fn parse_ln() {
    assert_correct!(
        Parsed::infix("ln(e)").unwrap(),
        Parsed::rpn("e ln").unwrap(),
        vec![E.into(), Function::Ln.into()],
        1.0
    );
}
#[test]
fn parse_exp() {
    assert_correct!(
        Parsed::infix("exp(1)").unwrap(),
        Parsed::rpn("1 exp").unwrap(),
        vec![1.0f64.into(), Function::Exp.into()],
        E
    );
}
#[test]
fn parse_max() {
    assert_correct!(
        Parsed::infix("max(1,2)").unwrap(),
        Parsed::rpn("1 2 max").unwrap(),
        vec![1.0f64.into(), 2.0f64.into(), Function::Max.into()],
        2.0
    );
}
#[test]
fn parse_cos() {
    assert_correct!(
        Parsed::infix("cos(pi/6)").unwrap(),
        Parsed::rpn("pi 6 / cos").unwrap(),
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
        Parsed::infix("acos(3//2/2)").unwrap(),
        Parsed::rpn("3 2 // 2 / acos").unwrap(),
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
        Parsed::infix("sin(pi/6)").unwrap(),
        Parsed::rpn("pi 6 / sin").unwrap(),
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
        Parsed::infix("asin(1/2)").unwrap(),
        Parsed::rpn("1 2 / asin").unwrap(),
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
        Parsed::infix("atan(1,1)").unwrap(),
        Parsed::rpn("1 1 atan").unwrap(),
        vec![1.0f64.into(), 1.0f64.into(), Function::Atan.into()],
        std::f64::consts::FRAC_PI_4
    );
}
#[test]
fn parse_quadratic() {
    assert_correct!(
        Parsed::infix("quadratic(1,-2,-1)").unwrap(),
        Parsed::rpn("1 2 _ 1 _ quadratic").unwrap(),
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
}
#[test]
fn parse_number() {
    assert_correct!(
        Parsed::infix("0.5").unwrap(),
        Parsed::rpn("0.5").unwrap(),
        vec![0.5f64.into()],
        0.5
    );
}
#[test]
fn parse_order_of_operations() {
    assert_correct!(
        Parsed::infix("-2*3+4*7+-2^2^3").unwrap(),
        Parsed::rpn("2 _ 3 * 4 7 * + 2 2 3 ^ ^ _ +").unwrap(),
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
        Parsed::infix("sin(max(2,3)/3*pi)").unwrap(),
        Parsed::rpn("2 3 max 3 / pi * sin").unwrap(),
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
