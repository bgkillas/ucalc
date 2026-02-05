use crate::Pow;
use crate::float::{Complex, Float};
fn res<T>(f: T) -> Complex
where
    Complex: From<T>,
{
    Complex::from(f)
}
fn approx(a: Complex, b: Complex) -> bool {
    (a - b).abs().0 < 1e-4
}
#[test]
fn test_norm() {
    assert_eq!(res((2, 2)).norm(), Float::from(8));
}
#[test]
fn test_abs() {
    assert_eq!(res((2, 2)).abs(), Float::from(2).sqrt() * Float::from(2));
}
#[test]
fn test_sin() {
    assert!(approx(res((2, 2)).sin(), res((3.420954, -1.5093064))));
}
#[test]
fn test_asin() {
    assert!(approx(res((2, 2)).asin(), res((0.754249, 1.734324))));
}
#[test]
fn test_acos() {
    assert!(approx(res((2, 2)).acos(), res((0.8165471, -1.734324))));
}
#[test]
fn test_cos() {
    assert!(approx(res((2, 2)).cos(), res((-1.565625, -3.297894))));
}
#[test]
fn test_ln() {
    assert!(approx(res((2, 2)).ln(), res((1.039720, 0.7853))));
}
#[test]
fn test_exp() {
    assert!(approx(res((2, 2)).exp(), res((-3.07493, 6.718849))));
}
#[test]
fn test_sqrt() {
    assert!(approx(res((2, 2)).sqrt(), res((1.55377, 0.64359))));
}
#[test]
fn test_recip() {
    assert!(approx(res((2, 2)).recip(), res(1) / res((2, 2))));
}
#[test]
fn test_pow() {
    assert!(approx(
        res((2, 3)).pow(res((4, 5))),
        res((-0.75304, -0.98642))
    ));
}
#[test]
fn test_atan2() {
    assert!(approx(
        res((2, 3)).atan2(&res((4, 5))),
        res((0.512, 0.037105))
    ));
}
#[test]
fn test_mul() {
    assert_eq!(res((2, 3)) * res((4, 5)), res((-7, 22)));
}
#[test]
fn test_add() {
    assert_eq!(res((2, 3)) + res((4, 5)), res((6, 8)));
}
#[test]
fn test_sub() {
    assert_eq!(res((2, 3)) - res((4, 5)), res((-2, -2)));
}
#[test]
fn test_div() {
    assert_eq!(res((2, 3)) / res((4, 5)), res((23, 2)) / Float::from(41));
}
#[test]
fn test_rem() {
    assert_eq!(res((5, 6)) % res((4, 5)), res((1, 2)));
}
