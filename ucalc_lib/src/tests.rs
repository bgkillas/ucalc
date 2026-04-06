use crate::functions::{AtanInputs, Function, ModifyInputs};
use crate::parse::{Derivative, ParseError};
use crate::parse::{Token, Tokens};
use crate::polynomial::Poly;
#[cfg(feature = "float_rand")]
use crate::rng;
use crate::variable::{Functions, Variables};
use crate::{FUNCTION_LIST, FunctionVar, Number, Variable, Volatility, get_help};
use std::fmt::Debug;
use std::num::NonZeroU8;
use ucalc_numbers::*;
fn assert_approx_eq(a: Number, b: Number) {
    assert!((a - b).abs() < Float::from(2.0).pow(Float::from(-8)))
}
fn assert_approx_teq(a: Number, b: Number, c: Number) {
    assert_eq!(a, b);
    assert_approx_eq(a, c);
}
fn assert_approx_correct(a: &str, b: &str, c: Vec<Token>, d: Number) {
    assert_approx_correct_with(a, b, Variables::default(), &[], Functions::default(), c, d);
}
fn assert_approx_correct_with(
    a: &str,
    b: &str,
    v: Variables,
    vf: &[Number],
    f: Functions,
    c: Vec<Token>,
    d: Number,
) {
    assert_teq(infix(a), rpn(b), Tokens(c));
    assert_approx_teq(
        infix(a).compute(
            vf,
            &f,
            &v,
            #[cfg(feature = "float_rand")]
            &mut rng(),
        ),
        rpn(b).compute(
            vf,
            &f,
            &v,
            #[cfg(feature = "float_rand")]
            &mut rng(),
        ),
        d,
    );
}
fn assert_teq<N: PartialEq + Debug>(a: N, b: N, c: N) {
    assert_eq!(a, c, "a");
    assert_eq!(b, c, "b");
}
fn assert_correct(a: &str, b: &str, c: Vec<Token>, d: Number) {
    assert_correct_with(
        a,
        b,
        &mut Variables::default(),
        &[],
        &[],
        &mut Functions::default(),
        c,
        d,
    );
}
fn assert_correct_with(
    a: &str,
    b: &str,
    v: &mut Variables,
    vv: &[&str],
    vf: &[Number],
    f: &mut Functions,
    c: Vec<Token>,
    d: Number,
) {
    let infix = Tokens::infix(
        a,
        v,
        f,
        vv,
        false,
        false,
        10,
        #[cfg(feature = "float_rand")]
        &mut rng(),
    )
    .unwrap()
    .unwrap();
    let rpn = Tokens::rpn(
        b,
        v,
        f,
        vv,
        false,
        false,
        10,
        #[cfg(feature = "float_rand")]
        &mut rng(),
    )
    .unwrap()
    .unwrap();
    assert_teq(&infix, &rpn, &Tokens(c));
    let infix = Tokens::infix(
        a,
        v,
        f,
        vv,
        false,
        true,
        10,
        #[cfg(feature = "float_rand")]
        &mut rng(),
    )
    .unwrap()
    .unwrap();
    assert_teq(
        infix.compute(
            vf,
            &f,
            &v,
            #[cfg(feature = "float_rand")]
            &mut rng(),
        ),
        rpn.compute(
            vf,
            &f,
            &v,
            #[cfg(feature = "float_rand")]
            &mut rng(),
        ),
        d,
    );
}
fn assert_fun(infix: &str, rpn: &str, expected: &Functions) {
    let mut f1 = Functions::default();
    assert!(
        Tokens::infix(
            infix,
            &mut Variables::default(),
            &mut f1,
            &[],
            true,
            false,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng()
        )
        .unwrap()
        .is_none()
    );
    let mut f2 = Functions::default();
    assert!(
        Tokens::rpn(
            rpn,
            &mut Variables::default(),
            &mut f2,
            &[],
            true,
            false,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng()
        )
        .unwrap()
        .is_none()
    );
    assert_teq(&f1, &f2, expected);
}
fn infix(s: &str) -> Tokens {
    Tokens::infix(
        s,
        &mut Variables::default(),
        &mut Functions::default(),
        &[],
        false,
        true,
        10,
        #[cfg(feature = "float_rand")]
        &mut rng(),
    )
    .unwrap()
    .unwrap()
}
fn rpn(s: &str) -> Tokens {
    Tokens::rpn(
        s,
        &mut Variables::default(),
        &mut Functions::default(),
        &[],
        false,
        true,
        10,
        #[cfg(feature = "float_rand")]
        &mut rng(),
    )
    .unwrap()
    .unwrap()
}
fn res<T>(f: T) -> Number
where
    Number: From<T>,
{
    Number::from(f)
}
fn num<T>(f: T) -> Token
where
    Number: From<T>,
{
    res(f).into()
}
fn var(n: &str) -> Token {
    Token::CustomVar(
        Variables::default()
            .iter()
            .position(|v| v.name.as_ref().is_some_and(|v| v.as_ref() == n))
            .unwrap() as u16,
    )
}
#[test]
fn test_poly_div() {
    let mut buffer = Poly(Vec::new());
    let mut poly = Poly(vec![res(-10), res(21), res(-12), res(1)]);
    poly.div_buffer(&Poly(vec![res(1), res(-2), res(1)]), &mut buffer);
    assert_eq!(poly, Poly(vec![res(0), res(0)]));
    assert_eq!(buffer, Poly(vec![res(-10), res(1)]));
}
#[test]
#[ignore]
#[cfg(feature = "complex")]
fn test_solve_poly() {
    let n = 4usize;
    let k = 10u32;
    let mut fun = Functions(vec![]);
    assert!(
        Tokens::infix(
            "let p(x,a,b,c,d,e,f,g,h,j,k)=(a+b i)x^0+(c+d i)x^1+(e+f i)x^2+(g+h i)x^3+(j+k i)x^4",
            &mut Variables::default(),
            &mut fun,
            &[],
            true,
            true,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng()
        )
        .unwrap()
        .is_none()
    );
    let mut buffer = Vec::with_capacity(8);
    for i in 0..n.pow(k) {
        if i.is_multiple_of(2048) {
            println!("{i} {}", n.pow(k));
        }
        let [a, b, c, d, e, f, g, h, i, j] = std::array::from_fn(|j| {
            ((i as isize >> (j as isize * n.ilog2() as isize)) % n as isize) - n as isize / 2
        });
        if c != 0 || d != 0 || e != 0 || f != 0 || g != 0 || h != 0 || i != 0 || j != 0 {
            let s = format!(
                "p(solve(y,p(y,{a}/2,{b}/2,{c}/2,{d}/2,{e}/2,{f}/2,{g}/2,{h}/2,{i}/2,{j}/2)),{a}/2,{b}/2,{c}/2,{d}/2,{e}/2,{f}/2,{g}/2,{h}/2,{i}/2,{j}/2)"
            );
            let res = Tokens::infix(
                &s,
                &mut Variables::default(),
                &mut fun,
                &[],
                false,
                true,
                10,
                #[cfg(feature = "float_rand")]
                &mut rng(),
            )
            .unwrap()
            .unwrap()
            .compute_buffer_with(
                &mut Vec::with_capacity(8),
                &[],
                &fun,
                &Variables::default(),
                &mut buffer,
                0,
                #[cfg(feature = "float_rand")]
                &mut rng(),
            );
            assert!(res.abs() < Float::from(2.0).pow(Float::from(-8)), "{s}");
        }
    }
}
#[test]
fn parse_neg() {
    assert_correct("-4", "4 ~", vec![num(4), Function::Negate.into()], res(-4));
    assert_correct("~4", "4 ~", vec![num(4), Function::Negate.into()], res(-4));
}
#[test]
fn parse_fact() {
    assert_correct(
        "5!",
        "5 !",
        vec![num(5), Function::Factorial.into()],
        res(120),
    );
    assert_correct(
        "3!^2",
        "3 ! 2 ^",
        vec![
            num(3),
            Function::Factorial.into(),
            num(2),
            Function::Pow.into(),
        ],
        res(36),
    );
}
#[test]
fn parse_mul() {
    assert_correct(
        "epie",
        "e pi * e *",
        vec![
            var("e"),
            var("pi"),
            Function::Mul.into(),
            var("e"),
            Function::Mul.into(),
        ],
        res(Constant::Pi) * res(Constant::E) * res(Constant::E),
    );
    assert_correct(
        "2*4",
        "2 4 *",
        vec![num(2), num(4), Function::Mul.into()],
        res(8),
    );
    assert_correct(
        "set(2,x,2x^2)",
        "2 x 2 x 2 ^ * set",
        vec![
            num(2),
            Token::Skip(5),
            num(2),
            Token::InnerVar(0),
            num(2),
            Function::Pow.into(),
            Function::Mul.into(),
            Function::Set.into(),
        ],
        res(8),
    );
    assert_correct(
        "2/3*3",
        "2 3 / 3 *",
        vec![
            num(2),
            num(3),
            Function::Div.into(),
            num(3),
            Function::Mul.into(),
        ],
        res(2),
    );
    assert_correct(
        "2/3 3",
        "2 3 / 3 *",
        vec![
            num(2),
            num(3),
            Function::Div.into(),
            num(3),
            Function::Mul.into(),
        ],
        res(2),
    );
    assert_correct(
        "(2)(3)",
        "2 3 *",
        vec![num(2), num(3), Function::Mul.into()],
        res(6),
    );
    assert_correct(
        "sqrt(4)sqrt(4)",
        "4 sqrt 4 sqrt *",
        vec![
            num(4),
            Function::Sqrt.into(),
            num(4),
            Function::Sqrt.into(),
            Function::Mul.into(),
        ],
        res(4),
    );
}
#[test]
fn parse_add() {
    assert_correct(
        "2+4",
        "2 4 +",
        vec![num(2), num(4), Function::Add.into()],
        res(6),
    );
}
#[test]
fn parse_sub() {
    assert_correct(
        "2-4",
        "2 4 -",
        vec![num(2), num(4), Function::Sub.into()],
        res(-2),
    );
}
#[test]
fn parse_div() {
    assert_correct(
        "2/4",
        "2 4 /",
        vec![num(2), num(4), Function::Div.into()],
        res(0.5),
    );
}
#[test]
fn parse_rem() {
    assert_correct(
        "7%4",
        "7 4 %",
        vec![num(7), num(4), Function::Rem.into()],
        res(3),
    );
    assert_correct(
        "7%4^2",
        "7 4 % 2 ^",
        vec![
            num(7),
            num(4),
            Function::Rem.into(),
            num(2),
            Function::Pow.into(),
        ],
        res(9),
    );
}
#[test]
fn test_inverses() {
    for f in [
        Function::Add,
        Function::Sub,
        Function::Mul,
        Function::Div,
        Function::Pow,
        Function::Root,
        Function::Negate,
        Function::Sin,
        Function::Cos,
        Function::Ln,
        Function::Asin,
        Function::Acos,
        Function::Exp,
        Function::Recip,
        #[cfg(feature = "complex")]
        Function::Conj,
        Function::Sinh,
        #[cfg(feature = "complex")]
        Function::Cosh,
        Function::Asinh,
        Function::Acosh,
        Function::Atanh,
        Function::Tanh,
        Function::Tan,
        Function::Atan(AtanInputs::One),
        Function::Sqrt,
        Function::Sq,
        Function::Cbrt,
        Function::Cb,
    ] {
        match f.inputs().get() {
            1 => {
                assert_approx_correct(
                    &format!("{f}(solve(x,{f}(x)-0.5))"),
                    &format!("x x {f} 0.5 - solve {f}"),
                    vec![
                        Token::Skip(4),
                        Token::InnerVar(0).into(),
                        f.into(),
                        num(0.5),
                        Function::Sub.into(),
                        Function::Solve.into(),
                        f.into(),
                    ],
                    res(0.5),
                );
            }
            2 => {
                assert_approx_correct(
                    &format!("{f}(solve(x,{f}(x,0.5)-0.5),0.5)"),
                    &format!("x x 0.5 {f} 0.5 - solve 0.5 {f}"),
                    vec![
                        Token::Skip(5),
                        Token::InnerVar(0).into(),
                        num(0.5),
                        f.into(),
                        num(0.5),
                        Function::Sub.into(),
                        Function::Solve.into(),
                        num(0.5),
                        f.into(),
                    ],
                    res(0.5),
                );
                assert_approx_correct(
                    &format!("{f}(0.5,solve(x,{f}(0.5,x)-0.5))"),
                    &format!("0.5 x 0.5 x {f} 0.5 - solve {f}"),
                    vec![
                        num(0.5),
                        Token::Skip(5),
                        num(0.5),
                        Token::InnerVar(0).into(),
                        f.into(),
                        num(0.5),
                        Function::Sub.into(),
                        Function::Solve.into(),
                        f.into(),
                    ],
                    res(0.5),
                );
            }
            _ => unreachable!(),
        }
    }
}
#[test]
fn parse_pow() {
    assert_correct(
        "2^4",
        "2 4 ^",
        vec![num(2), num(4), Function::Pow.into()],
        res(16),
    );
    assert_correct(
        "2^-4",
        "2 4 ~ ^",
        vec![
            num(2),
            num(4),
            Function::Negate.into(),
            Function::Pow.into(),
        ],
        res(16).recip(),
    );
    assert_correct(
        "1+-2^-4",
        "1 2 4 ~ ^ ~ +",
        vec![
            num(1),
            num(2),
            num(4),
            Function::Negate.into(),
            Function::Pow.into(),
            Function::Negate.into(),
            Function::Add.into(),
        ],
        res(1) - res(16).recip(),
    );
    assert_correct(
        "2**4",
        "2 4 **",
        vec![num(2), num(4), Function::Pow.into()],
        res(16),
    );
}
#[test]
fn parse_root() {
    assert_correct(
        "4//2",
        "4 2 //",
        vec![num(4), num(2), Function::Root.into()],
        res(2),
    );
}
#[test]
fn parse_min() {
    assert_correct(
        "min(1,2)",
        "1 2 min",
        vec![num(1), num(2), Function::Min.into()],
        res(1),
    );
}
#[test]
fn parse_ln() {
    assert_correct("ln(e)", "e ln", vec![var("e"), Function::Ln.into()], res(1));
}
#[test]
fn parse_exp() {
    assert_correct(
        "exp(1)",
        "1 exp",
        vec![num(1), Function::Exp.into()],
        res(Constant::E),
    );
}
#[test]
fn parse_max() {
    assert_correct(
        "max(1,2)",
        "1 2 max",
        vec![num(1), num(2), Function::Max.into()],
        res(2),
    );
}
#[test]
fn parse_vars() {
    assert_correct("pi", "pi", vec![var("pi")], res(Constant::Pi));
    assert_correct("e", "e", vec![var("e")], res(Constant::E));
    assert_correct("tau", "tau", vec![var("tau")], res(Constant::Tau));
    assert_correct("inf", "inf", vec![var("inf")], res(Constant::Infinity));
    #[cfg(feature = "complex")]
    assert_correct("i", "i", vec![var("i")], res((0, 1)));
}
#[test]
fn parse_cos() {
    assert_correct(
        "cos(pi/6)",
        "pi 6 / cos",
        vec![
            var("pi"),
            num(6),
            Function::Div.into(),
            Function::Cos.into(),
        ],
        (res(Constant::Pi) / Float::from(6)).cos(),
    );
}
#[test]
fn parse_acos() {
    assert_correct(
        "acos(3//2/2)",
        "3 2 // 2 / acos",
        vec![
            num(3),
            num(2),
            Function::Root.into(),
            num(2),
            Function::Div.into(),
            Function::Acos.into(),
        ],
        (res(3).sqrt() / Float::from(2)).acos(),
    );
}
#[test]
fn test_polynomial() {
    assert_correct(
        "solve(x,(x-2)(x-3)/(x-3))",
        "x x 2 - x 3 - * x 3 - / solve",
        vec![
            Token::Skip(11),
            Token::InnerVar(0),
            num(2),
            Function::Sub.into(),
            Token::InnerVar(0),
            num(3),
            Function::Sub.into(),
            Function::Mul.into(),
            Token::InnerVar(0),
            num(3),
            Function::Sub.into(),
            Function::Div.into(),
            Function::Solve.into(),
        ],
        res(2),
    );
    assert_correct(
        "solve(x,(x-2)(x-3)/(x-3)/(x-3))",
        "x x 2 - x 3 - * x 3 - / x 3 - / solve",
        vec![
            Token::Skip(15),
            Token::InnerVar(0),
            num(2),
            Function::Sub.into(),
            Token::InnerVar(0),
            num(3),
            Function::Sub.into(),
            Function::Mul.into(),
            Token::InnerVar(0),
            num(3),
            Function::Sub.into(),
            Function::Div.into(),
            Token::InnerVar(0),
            num(3),
            Function::Sub.into(),
            Function::Div.into(),
            Function::Solve.into(),
        ],
        res(2),
    );
    assert_correct(
        "solve(x,x+x^2)",
        "x x x 2 ^ + solve",
        vec![
            Token::Skip(5),
            Token::InnerVar(0),
            Token::InnerVar(0),
            num(2),
            Function::Pow.into(),
            Function::Add.into(),
            Function::Solve.into(),
        ],
        res(-1),
    );
    #[cfg(feature = "complex")]
    assert_correct(
        "solve(x,(x^0.5-2)(x^0.5+2))",
        "x x 0.5 ^ 2 - x 0.5 ^ 2 + * solve",
        vec![
            Token::Skip(11),
            Token::InnerVar(0),
            num(0.5),
            Function::Pow.into(),
            num(2),
            Function::Sub.into(),
            Token::InnerVar(0),
            num(0.5),
            Function::Pow.into(),
            num(2),
            Function::Add.into(),
            Function::Mul.into(),
            Function::Solve.into(),
        ],
        res(4),
    );
    #[cfg(feature = "complex")]
    assert_correct(
        "solve(x,(x-2)(x-1)(x+1)(x+2))",
        "x x 2 - x 1 - * x 1 + * x 2 + * solve",
        vec![
            Token::Skip(15),
            Token::InnerVar(0),
            num(2),
            Function::Sub.into(),
            Token::InnerVar(0),
            num(1),
            Function::Sub.into(),
            Function::Mul.into(),
            Token::InnerVar(0),
            num(1),
            Function::Add.into(),
            Function::Mul.into(),
            Token::InnerVar(0),
            num(2),
            Function::Add.into(),
            Function::Mul.into(),
            Function::Solve.into(),
        ],
        res(1),
    );
    #[cfg(feature = "complex")]
    assert_correct(
        "solve(x,(x-2)(x-1)(x+1)(x+3))",
        "x x 2 - x 1 - * x 1 + * x 3 + * solve",
        vec![
            Token::Skip(15),
            Token::InnerVar(0),
            num(2),
            Function::Sub.into(),
            Token::InnerVar(0),
            num(1),
            Function::Sub.into(),
            Function::Mul.into(),
            Token::InnerVar(0),
            num(1),
            Function::Add.into(),
            Function::Mul.into(),
            Token::InnerVar(0),
            num(3),
            Function::Add.into(),
            Function::Mul.into(),
            Function::Solve.into(),
        ],
        res(-3),
    );
    #[cfg(feature = "complex")]
    assert_approx_correct(
        "solve(x,(x-2)(x-1)(x+3))",
        "x x 2 - x 1 - * x 3 + * solve",
        vec![
            Token::Skip(11),
            Token::InnerVar(0),
            num(2),
            Function::Sub.into(),
            Token::InnerVar(0),
            num(1),
            Function::Sub.into(),
            Function::Mul.into(),
            Token::InnerVar(0),
            num(3),
            Function::Add.into(),
            Function::Mul.into(),
            Function::Solve.into(),
        ],
        res(-3),
    );
}
#[test]
fn parse_sin() {
    assert_correct(
        "sin(pi/6)",
        "pi 6 / sin",
        vec![
            var("pi"),
            num(6),
            Function::Div.into(),
            Function::Sin.into(),
        ],
        (res(Constant::Pi) / Float::from(6)).sin(),
    );
}
#[test]
fn parse_sinh() {
    assert_correct(
        "sinh(ln(2))",
        "2 ln sinh",
        vec![num(2), Function::Ln.into(), Function::Sinh.into()],
        res(0.75),
    );
    assert_correct(
        "asinh((e-1/e)/2)",
        "e 1 e / - 2 / asinh",
        vec![
            var("e"),
            num(1),
            var("e"),
            Function::Div.into(),
            Function::Sub.into(),
            num(2),
            Function::Div.into(),
            Function::Asinh.into(),
        ],
        res(1),
    );
}
#[test]
fn parse_cosh() {
    assert_correct(
        "cosh(ln(2))",
        "2 ln cosh",
        vec![num(2), Function::Ln.into(), Function::Cosh.into()],
        res(1.25),
    );
    assert_correct(
        "acosh((e+1/e)/2)",
        "e 1 e / + 2 / acosh",
        vec![
            var("e"),
            num(1),
            var("e"),
            Function::Div.into(),
            Function::Add.into(),
            num(2),
            Function::Div.into(),
            Function::Acosh.into(),
        ],
        res(1),
    );
}
#[test]
fn parse_tanh() {
    assert_correct(
        "tanh(1)",
        "1 tanh",
        vec![num(1), Function::Tanh.into()],
        res(1).sinh() / res(1).cosh(),
    );
    assert_correct(
        "atanh(0.5)",
        "0.5 atanh",
        vec![num(0.5), Function::Atanh.into()],
        res(0.5).atanh(),
    );
}
#[test]
fn parse_tan() {
    assert_correct(
        "tan(pi/6)",
        "pi 6 / tan",
        vec![
            var("pi"),
            num(6),
            Function::Div.into(),
            Function::Tan.into(),
        ],
        (res(Constant::Pi) / Float::from(6)).tan(),
    );
}
#[test]
fn parse_sqrt() {
    assert_correct(
        "sqrt(4)",
        "4 sqrt",
        vec![num(4), Function::Sqrt.into()],
        res(2),
    );
    assert_correct("sq(4)", "4 sq", vec![num(4), Function::Sq.into()], res(16));
}
#[test]
fn parse_cbrt() {
    assert_correct(
        "cbrt(8)",
        "8 cbrt",
        vec![num(8), Function::Cbrt.into()],
        res(2),
    );
    assert_correct("cb(2)", "2 cb", vec![num(2), Function::Cb.into()], res(8));
}
#[test]
fn parse_gamma() {
    assert_correct(
        "gamma(4)",
        "4 gamma",
        vec![num(4), Function::Gamma.into()],
        res(6),
    );
}
#[test]
#[cfg(feature = "float_rand")]
fn parse_rand_uniform() {
    assert_teq(
        infix("rand_uniform(2,3)"),
        rpn("2 3 rand_uniform"),
        Tokens(vec![num(2), num(3), Function::RandUniform.into()]),
    );
    let n = infix("rand_uniform(2,3)").compute(
        &[],
        &[],
        &[],
        #[cfg(feature = "float_rand")]
        &mut rng(),
    );
    #[cfg(feature = "complex")]
    assert!(n.imag.is_zero());
    let n = n.real();
    assert!(n <= &Float::from(3));
    assert!(n >= &Float::from(2));
}
#[test]
fn parse_erf() {
    assert_correct(
        "erf(100)",
        "100 erf",
        vec![num(100), Function::Erf.into()],
        res(1),
    );
}
#[test]
fn parse_erfc() {
    assert_correct(
        "erfc(100)",
        "100 erfc",
        vec![num(100), Function::Erfc.into()],
        res(0),
    );
}
#[test]
fn parse_abs() {
    #[cfg(feature = "complex")]
    assert_correct(
        "abs(2+2*i)",
        "2 2 i * + abs",
        vec![
            num(2),
            num(2),
            var("i"),
            Function::Mul.into(),
            Function::Add.into(),
            Function::Abs.into(),
        ],
        res(8).sqrt(),
    );
    assert_correct(
        "ln|-1+0.5|",
        "1 ~ 0.5 + abs ln",
        vec![
            num(1),
            Function::Negate.into(),
            num(0.5),
            Function::Add.into(),
            Function::Abs.into(),
            Function::Ln.into(),
        ],
        res(0.5).ln(),
    );
    assert_correct("|1|", "1 abs", vec![num(1), Function::Abs.into()], res(1));
    assert_correct(
        "||0|-0|",
        "0 abs 0 - abs",
        vec![
            num(0),
            Function::Abs.into(),
            num(0),
            Function::Sub.into(),
            Function::Abs.into(),
        ],
        res(0),
    );
    assert_correct(
        "||0!|-|0^1*|0|-|-2|||+|0|",
        "0 ! abs 0 1 ^ 0 abs * 2 ~ abs - abs - abs 0 abs +",
        vec![
            num(0),
            Function::Factorial.into(),
            Function::Abs.into(),
            num(0),
            num(1),
            Function::Pow.into(),
            num(0),
            Function::Abs.into(),
            Function::Mul.into(),
            num(2),
            Function::Negate.into(),
            Function::Abs.into(),
            Function::Sub.into(),
            Function::Abs.into(),
            Function::Sub.into(),
            Function::Abs.into(),
            num(0),
            Function::Abs.into(),
            Function::Add.into(),
        ],
        res(1),
    );
}
#[test]
fn test_empty() {
    assert_correct("", "", vec![num(0)], res(0));
}
#[test]
#[cfg(feature = "complex")]
fn parse_arg() {
    assert_correct(
        "arg(2+2*i)",
        "2 2 i * + arg",
        vec![
            num(2),
            num(2),
            var("i"),
            Function::Mul.into(),
            Function::Add.into(),
            Function::Arg.into(),
        ],
        res(Constant::Pi) / Float::from(4),
    );
}
#[test]
#[cfg(feature = "complex")]
fn parse_conj() {
    assert_correct(
        "conj(2+2*i)",
        "2 2 i * + conj",
        vec![
            num(2),
            num(2),
            var("i"),
            Function::Mul.into(),
            Function::Add.into(),
            Function::Conj.into(),
        ],
        res((2, -2)),
    );
}
#[test]
fn parse_recip() {
    assert_correct(
        "recip(2)",
        "2 recip",
        vec![num(2), Function::Recip.into()],
        res(0.5),
    );
}
#[test]
fn parse_asin() {
    assert_correct(
        "asin(1/2)",
        "1 2 / asin",
        vec![num(1), num(2), Function::Div.into(), Function::Asin.into()],
        (res(1) / Float::from(2)).asin(),
    );
}
#[test]
fn parse_atan() {
    assert_correct(
        "atan(1,1)",
        "1 1 atan2",
        vec![num(1), num(1), Function::Atan(AtanInputs::Two).into()],
        res(Constant::Pi) / Float::from(4),
    );
}
#[test]
fn parse_arctan() {
    assert_correct(
        "atan(1)",
        "1 atan1",
        vec![num(1), Function::Atan(AtanInputs::One).into()],
        res(Constant::Pi) / Float::from(4),
    );
}
#[test]
#[cfg(feature = "complex")]
fn parse_cubic() {
    assert_correct(
        "cubic(1,-2,0,1)",
        "1 2 ~ 0 1 cubic",
        vec![
            num(1),
            num(2),
            Function::Negate.into(),
            num(0),
            num(1),
            Function::Cubic.into(),
        ],
        (res(1) + res(5).sqrt()) / res(2),
    );
}
#[test]
#[cfg(feature = "complex")]
fn parse_quartic() {
    assert_correct(
        "quartic(1,0,-13,0,36)",
        "1 0 13 ~ 0 36 quartic",
        vec![
            num(1),
            num(0),
            num(13),
            Function::Negate.into(),
            num(0),
            num(36),
            Function::Quartic.into(),
        ],
        res(3),
    );
}
#[test]
fn parse_quadratic() {
    assert_correct(
        "quadratic(1,-2,-1)",
        "1 2 ~ 1 ~ quadratic",
        vec![
            num(1),
            num(2),
            Function::Negate.into(),
            num(1),
            Function::Negate.into(),
            Function::Quadratic.into(),
        ],
        res(2).sqrt() + Float::from(1),
    );
    assert_correct(
        "quadratic((4-2)/2,3-2-3,-ln(e))",
        "4 2 - 2 / 3 2 - 3 - e ln ~ quadratic",
        vec![
            num(4),
            num(2),
            Function::Sub.into(),
            num(2),
            Function::Div.into(),
            num(3),
            num(2),
            Function::Sub.into(),
            num(3),
            Function::Sub.into(),
            var("e"),
            Function::Ln.into(),
            Function::Negate.into(),
            Function::Quadratic.into(),
        ],
        res(2).sqrt() + Float::from(1),
    );
}
#[test]
fn parse_number() {
    assert_correct("0.5", "0.5", vec![num(0.5)], res(0.5));
}
#[test]
fn parse_order_of_operations() {
    assert_correct(
        "-2*3+4*7+-2^2^3",
        "2 ~ 3 * 4 7 * + 2 2 3 ^ ^ ~ +",
        vec![
            num(2),
            Function::Negate.into(),
            num(3),
            Function::Mul.into(),
            num(4),
            num(7),
            Function::Mul.into(),
            Function::Add.into(),
            num(2),
            num(2),
            num(3),
            Function::Pow.into(),
            Function::Pow.into(),
            Function::Negate.into(),
            Function::Add.into(),
        ],
        res(-234),
    );
    assert_correct(
        "sin(max(2,3)/3*pi)",
        "2 3 max 3 / pi * sin",
        vec![
            num(2),
            num(3),
            Function::Max.into(),
            num(3),
            Function::Div.into(),
            var("pi"),
            Function::Mul.into(),
            Function::Sin.into(),
        ],
        res(Constant::Pi).sin(),
    );
}
#[test]
fn test_graph_vars() {
    assert_correct_with(
        "x^y",
        "x y ^",
        &mut Variables::default(),
        &["x", "y"],
        &[Number::from(2), Number::from(3)],
        &mut Functions::default(),
        vec![Token::GraphVar(0), Token::GraphVar(1), Function::Pow.into()],
        res(8),
    );
    assert_correct_with(
        "x^y",
        "x y ^",
        &mut Variables::default(),
        &["x", "y"],
        &[Number::from(3), Number::from(2)],
        &mut Functions::default(),
        vec![Token::GraphVar(0), Token::GraphVar(1), Function::Pow.into()],
        res(9),
    );
}
#[test]
fn test_set() {
    assert_correct(
        "set(2,x,x^2)",
        "2 x x 2 ^ set",
        vec![
            num(2),
            Token::Skip(3),
            Token::InnerVar(0).into(),
            num(2),
            Function::Pow.into(),
            Function::Set.into(),
        ],
        res(4),
    );
    assert_correct(
        "set(solve(x,1+x),l,l)",
        "x 1 x + solve l l set",
        vec![
            Token::Skip(3),
            num(1),
            Token::InnerVar(0),
            Function::Add.into(),
            Function::Solve.into(),
            Token::Skip(1),
            Token::InnerVar(0),
            Function::Set.into(),
        ],
        res(-1),
    );
}
#[test]
fn test_numerical_differential() {
    assert_approx_correct(
        "numerical_differential(0,0,1,x+t)",
        "0 0 1 x t x t + numerical_differential",
        vec![
            num(0),
            num(0),
            num(1),
            Token::Skip(3),
            Token::InnerVar(0).into(),
            Token::InnerVar(1).into(),
            Function::Add.into(),
            Function::NumericalDifferential.into(),
        ],
        res(Constant::E) - res(2),
    );
}
#[test]
fn test_numerical_integral() {
    assert_approx_correct(
        "numerical_integral(2,3,x,x^2-2)",
        "2 3 x x 2 ^ 2 - numerical_integral",
        vec![
            num(2),
            num(3),
            Token::Skip(5),
            Token::InnerVar(0).into(),
            num(2),
            Function::Pow.into(),
            num(2),
            Function::Sub.into(),
            Function::NumericalIntegral.into(),
        ],
        res(13) / res(3),
    );
}
#[test]
fn test_numerical_derivative() {
    assert_approx_correct(
        "numerical_derivative(2,x,x^2-2)",
        "2 x x 2 ^ 2 - numerical_derivative",
        vec![
            num(2),
            Token::Skip(5),
            Token::InnerVar(0).into(),
            num(2),
            Function::Pow.into(),
            num(2),
            Function::Sub.into(),
            Function::NumericalDerivative.into(),
        ],
        res(4),
    );
}
#[test]
fn test_numerical_solve() {
    assert_approx_correct(
        "numerical_solve(2,x,x^2-2)",
        "2 x x 2 ^ 2 - numerical_solve",
        vec![
            num(2),
            Token::Skip(5),
            Token::InnerVar(0).into(),
            num(2),
            Function::Pow.into(),
            num(2),
            Function::Sub.into(),
            Function::NumericalSolve.into(),
        ],
        res(2).sqrt(),
    );
}
#[test]
fn test_solve() {
    assert_correct(
        "1^2+y^2=2",
        "y 1 2 ^ y 2 ^ + 2 - solve",
        vec![
            Token::Skip(9),
            num(1),
            num(2),
            Function::Pow.into(),
            Token::InnerVar(0),
            num(2),
            Function::Pow.into(),
            Function::Add.into(),
            num(2),
            Function::Sub.into(),
            Function::Solve.into(),
        ],
        res(1),
    );
    assert_correct(
        "0.5^2+y^2=1",
        "y 0.5 2 ^ y 2 ^ + 1 - solve",
        vec![
            Token::Skip(9),
            num(0.5),
            num(2),
            Function::Pow.into(),
            Token::InnerVar(0),
            num(2),
            Function::Pow.into(),
            Function::Add.into(),
            num(1),
            Function::Sub.into(),
            Function::Solve.into(),
        ],
        res(3).sqrt() / res(2),
    );
    assert_correct(
        "solve(x,2-3x)",
        "x 2 3 x * - solve",
        vec![
            Token::Skip(5),
            num(2),
            num(3),
            Token::InnerVar(0).into(),
            Function::Mul.into(),
            Function::Sub.into(),
            Function::Solve.into(),
        ],
        res(2) / res(3),
    );
    assert_correct(
        "solve(x,x^4-2x^2+1)",
        "x x 4 ^ 2 x 2 ^ * - 1 + solve",
        vec![
            Token::Skip(11),
            Token::InnerVar(0).into(),
            num(4),
            Function::Pow.into(),
            num(2),
            Token::InnerVar(0).into(),
            num(2),
            Function::Pow.into(),
            Function::Mul.into(),
            Function::Sub.into(),
            num(1),
            Function::Add.into(),
            Function::Solve.into(),
        ],
        res(1),
    );
    assert_correct(
        "solve(x,4-x-x)",
        "x 4 x - x - solve",
        vec![
            Token::Skip(5),
            num(4),
            Token::InnerVar(0).into(),
            Function::Sub.into(),
            Token::InnerVar(0).into(),
            Function::Sub.into(),
            Function::Solve.into(),
        ],
        res(2),
    );
    assert_correct(
        "solve(x,x^2-2)",
        "x x 2 ^ 2 - solve",
        vec![
            Token::Skip(5),
            Token::InnerVar(0).into(),
            num(2),
            Function::Pow.into(),
            num(2),
            Function::Sub.into(),
            Function::Solve.into(),
        ],
        res(2).sqrt(),
    );
    assert_correct(
        "solve(x,2*x-1)",
        "x 2 x * 1 - solve",
        vec![
            Token::Skip(5),
            num(2),
            Token::InnerVar(0).into(),
            Function::Mul.into(),
            num(1),
            Function::Sub.into(),
            Function::Solve.into(),
        ],
        res(0.5),
    );
    assert_correct(
        "solve(x,x*x-2*x-1)",
        "x x x * 2 x * - 1 - solve",
        vec![
            Token::Skip(9),
            Token::InnerVar(0).into(),
            Token::InnerVar(0).into(),
            Function::Mul.into(),
            num(2),
            Token::InnerVar(0).into(),
            Function::Mul.into(),
            Function::Sub.into(),
            num(1),
            Function::Sub.into(),
            Function::Solve.into(),
        ],
        Float::from(1) - res(2).sqrt(),
    );
    assert_correct(
        "solve(x,exp(x)^2-2exp(x)+1)",
        "x x exp 2 ^ 2 x exp * - 1 + solve",
        vec![
            Token::Skip(11),
            Token::InnerVar(0).into(),
            Function::Exp.into(),
            num(2),
            Function::Pow.into(),
            num(2),
            Token::InnerVar(0).into(),
            Function::Exp.into(),
            Function::Mul.into(),
            Function::Sub.into(),
            num(1),
            Function::Add.into(),
            Function::Solve.into(),
        ],
        res(0),
    );
    assert_correct(
        "solve(x,ln(x))",
        "x x ln solve",
        vec![
            Token::Skip(2),
            Token::InnerVar(0).into(),
            Function::Ln.into(),
            Function::Solve.into(),
        ],
        res(1),
    );
    let mut funs = Functions(vec![]);
    assert!(
        Tokens::infix(
            "f(n,k)=n-k",
            &mut Variables::default(),
            &mut funs,
            &[],
            true,
            true,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng()
        )
        .is_ok()
    );
    assert!(
        Tokens::infix(
            "g(n)=n^2-3",
            &mut Variables::default(),
            &mut funs,
            &[],
            true,
            true,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng()
        )
        .is_ok()
    );
    assert_correct_with(
        "solve(n,f(g(n+3)+3,2)-1)",
        "n n 3 + g 3 + 2 f 1 - solve",
        &mut Variables::default(),
        &[],
        &[],
        &mut funs,
        vec![
            Token::Skip(10),
            Token::InnerVar(0).into(),
            num(3),
            Function::Add.into(),
            Token::CustomFun(1, Derivative::default()),
            num(3),
            Function::Add.into(),
            num(2),
            Token::CustomFun(0, Derivative::default()),
            num(1),
            Function::Sub.into(),
            Function::Solve.into(),
        ],
        res(3).sqrt() - res(3),
    );
    let mut funs = Functions(vec![]);
    assert!(
        Tokens::infix(
            "f(n,k)=n-k",
            &mut Variables::default(),
            &mut funs,
            &[],
            true,
            true,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng()
        )
        .is_ok()
    );
    assert!(
        Tokens::infix(
            "g(n)=n*n-3",
            &mut Variables::default(),
            &mut funs,
            &[],
            true,
            true,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng()
        )
        .is_ok()
    );
    assert_correct_with(
        "solve(n,f(g(n+3)+3,2)-1)",
        "n n 3 + g 3 + 2 f 1 - solve",
        &mut Variables::default(),
        &[],
        &[],
        &mut funs,
        vec![
            Token::Skip(10),
            Token::InnerVar(0).into(),
            num(3),
            Function::Add.into(),
            Token::CustomFun(1, Derivative::default()),
            num(3),
            Function::Add.into(),
            num(2),
            Token::CustomFun(0, Derivative::default()),
            num(1),
            Function::Sub.into(),
            Function::Solve.into(),
        ],
        res(3).sqrt() - res(3),
    );
}
#[test]
fn test_fold() {
    assert_correct(
        "fold(1,9,1,x,k,x*k)",
        "1 9 1 x k x k * fold",
        vec![
            num(1),
            num(9),
            num(1),
            Token::Skip(3),
            Token::InnerVar(0).into(),
            Token::InnerVar(1).into(),
            Function::Mul.into(),
            Function::Fold.into(),
        ],
        res(362880),
    );
    assert_correct(
        "fold(1,9,1,x*k)",
        "1 9 1 x k x k * fold",
        vec![
            num(1),
            num(9),
            num(1),
            Token::Skip(3),
            Token::InnerVar(0).into(),
            Token::InnerVar(1).into(),
            Function::Mul.into(),
            Function::Fold.into(),
        ],
        res(362880),
    );
    assert_correct(
        "fold(1,9,1,x,x*k)",
        "1 9 1 x k x k * fold",
        vec![
            num(1),
            num(9),
            num(1),
            Token::Skip(3),
            Token::InnerVar(0).into(),
            Token::InnerVar(1).into(),
            Function::Mul.into(),
            Function::Fold.into(),
        ],
        res(362880),
    );
    assert_correct(
        "fold(1,9,0,x,k,x+k)",
        "1 9 0 x k x k + fold",
        vec![
            num(1),
            num(9),
            num(0),
            Token::Skip(3),
            Token::InnerVar(0).into(),
            Token::InnerVar(1).into(),
            Function::Add.into(),
            Function::Fold.into(),
        ],
        res(45),
    );
}
#[test]
fn test_if() {
    assert_correct(
        "if(1,2+1,3+4)",
        "1 2 1 + 3 4 + if",
        vec![
            num(1),
            Token::Skip(3),
            num(2),
            num(1),
            Function::Add.into(),
            Token::Skip(3),
            num(3),
            num(4),
            Function::Add.into(),
            Function::If.into(),
        ],
        res(3),
    );
    assert_correct(
        "if(1,2,3)",
        "1 2 3 if",
        vec![
            num(1),
            Token::Skip(1),
            num(2),
            Token::Skip(1),
            num(3),
            Function::If.into(),
        ],
        res(2),
    );
    assert_correct(
        "if(0,2,3)",
        "0 2 3 if",
        vec![
            num(0),
            Token::Skip(1),
            num(2),
            Token::Skip(1),
            num(3),
            Function::If.into(),
        ],
        res(3),
    );
}
#[test]
fn test_overwrite_var() {
    let vars1 = Variables(vec![Variable::new("n", res(2), Volatility::GraphConstant)]);
    let funs1 = Functions(Vec::new());
    let vars2 = Variables(vec![Variable::new("n", res(4), Volatility::GraphConstant)]);
    let funs2 = Functions(Vec::new());
    let vars3 = Variables(vec![Variable::null(res(4), Volatility::GraphConstant)]);
    let funs3 = Functions(vec![FunctionVar::new(
        "n",
        NonZeroU8::new(1).unwrap(),
        Tokens(vec![
            num(2),
            Token::InnerVar(0).into(),
            Function::Mul.into(),
        ]),
        Volatility::GraphConstant,
    )]);
    let vars4 = Variables(vec![Variable::null(res(4), Volatility::GraphConstant)]);
    let funs4 = Functions(vec![FunctionVar::new(
        "n",
        NonZeroU8::new(1).unwrap(),
        Tokens(vec![
            Token::InnerVar(0).into(),
            num(2),
            Function::Mul.into(),
        ]),
        Volatility::GraphConstant,
    )]);
    let vars5 = Variables(vec![
        Variable::null(res(4), Volatility::GraphConstant),
        Variable::new("n", res(8), Volatility::GraphConstant),
    ]);
    let funs5 = Functions(vec![FunctionVar::null(
        NonZeroU8::new(1).unwrap(),
        Tokens(vec![
            Token::InnerVar(0).into(),
            num(2),
            Function::Mul.into(),
        ]),
        Volatility::GraphConstant,
    )]);
    let mut v = Variables(Vec::new());
    let mut f = Functions(Vec::new());
    assert!(
        Tokens::infix(
            "let n=2",
            &mut v,
            &mut f,
            &[],
            false,
            true,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng()
        )
        .unwrap()
        .is_none()
    );
    assert_eq!(v, vars1);
    assert_eq!(f, funs1);
    assert!(
        Tokens::infix(
            "let n=n2",
            &mut v,
            &mut f,
            &[],
            false,
            true,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng()
        )
        .unwrap()
        .is_none()
    );
    assert_eq!(v, vars2);
    assert_eq!(f, funs2);
    assert!(
        Tokens::infix(
            "let n(k)=2k",
            &mut v,
            &mut f,
            &[],
            false,
            true,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng()
        )
        .unwrap()
        .is_none()
    );
    assert_eq!(v, vars3);
    assert_eq!(f, funs3);
    assert!(
        Tokens::infix(
            "let n(k)=k2",
            &mut v,
            &mut f,
            &[],
            false,
            true,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng()
        )
        .unwrap()
        .is_none()
    );
    assert_eq!(v, vars4);
    assert_eq!(f, funs4);
    assert!(
        Tokens::infix(
            "let n=2n(2)",
            &mut v,
            &mut f,
            &[],
            false,
            true,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng()
        )
        .unwrap()
        .is_none()
    );
    assert_eq!(v, vars5);
    assert_eq!(f, funs5);
    let mut v = Variables(Vec::new());
    let mut f = Functions(Vec::new());
    assert!(
        Tokens::rpn(
            "let n = 2",
            &mut v,
            &mut f,
            &[],
            false,
            true,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng()
        )
        .unwrap()
        .is_none()
    );
    assert_eq!(v, vars1);
    assert_eq!(f, funs1);
    assert!(
        Tokens::rpn(
            "let n = n 2 *",
            &mut v,
            &mut f,
            &[],
            false,
            true,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng()
        )
        .unwrap()
        .is_none()
    );
    assert_eq!(v, vars2);
    assert_eq!(f, funs2);
    assert!(
        Tokens::rpn(
            "let k n = 2 k *",
            &mut v,
            &mut f,
            &[],
            false,
            true,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng()
        )
        .unwrap()
        .is_none()
    );
    assert_eq!(v, vars3);
    assert_eq!(f, funs3);
    assert!(
        Tokens::rpn(
            "let k n = k 2 *",
            &mut v,
            &mut f,
            &[],
            false,
            true,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng()
        )
        .unwrap()
        .is_none()
    );
    assert_eq!(v, vars4);
    assert_eq!(f, funs4);
    assert!(
        Tokens::rpn(
            "let n = 2 2 n *",
            &mut v,
            &mut f,
            &[],
            false,
            true,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng()
        )
        .unwrap()
        .is_none()
    );
    assert_eq!(v, vars5);
    assert_eq!(f, funs5);
}
#[test]
fn test_custom_var() {
    let mut vars = Variables(vec![Variable::new("n", res(2), Volatility::GraphConstant)]);
    let mut v = Variables(Vec::new());
    assert!(
        Tokens::infix(
            "let n=2",
            &mut v,
            &mut Functions::default(),
            &[],
            false,
            true,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng()
        )
        .unwrap()
        .is_none()
    );
    assert_eq!(v, vars);
    let mut v = Variables(Vec::new());
    assert!(
        Tokens::rpn(
            "let n = 2",
            &mut v,
            &mut Functions::default(),
            &[],
            false,
            true,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng()
        )
        .unwrap()
        .is_none()
    );
    assert_eq!(v, vars);
    assert_correct_with(
        "2*n*2",
        "2 n * 2 *",
        &mut vars,
        &[],
        &[],
        &mut Functions::default(),
        vec![
            num(2),
            Token::CustomVar(0),
            Function::Mul.into(),
            num(2),
            Function::Mul.into(),
        ],
        res(8),
    );
}
#[test]
fn test_recursion() {
    let mut funs = Functions(vec![FunctionVar::new(
        "fact",
        NonZeroU8::new(1).unwrap(),
        Tokens(vec![
            Token::InnerVar(0),
            num(0),
            Function::Greater.into(),
            Token::Skip(6),
            Token::InnerVar(0),
            Token::InnerVar(0),
            num(1),
            Function::Sub.into(),
            Token::CustomFun(0, Derivative::default()),
            Function::Mul.into(),
            Token::Skip(1),
            num(1),
            Function::If.into(),
        ]),
        Volatility::GraphConstant,
    )]);
    assert_fun(
        "fact(n)=if(n>0,n*fact(n-1),1)",
        "n fact = n 0 > n n 1 - fact * 1 if",
        &funs,
    );
    assert_correct_with(
        "fact(5)",
        "5 fact",
        &mut Variables::default(),
        &[],
        &[],
        &mut funs,
        vec![num(5), Token::CustomFun(0, Derivative::default())],
        res(120),
    );
}
#[test]
fn test_inner_functions() {
    let f1 = FunctionVar::new(
        "f",
        NonZeroU8::new(2).unwrap(),
        Tokens(vec![
            Token::InnerVar(0),
            Token::InnerVar(1),
            Function::Sub.into(),
        ]),
        Volatility::GraphConstant,
    );
    let f2 = FunctionVar::new(
        "g",
        NonZeroU8::new(2).unwrap(),
        Tokens(vec![
            Token::InnerVar(0),
            Token::InnerVar(1),
            Function::Mul.into(),
            Token::InnerVar(0),
            Token::InnerVar(1),
            Token::CustomFun(0, Derivative::default()),
            Function::Sub.into(),
        ]),
        Volatility::GraphConstant,
    );
    let mut funs = Functions(vec![f1.clone(), f2.clone()]);
    let mut f = Functions::default();
    assert!(
        Tokens::infix(
            "f(n,k)=n-k",
            &mut Variables::default(),
            &mut f,
            &[],
            true,
            true,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng()
        )
        .unwrap()
        .is_none()
    );
    assert!(
        Tokens::infix(
            "g(n,k)=n*k-f(n,k)",
            &mut Variables::default(),
            &mut f,
            &[],
            true,
            true,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng()
        )
        .unwrap()
        .is_none()
    );
    assert_eq!(f, funs);
    let mut f = Functions::default();
    assert!(
        Tokens::rpn(
            "n k f = n k -",
            &mut Variables::default(),
            &mut f,
            &[],
            true,
            true,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng()
        )
        .unwrap()
        .is_none()
    );
    assert!(
        Tokens::rpn(
            "n k g = n k * n k f -",
            &mut Variables::default(),
            &mut f,
            &[],
            true,
            true,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng()
        )
        .unwrap()
        .is_none()
    );
    assert_eq!(f, funs);
    assert_correct_with(
        "g(f(g(2,3)*2,g(3,2)*2)-1,2)",
        "2 3 g 2 * 3 2 g 2 * f 1 - 2 g",
        &mut Variables::default(),
        &[],
        &[],
        &mut funs,
        vec![
            num(2),
            num(3),
            Token::CustomFun(1, Derivative::default()),
            num(2),
            Function::Mul.into(),
            num(3),
            num(2),
            Token::CustomFun(1, Derivative::default()),
            num(2),
            Function::Mul.into(),
            Token::CustomFun(0, Derivative::default()),
            num(1),
            Function::Sub.into(),
            num(2),
            Token::CustomFun(1, Derivative::default()),
        ],
        res(5),
    );
}
#[test]
fn test_composed_functions() {
    let f1 = FunctionVar::new(
        "f",
        NonZeroU8::new(2).unwrap(),
        Tokens(vec![
            Token::InnerVar(0),
            Token::InnerVar(1),
            Function::Sub.into(),
        ]),
        Volatility::GraphConstant,
    );
    let f2 = FunctionVar::new(
        "g",
        NonZeroU8::new(2).unwrap(),
        Tokens(vec![
            Token::InnerVar(0),
            Token::InnerVar(1),
            Function::Mul.into(),
            Token::InnerVar(1),
            Function::Mul.into(),
        ]),
        Volatility::GraphConstant,
    );
    let mut funs = Functions(vec![f1.clone(), f2.clone()]);
    assert_fun("f(n,k)=n-k", "n k f = n k -", &Functions(vec![f1.clone()]));
    assert_fun(
        "g(n,k)=n*k*k",
        "n k g = n k * k *",
        &Functions(vec![f2.clone()]),
    );
    assert_correct_with(
        "g(f(g(2,3)*2,g(3,2)*2)-1,2)",
        "2 3 g 2 * 3 2 g 2 * f 1 - 2 g",
        &mut Variables::default(),
        &[],
        &[],
        &mut funs,
        vec![
            num(2),
            num(3),
            Token::CustomFun(1, Derivative::default()),
            num(2),
            Function::Mul.into(),
            num(3),
            num(2),
            Token::CustomFun(1, Derivative::default()),
            num(2),
            Function::Mul.into(),
            Token::CustomFun(0, Derivative::default()),
            num(1),
            Function::Sub.into(),
            num(2),
            Token::CustomFun(1, Derivative::default()),
        ],
        res(44),
    );
}
#[test]
fn test_custom_functions() {
    let mut funs = Functions(vec![FunctionVar::new(
        "f",
        NonZeroU8::new(2).unwrap(),
        Tokens(vec![
            Token::InnerVar(0),
            Token::InnerVar(1),
            Function::Sub.into(),
        ]),
        Volatility::GraphConstant,
    )]);
    assert_fun("f(n,k)=n-k", "n k f = n k -", &funs);
    assert_correct_with(
        "f(3,4)",
        "3 4 f",
        &mut Variables::default(),
        &[],
        &[],
        &mut funs,
        vec![num(3), num(4), Token::CustomFun(0, Derivative::default())],
        res(-1),
    );
    assert_correct_with(
        "sum(1,2,f(n,4))",
        "1 2 n n 4 f sum",
        &mut Variables::default(),
        &[],
        &[],
        &mut funs,
        vec![
            num(1),
            num(2),
            Token::Skip(3),
            Token::InnerVar(0),
            num(4),
            Token::CustomFun(0, Derivative::default()),
            Function::Sum.into(),
        ],
        res(-5),
    );
    assert_correct_with(
        "sum(0,10,n,sum(3,6,k,f(n,k)^2+f(k,n)-2))",
        "n 0 10 k 3 6 n k f 2 ^ k n f + 2 - sum sum",
        &mut Variables::default(),
        &[],
        &[],
        &mut funs,
        vec![
            num(0),
            num(10),
            Token::Skip(15),
            num(3),
            num(6),
            Token::Skip(11),
            Token::InnerVar(0),
            Token::InnerVar(1),
            Token::CustomFun(0, Derivative::default()),
            num(2),
            Function::Pow.into(),
            Token::InnerVar(1),
            Token::InnerVar(0),
            Token::CustomFun(0, Derivative::default()),
            Function::Add.into(),
            num(2),
            Function::Sub.into(),
            Function::Sum.into(),
            Function::Sum.into(),
        ],
        res(396),
    );
}
#[test]
fn test_sum() {
    assert_correct(
        "sum(0,10,x,x^2)",
        "0 10 x x 2 ^ sum",
        vec![
            num(0),
            num(10),
            Token::Skip(3),
            Token::InnerVar(0),
            num(2),
            Function::Pow.into(),
            Function::Sum.into(),
        ],
        res(385),
    );
    assert_correct(
        "set(2^2,n,sum(1,n,sum(1,n,sum(1,n,sum(1,n,a+b+c+d)))))",
        "2 2 ^ n 1 n d 1 n c 1 n b 1 n a a b + c + d + sum sum sum sum set",
        vec![
            num(2),
            num(2),
            Function::Pow.into(),
            Token::Skip(23),
            num(1),
            Token::InnerVar(0),
            Token::Skip(19),
            num(1),
            Token::InnerVar(0),
            Token::Skip(15),
            num(1),
            Token::InnerVar(0),
            Token::Skip(11),
            num(1),
            Token::InnerVar(0),
            Token::Skip(7),
            Token::InnerVar(4),
            Token::InnerVar(3),
            Function::Add.into(),
            Token::InnerVar(2),
            Function::Add.into(),
            Token::InnerVar(1),
            Function::Add.into(),
            Function::Sum.into(),
            Function::Sum.into(),
            Function::Sum.into(),
            Function::Sum.into(),
            Function::Set.into(),
        ],
        res(2560),
    );
    assert_correct(
        "sum(1,3,a,sum(1,9,3)+a)",
        "1 3 a 1 9 b 3 sum a + sum",
        vec![
            num(1),
            num(3),
            Token::Skip(7),
            num(1),
            num(9),
            Token::Skip(1),
            num(3),
            Function::Sum.into(),
            Token::InnerVar(0),
            Function::Add.into(),
            Function::Sum.into(),
        ],
        res(87),
    );
    assert_correct(
        "sum(0,10,x^2)",
        "0 10 x x 2 ^ sum",
        vec![
            num(0),
            num(10),
            Token::Skip(3),
            Token::InnerVar(0),
            num(2),
            Function::Pow.into(),
            Function::Sum.into(),
        ],
        res(385),
    );
    assert_correct(
        "sum(0,10,sum(n,10,n+k))",
        "0 10 n n 10 k n k + sum sum",
        vec![
            num(0),
            num(10),
            Token::Skip(7),
            Token::InnerVar(0),
            num(10),
            Token::Skip(3),
            Token::InnerVar(0),
            Token::InnerVar(1),
            Function::Add.into(),
            Function::Sum.into(),
            Function::Sum.into(),
        ],
        res(660),
    );
    assert_correct(
        "sum(0,10,sum(0,10,n n+k))",
        "0 10 k 0 10 n n n * k + sum sum",
        vec![
            num(0),
            num(10),
            Token::Skip(9),
            num(0),
            num(10),
            Token::Skip(5),
            Token::InnerVar(1),
            Token::InnerVar(1),
            Function::Mul.into(),
            Token::InnerVar(0),
            Function::Add.into(),
            Function::Sum.into(),
            Function::Sum.into(),
        ],
        res(4840),
    );
    assert_correct(
        "sum(0,10,sq(x))",
        "0 10 x x sq sum",
        vec![
            num(0),
            num(10),
            Token::Skip(2),
            Token::InnerVar(0),
            Function::Sq.into(),
            Function::Sum.into(),
        ],
        res(385),
    );
    assert_correct(
        "sum(0,10,x,pix)",
        "0 10 x pi x * sum",
        vec![
            num(0),
            num(10),
            Token::Skip(3),
            var("pi"),
            Token::InnerVar(0),
            Function::Mul.into(),
            Function::Sum.into(),
        ],
        res(55) * res(Constant::Pi),
    );
}
#[test]
fn test_inner_fn() {
    assert_correct(
        "sum(0,10,x,sum(3,6,y,x-y)+prod(3,6,y,x-y))",
        "0 10 x 3 6 y x y - sum 3 6 y x y - prod + sum",
        vec![
            num(0),
            num(10),
            Token::Skip(15),
            num(3),
            num(6),
            Token::Skip(3),
            Token::InnerVar(0),
            Token::InnerVar(1),
            Function::Sub.into(),
            Function::Sum.into(),
            num(3),
            num(6),
            Token::Skip(3),
            Token::InnerVar(0),
            Token::InnerVar(1),
            Function::Sub.into(),
            Function::Prod.into(),
            Function::Add.into(),
            Function::Sum.into(),
        ],
        res(1870),
    );
}
#[test]
fn test_modify() {
    assert_correct(
        "set(2,x,modify(3,x,x))",
        "2 x 3 x x modify3 set",
        vec![
            num(2),
            Token::Skip(6),
            num(3),
            Token::Skip(1),
            Token::InnerVar(0),
            Token::Skip(1),
            Token::InnerVar(0),
            Function::Modify(ModifyInputs::Three).into(),
            Function::Set.into(),
        ],
        res(3),
    );
    assert_correct(
        "set(2,modify(3,x,x))",
        "2 x 3 x x modify3 set",
        vec![
            num(2),
            Token::Skip(6),
            num(3),
            Token::Skip(1),
            Token::InnerVar(0),
            Token::Skip(1),
            Token::InnerVar(0),
            Function::Modify(ModifyInputs::Three).into(),
            Function::Set.into(),
        ],
        res(3),
    );
    assert_correct(
        "set(2,x,modify(3,x))",
        "2 x 3 x modify set",
        vec![
            num(2),
            Token::Skip(4),
            num(3),
            Token::Skip(1),
            Token::InnerVar(0),
            Function::Modify(ModifyInputs::Two).into(),
            Function::Set.into(),
        ],
        res(0),
    );
    assert_correct(
        "set(2,modify(3,x))",
        "2 x 3 x modify set",
        vec![
            num(2),
            Token::Skip(4),
            num(3),
            Token::Skip(1),
            Token::InnerVar(0),
            Function::Modify(ModifyInputs::Two).into(),
            Function::Set.into(),
        ],
        res(0),
    );
}
#[test]
fn test_while() {
    assert_correct(
        "set(1,while(n<5,modify(n+1,n),2n))",
        "1 n n 5 < n 1 + n modify 2 n * while3 set",
        vec![
            num(1),
            Token::Skip(16),
            Token::Skip(3),
            Token::InnerVar(0),
            num(5),
            Function::Less.into(),
            Token::Skip(6),
            Token::InnerVar(0),
            num(1),
            Function::Add.into(),
            Token::Skip(1),
            Token::InnerVar(0),
            Function::Modify(ModifyInputs::Two).into(),
            Token::Skip(3),
            num(2),
            Token::InnerVar(0),
            Function::Mul.into(),
            Function::While(ModifyInputs::Three).into(),
            Function::Set.into(),
        ],
        res(10),
    );
    assert_correct(
        "set(1,while(n<5,modify(n+1,n,2n)))",
        "1 n n 5 < n 1 + n 2 n * modify3 while2 set",
        vec![
            num(1),
            Token::Skip(16),
            Token::Skip(3),
            Token::InnerVar(0),
            num(5),
            Function::Less.into(),
            Token::Skip(10),
            Token::InnerVar(0),
            num(1),
            Function::Add.into(),
            Token::Skip(1),
            Token::InnerVar(0),
            Token::Skip(3),
            num(2),
            Token::InnerVar(0),
            Function::Mul.into(),
            Function::Modify(ModifyInputs::Three).into(),
            Function::While(ModifyInputs::Two).into(),
            Function::Set.into(),
        ],
        res(10),
    );
}
#[test]
fn test_exprs() {
    assert_correct(
        "set(1,while(n<10,exprs(modify(n+1,n),modify(n+2,n),modify(n+3,n,2n))))",
        "1 n n 10 < n 1 + n modify n 2 + n modify n 3 + n 2 n * modify3 exprs3 while2 set",
        vec![
            num(1),
            Token::Skip(32),
            Token::Skip(3),
            Token::InnerVar(0),
            num(10),
            Function::Less.into(),
            Token::Skip(26),
            Token::Skip(6),
            Token::InnerVar(0),
            num(1),
            Function::Add.into(),
            Token::Skip(1),
            Token::InnerVar(0),
            Function::Modify(ModifyInputs::Two).into(),
            Token::Skip(6),
            Token::InnerVar(0),
            num(2),
            Function::Add.into(),
            Token::Skip(1),
            Token::InnerVar(0),
            Function::Modify(ModifyInputs::Two).into(),
            Token::Skip(10),
            Token::InnerVar(0),
            num(3),
            Function::Add.into(),
            Token::Skip(1),
            Token::InnerVar(0),
            Token::Skip(3),
            num(2),
            Token::InnerVar(0),
            Function::Mul.into(),
            Function::Modify(ModifyInputs::Three).into(),
            Function::Exprs(NonZeroU8::new(3).unwrap()).into(),
            Function::While(ModifyInputs::Two).into(),
            Function::Set.into(),
        ],
        res(26),
    );
}
#[test]
fn test_prod() {
    assert_correct(
        "prod(1,4,x,x^2)",
        "1 4 x x 2 ^ prod",
        vec![
            num(1),
            num(4),
            Token::Skip(3),
            Token::InnerVar(0),
            num(2),
            Function::Pow.into(),
            Function::Prod.into(),
        ],
        res(576),
    );
    assert_correct(
        "prod(1,3,a,prod(1,3,b,a b))",
        "1 3 a 1 3 b a b * prod prod",
        vec![
            num(1),
            num(3),
            Token::Skip(7),
            num(1),
            num(3),
            Token::Skip(3),
            Token::InnerVar(0),
            Token::InnerVar(1),
            Function::Mul.into(),
            Function::Prod.into(),
            Function::Prod.into(),
        ],
        res(46656),
    );
    assert_correct(
        "prod(1,3,sum(1,3,a b))",
        "1 3 b 1 3 a a b * sum prod",
        vec![
            num(1),
            num(3),
            Token::Skip(7),
            num(1),
            num(3),
            Token::Skip(3),
            Token::InnerVar(1),
            Token::InnerVar(0),
            Function::Mul.into(),
            Function::Sum.into(),
            Function::Prod.into(),
        ],
        res(1296),
    );
    assert_correct(
        "prod(1,3,sum(1,3,a)+sum(1,3,b)+c)",
        "1 3 c 1 3 a a sum 1 3 b b sum + c + prod",
        vec![
            num(1),
            num(3),
            Token::Skip(13),
            num(1),
            num(3),
            Token::Skip(1),
            Token::InnerVar(1),
            Function::Sum.into(),
            num(1),
            num(3),
            Token::Skip(1),
            Token::InnerVar(1),
            Function::Sum.into(),
            Function::Add.into(),
            Token::InnerVar(0),
            Function::Add.into(),
            Function::Prod.into(),
        ],
        res(2730),
    );
}
#[test]
fn test_iter() {
    assert_correct(
        "iter(1,4,x,x/2)",
        "1 4 x x 2 / iter",
        vec![
            num(1),
            num(4),
            Token::Skip(3),
            Token::InnerVar(0),
            num(2),
            Function::Div.into(),
            Function::Iter.into(),
        ],
        res(1) / Float::from(16),
    );
    assert_correct(
        "iter(1,0,x,x/2)",
        "1 0 x x 2 / iter",
        vec![
            num(1),
            num(0),
            Token::Skip(3),
            Token::InnerVar(0),
            num(2),
            Function::Div.into(),
            Function::Iter.into(),
        ],
        res(1),
    );
    assert_correct(
        "iter(1,4,x,iter(2,5,y,x/y))",
        "1 4 x 2 5 y x y / iter iter",
        vec![
            num(1),
            num(4),
            Token::Skip(7),
            num(2),
            num(5),
            Token::Skip(3),
            Token::InnerVar(0),
            Token::InnerVar(1),
            Function::Div.into(),
            Function::Iter.into(),
            Function::Iter.into(),
        ],
        res(1) / Float::from(16),
    );
}
#[test]
fn test_cmp() {
    assert_correct(
        "1>=0",
        "1 0 >=",
        vec![num(1), num(0), Function::GreaterEqual.into()],
        res(1),
    );
    assert_correct(
        "1<=0",
        "1 0 <=",
        vec![num(1), num(0), Function::LessEqual.into()],
        res(0),
    );
    assert_correct(
        "1==0",
        "1 0 ==",
        vec![num(1), num(0), Function::Equal.into()],
        res(0),
    );
    assert_correct(
        "1!=0",
        "1 0 !=",
        vec![num(1), num(0), Function::NotEqual.into()],
        res(1),
    );
    assert_correct(
        "1>0",
        "1 0 >",
        vec![num(1), num(0), Function::Greater.into()],
        res(1),
    );
    assert_correct(
        "1<0",
        "1 0 <",
        vec![num(1), num(0), Function::Less.into()],
        res(0),
    );
    assert_correct(
        "1>0&2>1?0>1",
        "1 0 > 2 1 > & 0 1 > ?",
        vec![
            num(1),
            num(0),
            Function::Greater.into(),
            num(2),
            num(1),
            Function::Greater.into(),
            Function::And.into(),
            num(0),
            num(1),
            Function::Greater.into(),
            Function::Or.into(),
        ],
        res(1),
    );
    assert_correct(
        "1&0",
        "1 0 &",
        vec![num(1), num(0), Function::And.into()],
        res(0),
    );
    assert_correct(
        "1?0",
        "1 0 ?",
        vec![num(1), num(0), Function::Or.into()],
        res(1),
    );
    assert_correct(";1", "1 ;", vec![num(1), Function::Not.into()], res(0));
    assert_correct(
        "1==1==1",
        "1 1 1 == ==",
        vec![
            num(1),
            num(1),
            num(1),
            Function::Equal.into(),
            Function::Equal.into(),
        ],
        res(1),
    );
    assert_correct(
        "3!=2!=1",
        "3 2 1 != !=",
        vec![
            num(3),
            num(2),
            num(1),
            Function::NotEqual.into(),
            Function::NotEqual.into(),
        ],
        res(1),
    );
    assert_correct(
        "3>2<4",
        "3 2 4 < >",
        vec![
            num(3),
            num(2),
            num(4),
            Function::Less.into(),
            Function::Greater.into(),
        ],
        res(1),
    );
    assert_correct(
        "3>2>4",
        "3 2 4 > >",
        vec![
            num(3),
            num(2),
            num(4),
            Function::Greater.into(),
            Function::Greater.into(),
        ],
        res(0),
    );
}
#[test]
fn test_tetration() {
    assert_correct(
        "2^^3",
        "2 3 ^^",
        vec![num(2), num(3), Function::Tetration.into()],
        res(16),
    );
}
#[test]
fn test_subfactorial() {
    assert_correct(
        "!4",
        "4 .",
        vec![num(4), Function::SubFactorial.into()],
        res(9),
    );
}
#[test]
fn test_ceil() {
    assert_correct(
        "ceil(4.5)",
        "4.5 ceil",
        vec![num(4.5), Function::Ceil.into()],
        res(5),
    );
    assert_correct(
        "floor(4.5)",
        "4.5 floor",
        vec![num(4.5), Function::Floor.into()],
        res(4),
    );
    assert_correct(
        "round(4.5)",
        "4.5 round",
        vec![num(4.5), Function::Round.into()],
        res(5),
    );
    assert_correct(
        "trunc(4.5)",
        "4.5 trunc",
        vec![num(4.5), Function::Trunc.into()],
        res(4),
    );
    assert_correct(
        "fract(4.5)",
        "4.5 fract",
        vec![num(4.5), Function::Fract.into()],
        res(0.5),
    );
}
#[test]
#[cfg(feature = "complex")]
fn test_real() {
    assert_correct(
        "real(1+2*i)",
        "1 2 i * + real",
        vec![
            num(1),
            num(2),
            var("i"),
            Function::Mul.into(),
            Function::Add.into(),
            Function::Real.into(),
        ],
        res(1),
    );
    assert_correct(
        "imag(1+2*i)",
        "1 2 i * + imag",
        vec![
            num(1),
            num(2),
            var("i"),
            Function::Mul.into(),
            Function::Add.into(),
            Function::Imag.into(),
        ],
        res(2),
    );
}
#[test]
fn test_err() {
    assert_teq(
        Tokens::infix(
            "2.3.4",
            &mut Variables::default(),
            &mut Functions::default(),
            &[],
            false,
            true,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng(),
        ),
        Tokens::rpn(
            "2.3.4",
            &mut Variables::default(),
            &mut Functions::default(),
            &[],
            false,
            true,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng(),
        ),
        Err(ParseError::UnknownToken("2.3.4")),
    );
    assert_eq!(
        Tokens::infix(
            "(2+)",
            &mut Variables::default(),
            &mut Functions::default(),
            &[],
            false,
            true,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng()
        ),
        Err(ParseError::MissingInput)
    );
    assert_eq!(
        Tokens::infix(
            "|(|)",
            &mut Variables::default(),
            &mut Functions::default(),
            &[],
            false,
            true,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng()
        ),
        Err(ParseError::AbsoluteBracketFailed)
    );
    assert_eq!(
        Tokens::infix(
            "(|)|",
            &mut Variables::default(),
            &mut Functions::default(),
            &[],
            false,
            true,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng()
        ),
        Err(ParseError::MissingInput)
    );
    assert_eq!(
        Tokens::infix(
            "(|2)|",
            &mut Variables::default(),
            &mut Functions::default(),
            &[],
            false,
            true,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng()
        ),
        Err(ParseError::AbsoluteBracketFailed)
    );
    assert_eq!(
        Tokens::infix(
            "=2",
            &mut Variables::default(),
            &mut Functions::default(),
            &[],
            true,
            true,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng()
        ),
        Err(ParseError::VarExpectedName)
    );
    assert_eq!(
        Tokens::rpn(
            "= 2",
            &mut Variables::default(),
            &mut Functions::default(),
            &[],
            true,
            true,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng()
        ),
        Err(ParseError::VarExpectedName)
    );
    assert_eq!(
        Tokens::infix(
            "sin'`(2)",
            &mut Variables::default(),
            &mut Functions::default(),
            &[],
            false,
            true,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng()
        ),
        Err(ParseError::MixedError)
    );
    assert_eq!(
        Tokens::infix(
            "sin`'(2)",
            &mut Variables::default(),
            &mut Functions::default(),
            &[],
            false,
            true,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng()
        ),
        Err(ParseError::MixedError)
    );
    assert_eq!(
        Tokens::infix(
            "sin'(2,3)",
            &mut Variables::default(),
            &mut Functions::default(),
            &[],
            false,
            true,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng()
        ),
        Err(ParseError::ExtraInput)
    );
    assert_eq!(
        Tokens::infix(
            "sin(2,3)",
            &mut Variables::default(),
            &mut Functions::default(),
            &[],
            false,
            true,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng()
        ),
        Err(ParseError::ExtraInput)
    );
    assert_eq!(
        Tokens::rpn(
            "=-=",
            &mut Variables::default(),
            &mut Functions::default(),
            &[],
            true,
            true,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng()
        ),
        Err(ParseError::UnknownToken("=-="))
    );
    assert_eq!(
        Tokens::infix(
            "\\",
            &mut Variables::default(),
            &mut Functions::default(),
            &[],
            true,
            true,
            10,
            #[cfg(feature = "float_rand")]
            &mut rng()
        ),
        Err(ParseError::UnknownToken("\\"))
    );
}
#[test]
fn function_exists() {
    for f in [
        Function::Add,
        Function::Sub,
        Function::Mul,
        Function::Div,
        Function::Pow,
        Function::Tetration,
        Function::Root,
        Function::Rem,
        Function::Negate,
        Function::Factorial,
        Function::SubFactorial,
        Function::Equal,
        Function::NotEqual,
        Function::Greater,
        Function::Less,
        Function::GreaterEqual,
        Function::LessEqual,
        Function::And,
        Function::Or,
        Function::Not,
        Function::Sin,
        Function::Asin,
        Function::Cos,
        Function::Acos,
        Function::Tan,
        Function::Sinh,
        Function::Asinh,
        Function::Cosh,
        Function::Acosh,
        Function::Tanh,
        Function::Atanh,
        Function::Ln,
        Function::Exp,
        Function::Atan(AtanInputs::One),
        Function::Atan(AtanInputs::Two),
        Function::Max,
        Function::Min,
        Function::Quadratic,
        #[cfg(feature = "complex")]
        Function::Cubic,
        #[cfg(feature = "complex")]
        Function::Quartic,
        Function::Sqrt,
        Function::Cbrt,
        Function::Sq,
        Function::Cb,
        Function::Sum,
        Function::Prod,
        Function::Gamma,
        #[cfg(feature = "float_rand")]
        Function::RandUniform,
        Function::Erf,
        Function::Erfc,
        Function::Abs,
        #[cfg(feature = "complex")]
        Function::Arg,
        Function::Recip,
        #[cfg(feature = "complex")]
        Function::Conj,
        Function::Iter,
        Function::Ceil,
        Function::Floor,
        Function::Round,
        Function::Trunc,
        Function::Fract,
        #[cfg(feature = "complex")]
        Function::Real,
        #[cfg(feature = "complex")]
        Function::Imag,
        Function::If,
        Function::Fold,
        Function::Set,
        Function::Modify(ModifyInputs::Two),
        Function::Modify(ModifyInputs::Three),
        Function::While(ModifyInputs::Two),
        Function::While(ModifyInputs::Three),
        Function::Exprs(NonZeroU8::new(1).unwrap()),
        Function::Solve,
        Function::NumericalDerivative,
        Function::NumericalDifferential,
        Function::NumericalIntegral,
        Function::NumericalSolve,
    ] {
        assert_eq!(
            std::mem::discriminant(&Function::try_from(f.to_string().as_str()).unwrap()),
            std::mem::discriminant(&f),
            "{f}"
        );
        assert!(
            FUNCTION_LIST
                .iter()
                .any(|l| l.starts_with(format!("{f}(").as_str())),
            "{f}"
        );
        assert_ne!(get_help(f.to_string().as_str()), "unknown", "{f}")
    }
}
