#![no_main]
use libfuzzer_sys::fuzz_target;
use ucalc_lib::*;
fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let vars = Variables::default();
        let mut funs = Functions::default();
        let tokens = Tokens::infix(s, &vars, &[], &funs).unwrap();
        tokens.compute(&[], &funs);
        let tokens = Tokens::rpn(s, &vars, &[], &funs).unwrap();
        tokens.compute(&[], &funs);
        let tokens = Tokens::infix(s, &vars, &["x", "y", "z"], &funs).unwrap();
        tokens.compute(
            &[Complex::from(0), Complex::from(1), Complex::from(2)],
            &funs,
        );
        let tokens = Tokens::rpn(s, &vars, &["x", "y", "z"], &funs).unwrap();
        tokens.compute(
            &[Complex::from(0), Complex::from(1), Complex::from(2)],
            &funs,
        );
        funs.push(FunctionVar::new(
            "f",
            3,
            Tokens(vec![
                Token::InnerVar(0),
                Token::InnerVar(1),
                Operators::Add.into(),
                Token::InnerVar(2),
                Operators::Add.into(),
            ]),
        ));
        funs.push(FunctionVar::new("g", 1, Tokens(vec![Token::InnerVar(0)])));
        let tokens = Tokens::infix(s, &vars, &[], &funs).unwrap();
        tokens.compute(&[], &funs);
        let tokens = Tokens::rpn(s, &vars, &[], &funs).unwrap();
        tokens.compute(&[], &funs);
        let tokens = Tokens::infix(s, &vars, &["x", "y", "z"], &funs).unwrap();
        tokens.compute(
            &[Complex::from(0), Complex::from(1), Complex::from(2)],
            &funs,
        );
        let tokens = Tokens::rpn(s, &vars, &["x", "y", "z"], &funs).unwrap();
        tokens.compute(
            &[Complex::from(0), Complex::from(1), Complex::from(2)],
            &funs,
        );
    }
});
