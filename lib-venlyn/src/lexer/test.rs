use color_print::cprintln;

use crate::lexer::{self, token::Token as T};

struct LexTest<'a> {
    code: &'a str,
    tokens: Vec<T>,
}

impl<'a> LexTest<'a> {
    fn new(code: &'a str, tokens: impl Into<Vec<T>>) -> Self {
        LexTest {
            code: code,
            tokens: tokens.into(),
        }
    }
}

#[test]
pub fn weird_lex_tests() {
    let tests = [LexTest::new(
        "let this=4;let that = 5; let somet hing      = 1 == 4;",
        [
            T::Let,
            T::Id("this".into()),
            T::Eq,
            T::NumericalLiteral(4),
            T::Semi,
            T::Let,
            T::Id("that".into()),
            T::Eq,
            T::NumericalLiteral(5),
            T::Semi,
            T::Let,
            T::Id("somet".into()),
            T::Id("hing".into()),
            T::Eq,
            T::NumericalLiteral(1),
            T::CmpEq,
            T::NumericalLiteral(4),
            T::Semi,
        ],
    )];

    let results: Vec<Result<(), (&LexTest, Vec<T>)>> = tests.iter().map(run_test).collect();

    results.iter().for_each(|res| {
            println!();
            match res {
                Ok(()) => cprintln!("<green>SUCESSFUL LEX TEST</green>"),
                Err(err) => cprintln!("<red>FAILED TO LEX:</red> <green>\"{}\"</green>\n<red>GOT OUTPUT:</red> {:?}\n<red>EXPECTED OUTPUT:</red> {:?}", err.0.code, err.0.tokens, err.1)
            }
        });

    assert!(results.iter().all(Result::is_ok));
}
fn run_test<'a>(test: &'a LexTest) -> Result<(), (&'a LexTest<'a>, Vec<T>)> {
    let tokens = lexer::lex(test.code);
    match tokens == test.tokens.to_vec() {
        true => Ok(()),
        false => Err((test, tokens)),
    }
}
