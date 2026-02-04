use crate::FunctionVar;
use crate::functions::Function;
use crate::operators::Operators;
use crate::parse::ParseError;
use crate::parse::{Token, Tokens};
use crate::variable::{Functions, Variables};
use ucalc_numbers::{Complex, Constant};

macro_rules! assert_teq {
    ($a:expr, $b:expr, $c:expr) => {
        assert_eq!($a, $b);
        assert_eq!($a, $c);
    };
}
macro_rules! assert_correct {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        assert_correct_with!(
            $a,
            $b,
            Variables::default(),
            &[],
            Functions::default(),
            $c,
            $d
        );
    };
}
macro_rules! assert_correct_with {
    ($a:expr, $b:expr, $v:expr, $vf:expr, $f:expr, $c:expr, $d:expr) => {
        assert_teq!($a, $b, Tokens($c));
        assert_teq!($a.compute($vf, &$f), $b.compute($vf, &$f), $d);
    };
}
fn infix(s: &str) -> Tokens {
    Tokens::infix(s, &Variables::default(), &[], &Functions::default()).unwrap()
}
fn rpn(s: &str) -> Tokens {
    Tokens::rpn(s, &Variables::default(), &[], &Functions::default()).unwrap()
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
fn parse_fact() {
    assert_correct!(
        infix("5!"),
        rpn("5 !"),
        vec![num(5), Operators::Factorial.into()],
        res(120)
    );
    assert_correct!(
        infix("3!^2"),
        rpn("3 ! 2 ^"),
        vec![
            num(3),
            Operators::Factorial.into(),
            num(2),
            Operators::Pow.into()
        ],
        res(36)
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
        res(Constant::E)
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
        (res(Constant::Pi) / 6).cos()
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
        (res(Constant::Pi) / 6).sin()
    );
}
#[test]
fn parse_sinh() {
    assert_correct!(
        infix("2*sinh(1)"),
        rpn("2 1 sinh *"),
        vec![num(2), num(1), Function::Sinh.into(), Operators::Mul.into(),],
        res(Constant::E) - res(Constant::E).recip()
    );
    assert_correct!(
        infix("asinh((e-1/e)/2)"),
        rpn("e 1 e / - 2 / asinh"),
        vec![
            num(Constant::E),
            num(1),
            num(Constant::E),
            Operators::Div.into(),
            Operators::Sub.into(),
            num(2),
            Operators::Div.into(),
            Function::Asinh.into(),
        ],
        res(1)
    );
}
#[test]
fn parse_cosh() {
    assert_correct!(
        infix("2*cosh(1)"),
        rpn("2 1 cosh *"),
        vec![num(2), num(1), Function::Cosh.into(), Operators::Mul.into(),],
        res(Constant::E) + res(Constant::E).recip()
    );
    assert_correct!(
        infix("acosh((e+1/e)/2)"),
        rpn("e 1 e / + 2 / acosh"),
        vec![
            num(Constant::E),
            num(1),
            num(Constant::E),
            Operators::Div.into(),
            Operators::Add.into(),
            num(2),
            Operators::Div.into(),
            Function::Acosh.into(),
        ],
        res(1)
    );
}
#[test]
fn parse_tanh() {
    assert_correct!(
        infix("tanh(1)"),
        rpn("1 tanh"),
        vec![num(1), Function::Tanh.into(),],
        res(1).sinh() / res(1).cosh()
    );
    assert_correct!(
        infix("atanh(1)"),
        rpn("1 atanh"),
        vec![num(1), Function::Atanh.into(),],
        res(1).atanh()
    );
}
#[test]
fn parse_tan() {
    assert_correct!(
        infix("tan(pi/6)"),
        rpn("pi 6 / tan"),
        vec![
            num(Constant::Pi),
            num(6),
            Operators::Div.into(),
            Function::Tan.into()
        ],
        (res(Constant::Pi) / 6).tan()
    );
}
#[test]
fn parse_sqrt() {
    assert_correct!(
        infix("sqrt(4)"),
        rpn("4 sqrt"),
        vec![num(4), Function::Sqrt.into()],
        res(2)
    );
    assert_correct!(
        infix("sq(4)"),
        rpn("4 sq"),
        vec![num(4), Function::Sq.into()],
        res(16)
    );
}
#[test]
fn parse_cbrt() {
    assert_correct!(
        infix("cbrt(8)"),
        rpn("8 cbrt"),
        vec![num(8), Function::Cbrt.into()],
        res(2)
    );
    assert_correct!(
        infix("cb(2)"),
        rpn("2 cb"),
        vec![num(2), Function::Cb.into()],
        res(8)
    );
}
#[test]
fn parse_gamma() {
    assert_correct!(
        infix("gamma(4)"),
        rpn("4 gamma"),
        vec![num(4), Function::Gamma.into()],
        res(6)
    );
}
#[test]
fn parse_erf() {
    assert_correct!(
        infix("erf(100)"),
        rpn("100 erf"),
        vec![num(100), Function::Erf.into()],
        res(1)
    );
}
#[test]
fn parse_erfc() {
    assert_correct!(
        infix("erfc(100)"),
        rpn("100 erfc"),
        vec![num(100), Function::Erfc.into()],
        res(0)
    );
}
#[test]
fn parse_abs() {
    assert_correct!(
        infix("abs(2+2*i)"),
        rpn("2 2 i * + abs"),
        vec![
            num(2),
            num(2),
            num((0, 1)),
            Operators::Mul.into(),
            Operators::Add.into(),
            Function::Abs.into()
        ],
        res(8).sqrt()
    );
    assert_correct!(
        infix("|1|"),
        rpn("1 abs"),
        vec![num(1), Function::Abs.into(),],
        res(1)
    );
    assert_correct!(
        infix("||0|-0|"),
        rpn("0 abs 0 - abs"),
        vec![
            num(0),
            Function::Abs.into(),
            num(0),
            Operators::Sub.into(),
            Function::Abs.into()
        ],
        res(0)
    );
    assert_correct!(
        infix("||0!|-|0^1*|0|-|-2|||+|0|"),
        rpn("0 ! abs 0 1 ^ 0 abs * 2 _ abs - abs - abs 0 abs +"),
        vec![
            num(0),
            Operators::Factorial.into(),
            Function::Abs.into(),
            num(0),
            num(1),
            Operators::Pow.into(),
            num(0),
            Function::Abs.into(),
            Operators::Mul.into(),
            num(2),
            Operators::Negate.into(),
            Function::Abs.into(),
            Operators::Sub.into(),
            Function::Abs.into(),
            Operators::Sub.into(),
            Function::Abs.into(),
            num(0),
            Function::Abs.into(),
            Operators::Add.into(),
        ],
        res(1)
    );
}
#[test]
fn parse_arg() {
    assert_correct!(
        infix("arg(2+2*i)"),
        rpn("2 2 i * + arg"),
        vec![
            num(2),
            num(2),
            num((0, 1)),
            Operators::Mul.into(),
            Operators::Add.into(),
            Function::Arg.into()
        ],
        res(Constant::Pi) / 4
    );
}
#[test]
fn parse_conj() {
    assert_correct!(
        infix("conj(2+2*i)"),
        rpn("2 2 i * + conj"),
        vec![
            num(2),
            num(2),
            num((0, 1)),
            Operators::Mul.into(),
            Operators::Add.into(),
            Function::Conj.into()
        ],
        res((2, -2))
    );
}
#[test]
fn parse_recip() {
    assert_correct!(
        infix("recip(2)"),
        rpn("2 recip"),
        vec![num(2), Function::Recip.into()],
        res(0.5)
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
        vec![num(1), num(1), Function::Atan2.into()],
        res(Constant::Pi) / 4
    );
}
#[test]
fn parse_arctan() {
    assert_correct!(
        infix("arctan(1)"),
        rpn("1 arctan"),
        vec![num(1), Function::Atan.into()],
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
        res(2).sqrt() - 1
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
        res(2).sqrt() - 1
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
        res(Constant::Pi).sin()
    );
}
#[test]
fn test_graph_vars() {
    let infix = Tokens::infix(
        "x^y",
        &Variables::default(),
        &["x", "y"],
        &Functions::default(),
    )
    .unwrap();
    let rpn = Tokens::rpn(
        "x y ^",
        &Variables::default(),
        &["x", "y"],
        &Functions::default(),
    )
    .unwrap();
    assert_correct_with!(
        infix,
        rpn,
        vars,
        &[Complex::from(2), Complex::from(3)],
        Functions::default(),
        vec![
            Token::GraphVar(0),
            Token::GraphVar(1),
            Operators::Pow.into()
        ],
        res(8)
    );
    assert_correct_with!(
        infix,
        rpn,
        vars,
        &[Complex::from(3), Complex::from(2)],
        Functions::default(),
        vec![
            Token::GraphVar(0),
            Token::GraphVar(1),
            Operators::Pow.into()
        ],
        res(9)
    );
}
#[test]
fn test_set() {
    assert_correct!(
        infix("set(x,2,x^2)"),
        rpn("x 2 x 2 ^ set"),
        vec![
            num(2),
            Token::Skip(3),
            Token::InnerVar(0).into(),
            num(2),
            Operators::Pow.into(),
            Function::Set.into()
        ],
        res(4)
    );
}
#[test]
fn test_solve() {
    assert_correct!(
        infix("solve(x,x^2-2)"),
        rpn("x x 2 ^ 2 - solve"),
        vec![
            Token::Skip(5),
            Token::InnerVar(0).into(),
            num(2),
            Operators::Pow.into(),
            num(2),
            Operators::Sub.into(),
            Function::Solve.into()
        ],
        res(2).sqrt()
    );
    assert_correct!(
        infix("solve(x,2*x-1)"),
        rpn("x 2 x * 1 - solve"),
        vec![
            Token::Skip(5),
            num(2),
            Token::InnerVar(0).into(),
            Operators::Mul.into(),
            num(1),
            Operators::Sub.into(),
            Function::Solve.into()
        ],
        res(0.5)
    );
    assert_correct!(
        infix("solve(x,x*x-2*x-1)"),
        rpn("x x x * 2 x * - 1 - solve"),
        vec![
            Token::Skip(9),
            Token::InnerVar(0).into(),
            Token::InnerVar(0).into(),
            Operators::Mul.into(),
            num(2),
            Token::InnerVar(0).into(),
            Operators::Mul.into(),
            Operators::Sub.into(),
            num(1),
            Operators::Sub.into(),
            Function::Solve.into()
        ],
        -(res(2).sqrt() - 1)
    );
}
#[test]
fn test_fold() {
    assert_correct!(
        infix("fold(x,k,1,1,9,x*k)"),
        rpn("x k 1 1 9 x k * fold"),
        vec![
            num(1),
            num(1),
            num(9),
            Token::Skip(3),
            Token::InnerVar(0).into(),
            Token::InnerVar(1).into(),
            Operators::Mul.into(),
            Function::Fold.into()
        ],
        res(362880)
    );
    assert_correct!(
        infix("fold(x,k,0,1,9,x+k)"),
        rpn("x k 0 1 9 x k + fold"),
        vec![
            num(0),
            num(1),
            num(9),
            Token::Skip(3),
            Token::InnerVar(0).into(),
            Token::InnerVar(1).into(),
            Operators::Add.into(),
            Function::Fold.into()
        ],
        res(45)
    );
}
#[test]
fn test_if() {
    assert_correct!(
        infix("if(1,2,3)"),
        rpn("1 2 3 if"),
        vec![
            num(1),
            Token::Skip(1),
            num(2),
            Token::Skip(1),
            num(3),
            Function::If.into()
        ],
        res(2)
    );
    assert_correct!(
        infix("if(0,2,3)"),
        rpn("0 2 3 if"),
        vec![
            num(0),
            Token::Skip(1),
            num(2),
            Token::Skip(1),
            num(3),
            Function::If.into()
        ],
        res(3)
    );
}
#[test]
fn test_recursion() {
    let funs = Functions(vec![FunctionVar::new(
        "fact",
        1,
        Tokens(vec![
            Token::InnerVar(0),
            num(0),
            Operators::Greater.into(),
            Token::Skip(6),
            Token::InnerVar(0),
            Token::InnerVar(0),
            num(1),
            Operators::Sub.into(),
            Token::Fun(0),
            Operators::Mul.into(),
            Token::Skip(1),
            num(1),
            Function::If.into(),
        ]),
    )]);
    assert_correct_with!(
        Tokens::infix("fact(5)", &Variables::default(), &[], &funs).unwrap(),
        Tokens::rpn("5 fact", &Variables::default(), &[], &funs).unwrap(),
        Variables::default(),
        &[],
        funs,
        vec![num(5), Token::Fun(0)],
        res(120)
    );
}
#[test]
fn test_composed_functions() {
    let funs = Functions(vec![
        FunctionVar::new(
            "f",
            2,
            Tokens(vec![
                Token::InnerVar(0),
                Token::InnerVar(1),
                Operators::Sub.into(),
            ]),
        ),
        FunctionVar::new(
            "g",
            2,
            Tokens(vec![
                Token::InnerVar(0),
                Token::InnerVar(1),
                Operators::Mul.into(),
                Token::InnerVar(1),
                Operators::Mul.into(),
            ]),
        ),
    ]);
    assert_correct_with!(
        Tokens::infix(
            "g(f(g(2,3)*2,g(3,2)*2)-1,2)",
            &Variables::default(),
            &[],
            &funs
        )
        .unwrap(),
        Tokens::rpn(
            "2 3 g 2 * 3 2 g 2 * f 1 - 2 g",
            &Variables::default(),
            &[],
            &funs
        )
        .unwrap(),
        Variables::default(),
        &[],
        funs,
        vec![
            num(2),
            num(3),
            Token::Fun(1),
            num(2),
            Operators::Mul.into(),
            num(3),
            num(2),
            Token::Fun(1),
            num(2),
            Operators::Mul.into(),
            Token::Fun(0),
            num(1),
            Operators::Sub.into(),
            num(2),
            Token::Fun(1)
        ],
        res(44)
    );
}
#[test]
fn test_custom_functions() {
    let funs = Functions(vec![FunctionVar::new(
        "f",
        2,
        Tokens(vec![
            Token::InnerVar(0),
            Token::InnerVar(1),
            Operators::Sub.into(),
        ]),
    )]);
    assert_correct_with!(
        Tokens::infix("f(3,4)", &Variables::default(), &[], &funs).unwrap(),
        Tokens::rpn("3 4 f", &Variables::default(), &[], &funs).unwrap(),
        Variables::default(),
        &[],
        funs,
        vec![num(3), num(4), Token::Fun(0)],
        res(-1)
    );
    assert_correct_with!(
        Tokens::infix(
            "sum(n,0,10,sum(k,3,6,f(n,k)^2+f(k,n)-2))",
            &Variables::default(),
            &[],
            &funs
        )
        .unwrap(),
        Tokens::rpn(
            "n 0 10 k 3 6 n k f 2 ^ k n f + 2 - sum sum",
            &Variables::default(),
            &[],
            &funs
        )
        .unwrap(),
        Variables::default(),
        &[],
        funs,
        vec![
            num(0),
            num(10),
            Token::Skip(15),
            num(3),
            num(6),
            Token::Skip(11),
            Token::InnerVar(0),
            Token::InnerVar(1),
            Token::Fun(0),
            num(2),
            Operators::Pow.into(),
            Token::InnerVar(1),
            Token::InnerVar(0),
            Token::Fun(0),
            Operators::Add.into(),
            num(2),
            Operators::Sub.into(),
            Function::Sum.into(),
            Function::Sum.into()
        ],
        res(396)
    );
}
#[test]
fn test_sum() {
    assert_correct!(
        infix("sum(x,0,10,x^2)"),
        rpn("x 0 10 x 2 ^ sum"),
        vec![
            num(0),
            num(10),
            Token::Skip(3),
            Token::InnerVar(0),
            num(2),
            Operators::Pow.into(),
            Function::Sum.into()
        ],
        res(385)
    );
}
#[test]
fn test_inner_fn() {
    assert_correct!(
        infix("sum(x,0,10,sum(y,3,6,x-y)+prod(y,3,6,x-y))"),
        rpn("x 0 10 y 3 6 x y - sum y 3 6 x y - prod + sum"),
        vec![
            num(0),
            num(10),
            Token::Skip(15),
            num(3),
            num(6),
            Token::Skip(3),
            Token::InnerVar(0),
            Token::InnerVar(1),
            Operators::Sub.into(),
            Function::Sum.into(),
            num(3),
            num(6),
            Token::Skip(3),
            Token::InnerVar(0),
            Token::InnerVar(1),
            Operators::Sub.into(),
            Function::Prod.into(),
            Operators::Add.into(),
            Function::Sum.into()
        ],
        res(1870)
    );
}
#[test]
fn test_prod() {
    assert_correct!(
        infix("prod(x,1,4,x^2)"),
        rpn("x 1 4 x 2 ^ prod"),
        vec![
            num(1),
            num(4),
            Token::Skip(3),
            Token::InnerVar(0),
            num(2),
            Operators::Pow.into(),
            Function::Prod.into()
        ],
        res(576)
    );
}
#[test]
fn test_iter() {
    assert_correct!(
        infix("iter(x,1,4,x/2)"),
        rpn("x 1 4 x 2 / iter"),
        vec![
            num(1),
            num(4),
            Token::Skip(3),
            Token::InnerVar(0),
            num(2),
            Operators::Div.into(),
            Function::Iter.into()
        ],
        res(1) / 16
    );
    assert_correct!(
        infix("iter(x,1,0,x/2)"),
        rpn("x 1 0 x 2 / iter"),
        vec![
            num(1),
            num(0),
            Token::Skip(3),
            Token::InnerVar(0),
            num(2),
            Operators::Div.into(),
            Function::Iter.into()
        ],
        res(1)
    );
    assert_correct!(
        infix("iter(x,1,4,iter(y,2,5,x/y))"),
        rpn("x 1 4 y 2 5 x y / iter iter"),
        vec![
            num(1),
            num(4),
            Token::Skip(7),
            num(2),
            num(5),
            Token::Skip(3),
            Token::InnerVar(0),
            Token::InnerVar(1),
            Operators::Div.into(),
            Function::Iter.into(),
            Function::Iter.into()
        ],
        res(1) / 16
    );
}
#[test]
fn test_cmp() {
    assert_correct!(
        infix("1>=0"),
        rpn("1 0 >="),
        vec![num(1), num(0), Operators::GreaterEqual.into()],
        res(1)
    );
    assert_correct!(
        infix("1<=0"),
        rpn("1 0 <="),
        vec![num(1), num(0), Operators::LessEqual.into()],
        res(0)
    );
    assert_correct!(
        infix("1==0"),
        rpn("1 0 =="),
        vec![num(1), num(0), Operators::Equal.into()],
        res(0)
    );
    assert_correct!(
        infix("1!=0"),
        rpn("1 0 !="),
        vec![num(1), num(0), Operators::NotEqual.into()],
        res(1)
    );
    assert_correct!(
        infix("1>0"),
        rpn("1 0 >"),
        vec![num(1), num(0), Operators::Greater.into()],
        res(1)
    );
    assert_correct!(
        infix("1<0"),
        rpn("1 0 <"),
        vec![num(1), num(0), Operators::Less.into()],
        res(0)
    );
    assert_correct!(
        infix("1>0&2>1?0>1"),
        rpn("1 0 > 2 1 > & 0 1 > ?"),
        vec![
            num(1),
            num(0),
            Operators::Greater.into(),
            num(2),
            num(1),
            Operators::Greater.into(),
            Operators::And.into(),
            num(0),
            num(1),
            Operators::Greater.into(),
            Operators::Or.into()
        ],
        res(1)
    );
    assert_correct!(
        infix("1&0"),
        rpn("1 0 &"),
        vec![num(1), num(0), Operators::And.into()],
        res(0)
    );
    assert_correct!(
        infix("1?0"),
        rpn("1 0 ?"),
        vec![num(1), num(0), Operators::Or.into()],
        res(1)
    );
    assert_correct!(
        infix("'1"),
        rpn("1 '"),
        vec![num(1), Operators::Not.into()],
        res(0)
    );
    assert_correct!(
        infix("1==1==1"),
        rpn("1 1 1 == =="),
        vec![
            num(1),
            num(1),
            num(1),
            Operators::Equal.into(),
            Operators::Equal.into()
        ],
        res(1)
    );
    assert_correct!(
        infix("3!=2!=1"),
        rpn("3 2 1 != !="),
        vec![
            num(3),
            num(2),
            num(1),
            Operators::NotEqual.into(),
            Operators::NotEqual.into()
        ],
        res(1)
    );
    assert_correct!(
        infix("3>2<4"),
        rpn("3 2 4 < >"),
        vec![
            num(3),
            num(2),
            num(4),
            Operators::Less.into(),
            Operators::Greater.into()
        ],
        res(1)
    );
    assert_correct!(
        infix("3>2>4"),
        rpn("3 2 4 > >"),
        vec![
            num(3),
            num(2),
            num(4),
            Operators::Greater.into(),
            Operators::Greater.into()
        ],
        res(0)
    );
}
#[test]
fn test_tetration() {
    assert_correct!(
        infix("2^^3"),
        rpn("2 3 ^^"),
        vec![num(2), num(3), Operators::Tetration.into()],
        res(16)
    );
}
#[test]
fn test_subfactorial() {
    assert_correct!(
        infix("!4"),
        rpn("4 ."),
        vec![num(4), Operators::SubFactorial.into()],
        res(9)
    );
}
#[test]
fn test_ceil() {
    assert_correct!(
        infix("ceil(4.5)"),
        rpn("4.5 ceil"),
        vec![num(4.5), Function::Ceil.into()],
        res(5)
    );
    assert_correct!(
        infix("floor(4.5)"),
        rpn("4.5 floor"),
        vec![num(4.5), Function::Floor.into()],
        res(4)
    );
    assert_correct!(
        infix("round(4.5)"),
        rpn("4.5 round"),
        vec![num(4.5), Function::Round.into()],
        res(5)
    );
    assert_correct!(
        infix("trunc(4.5)"),
        rpn("4.5 trunc"),
        vec![num(4.5), Function::Trunc.into()],
        res(4)
    );
    assert_correct!(
        infix("fract(4.5)"),
        rpn("4.5 fract"),
        vec![num(4.5), Function::Fract.into()],
        res(0.5)
    );
}
#[test]
fn test_real() {
    assert_correct!(
        infix("real(1+2*i)"),
        rpn("1 2 i * + real"),
        vec![
            num(1),
            num(2),
            num((0, 1)),
            Operators::Mul.into(),
            Operators::Add.into(),
            Function::Real.into()
        ],
        res(1)
    );
    assert_correct!(
        infix("imag(1+2*i)"),
        rpn("1 2 i * + imag"),
        vec![
            num(1),
            num(2),
            num((0, 1)),
            Operators::Mul.into(),
            Operators::Add.into(),
            Function::Imag.into()
        ],
        res(2)
    );
}
#[test]
fn test_err() {
    assert_eq!(
        Tokens::infix("(2+3))", &Variables::default(), &[], &Functions::default()),
        Err(ParseError::LeftParenthesisNotFound)
    );
    assert_eq!(
        Tokens::infix("((2+3)", &Variables::default(), &[], &Functions::default()),
        Err(ParseError::RightParenthesisNotFound)
    );
    assert_teq!(
        Tokens::infix("2.3.4", &Variables::default(), &[], &Functions::default()),
        Tokens::rpn("2.3.4", &Variables::default(), &[], &Functions::default()),
        Err(ParseError::UnknownToken("2.3.4".to_string()))
    );
    assert_eq!(
        Tokens::infix("(2+)", &Variables::default(), &[], &Functions::default()),
        Err(ParseError::MissingInput)
    );
    assert_eq!(
        Tokens::infix("|2", &Variables::default(), &[], &Functions::default()),
        Err(ParseError::AbsoluteBracketFailed)
    );
    assert_eq!(
        Tokens::infix("|(|)", &Variables::default(), &[], &Functions::default()),
        Err(ParseError::AbsoluteBracketFailed)
    );
    assert_eq!(
        Tokens::infix("(|)|", &Variables::default(), &[], &Functions::default()),
        Err(ParseError::LeftParenthesisNotFound)
    );
    /*assert_teq!(
        Tokens::infix("abc(2)", Variables::default(), Functions::default()),
        Tokens::rpn("2 abc", Variables::default(), Functions::default()),
        Err(ParseError::UnknownToken("abc".to_string()))
    );*/
}
