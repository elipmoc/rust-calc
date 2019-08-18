pub mod ast;
pub mod parser;

use nom::Err;
use nom::error::ErrorKind;

///受け取った文字列をパースして評価する
pub fn expr_eval(s: &str) -> Result<i32, Err<(&str, ErrorKind)>> {
    parser::expr_parser(s)
        .map(|(_, expr)| expr.eval())
}

#[test]
fn expr_eval_test() {
    assert_eq!(
        expr_eval("1+2+3+4+5").unwrap(),
        1 + 2 + 3 + 4 + 5
    );

    assert_eq!(
        expr_eval("1+2*3-7").unwrap(),
        1 + 2 * 3 - 7
    );

    assert_eq!(
        expr_eval("(2*24)/(5+3)").unwrap(),
        (2 * 24) / (5 + 3)
    );
}