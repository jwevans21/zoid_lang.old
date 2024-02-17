use expect_test::expect_file;
use zoid_lexer::ZoidLexer;

mod common;

init!("../../../examples/basic.zd");

#[test]
fn basic_test() {
    let lexer = ZoidLexer::new(INPUT, FNAME);

    let expected = expect_file!["./basic.tokens"];
    let actual: Vec<_> = lexer.collect();

    expected.assert_debug_eq(&actual);
}
