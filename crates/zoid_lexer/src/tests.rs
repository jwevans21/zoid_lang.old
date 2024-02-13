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

    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(0..1),
            kind: ZoidTokenKind::LParen,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(1..2).new_col(2),
            kind: ZoidTokenKind::RParen,
        }))
    );

    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(2..3).new_col(3),
            kind: ZoidTokenKind::LBrace,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(3..4).new_col(4),
            kind: ZoidTokenKind::RBrace,
        }))
    );

    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(4..5).new_col(5),
            kind: ZoidTokenKind::LBracket,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(5..6).new_col(6),
            kind: ZoidTokenKind::RBracket,
        }))
    );

    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(6..8).new_col(7),
            kind: ZoidTokenKind::LGenericBracket,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(8..10).new_col(9),
            kind: ZoidTokenKind::RGenericBracket,
        }))
    );

    assert_eq!(lexer.tokenize(), Ok(None));
}

#[test]
fn special() {
    let fname = "test_special";
    let mut lexer = ZoidLexer::new(":,;", fname);

    let base_loc = ZoidLocation {
        file_name: fname,
        line: 1,
        column: 1,
        start: 0,
        end: 0,
    };

    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(0..1),
            kind: ZoidTokenKind::Colon,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(1..2).new_col(2),
            kind: ZoidTokenKind::Comma,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(2..3).new_col(3),
            kind: ZoidTokenKind::Semicolon,
        }))
    );
    assert_eq!(lexer.tokenize(), Ok(None));
}

#[test]
fn operators() {
    let fname = "test_operators";
    let input = "<= < >= > = == ! != & && | || ~ + - * / % << >>\n\
                         .& .* .? ..< ..= .. ... .";

    let mut lexer = ZoidLexer::new(input, fname);
    let base_loc = ZoidLocation {
        file_name: fname,
        line: 1,
        column: 1,
        start: 0,
        end: 0,
    };

    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(0..2),
            kind: ZoidTokenKind::OpLeq,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(3..4).new_col(4),
            kind: ZoidTokenKind::OpLt,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(5..7).new_col(6),
            kind: ZoidTokenKind::OpGeq,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(8..9).new_col(9),
            kind: ZoidTokenKind::OpGt,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(10..11).new_col(11),
            kind: ZoidTokenKind::OpAssign,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(12..14).new_col(13),
            kind: ZoidTokenKind::OpEq,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(15..16).new_col(16),
            kind: ZoidTokenKind::OpNot,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(17..19).new_col(18),
            kind: ZoidTokenKind::OpNe,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(20..21).new_col(21),
            kind: ZoidTokenKind::OpBitAnd,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(22..24).new_col(23),
            kind: ZoidTokenKind::OpAnd,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(25..26).new_col(26),
            kind: ZoidTokenKind::OpBitOr,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(27..29).new_col(28),
            kind: ZoidTokenKind::OpOr,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(30..31).new_col(31),
            kind: ZoidTokenKind::OpBitNot,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(32..33).new_col(33),
            kind: ZoidTokenKind::OpAdd,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(34..35).new_col(35),
            kind: ZoidTokenKind::OpSub,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(36..37).new_col(37),
            kind: ZoidTokenKind::OpMul,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(38..39).new_col(39),
            kind: ZoidTokenKind::OpDiv,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(40..41).new_col(41),
            kind: ZoidTokenKind::OpRem,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(42..44).new_col(43),
            kind: ZoidTokenKind::OpShl,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(45..47).new_col(46),
            kind: ZoidTokenKind::OpShr,
        }))
    );

    let base_loc = base_loc.new_line(2).new_col(1);

    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(48..50),
            kind: ZoidTokenKind::OpAddr,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(51..53).new_col(4),
            kind: ZoidTokenKind::OpDeref,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(54..56).new_col(7),
            kind: ZoidTokenKind::OpUnwrap,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(57..60).new_col(10),
            kind: ZoidTokenKind::OpRangeExclusive,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(61..64).new_col(14),
            kind: ZoidTokenKind::OpRangeInclusive,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(65..67).new_col(18),
            kind: ZoidTokenKind::OpRangeExclusive,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(68..71).new_col(21),
            kind: ZoidTokenKind::VaArgs,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(72..73).new_col(25),
            kind: ZoidTokenKind::OpDot,
        }))
    );

    assert_eq!(lexer.tokenize(), Ok(None));
}

#[test]
fn identifiers() {
    let fname = "test_identifiers";
    let input = "a abc _a __ _ a_090_";

    let mut lexer = ZoidLexer::new(input, fname);
    let base_loc = ZoidLocation {
        file_name: fname,
        line: 1,
        column: 1,
        start: 0,
        end: 0,
    };

    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(0..1),
            kind: ZoidTokenKind::Identifier,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(2..5).new_col(3),
            kind: ZoidTokenKind::Identifier,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(6..8).new_col(7),
            kind: ZoidTokenKind::Identifier,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(9..11).new_col(10),
            kind: ZoidTokenKind::Identifier,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(12..13).new_col(13),
            kind: ZoidTokenKind::Identifier,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_range(14..20).new_col(15),
            kind: ZoidTokenKind::Identifier,
        }))
    );
    assert_eq!(lexer.tokenize(), Ok(None));
}
