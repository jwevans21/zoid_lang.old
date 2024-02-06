#![cfg(test)]

use super::*;

#[test]
fn brackets() {
    let fname = "test_brackets";
    let mut lexer = ZoidLexer::new("(){}[]<::>", fname);

    let base_loc = ZoidLocation {
        file_name: fname,
        line: 1,
        column: 1,
        start: 0,
        end: 0,
    };


    assert_eq!(lexer.tokenize(), Ok(Some(ZoidToken {
        location: base_loc.new_range(0..1),
        kind: ZoidTokenKind::LParen,
    })));
    assert_eq!(lexer.tokenize(), Ok(Some(ZoidToken {
        location: base_loc.new_range(1..2).new_col(2),
        kind: ZoidTokenKind::RParen,
    })));

    assert_eq!(lexer.tokenize(), Ok(Some(ZoidToken {
        location: base_loc.new_range(2..3).new_col(3),
        kind: ZoidTokenKind::LBrace,
    })));
    assert_eq!(lexer.tokenize(), Ok(Some(ZoidToken {
        location: base_loc.new_range(3..4).new_col(4),
        kind: ZoidTokenKind::RBrace,
    })));

    assert_eq!(lexer.tokenize(), Ok(Some(ZoidToken {
        location: base_loc.new_range(4..5).new_col(5),
        kind: ZoidTokenKind::LBracket,
    })));
    assert_eq!(lexer.tokenize(), Ok(Some(ZoidToken {
        location: base_loc.new_range(5..6).new_col(6),
        kind: ZoidTokenKind::RBracket,
    })));

    assert_eq!(lexer.tokenize(), Ok(Some(ZoidToken {
        location: base_loc.new_range(6..8).new_col(7),
        kind: ZoidTokenKind::LGenericBracket,
    })));
    assert_eq!(lexer.tokenize(), Ok(Some(ZoidToken {
        location: base_loc.new_range(8..10).new_col(9),
        kind: ZoidTokenKind::RGenericBracket,
    })));

    assert_eq!(lexer.tokenize(), Ok(None));
}