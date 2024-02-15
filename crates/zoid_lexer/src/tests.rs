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

#[test]
fn keywords() {
    let fname = "test_keywords";
    let input = r#"if
else
fn
let
return
for
while
break
continue
in
struct
enum
union
impl
trait
where
async
await
gen
yield
import
importc

and
or
not"#;

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
            kind: ZoidTokenKind::KWIf,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_line(2).new_range(3..7),
            kind: ZoidTokenKind::KWElse,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_line(3).new_range(8..10),
            kind: ZoidTokenKind::KWFn,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_line(4).new_range(11..14),
            kind: ZoidTokenKind::KWLet,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_line(5).new_range(15..21),
            kind: ZoidTokenKind::KWReturn,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_line(6).new_range(22..25),
            kind: ZoidTokenKind::KWFor,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_line(7).new_range(26..31),
            kind: ZoidTokenKind::KWWhile,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_line(8).new_range(32..37),
            kind: ZoidTokenKind::KWBreak,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_line(9).new_range(38..46),
            kind: ZoidTokenKind::KWContinue,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_line(10).new_range(47..49),
            kind: ZoidTokenKind::KWIn,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_line(11).new_range(50..56),
            kind: ZoidTokenKind::KWStruct,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_line(12).new_range(57..61),
            kind: ZoidTokenKind::KWEnum,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_line(13).new_range(62..67),
            kind: ZoidTokenKind::KWUnion,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_line(14).new_range(68..72),
            kind: ZoidTokenKind::KWImpl,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_line(15).new_range(73..78),
            kind: ZoidTokenKind::KWTrait,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_line(16).new_range(79..84),
            kind: ZoidTokenKind::KWWhere,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_line(17).new_range(85..90),
            kind: ZoidTokenKind::KWAsync,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_line(18).new_range(91..96),
            kind: ZoidTokenKind::KWAwait,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_line(19).new_range(97..100),
            kind: ZoidTokenKind::KWGen,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_line(20).new_range(101..106),
            kind: ZoidTokenKind::KWYield,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_line(21).new_range(107..113),
            kind: ZoidTokenKind::KWImport,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_line(22).new_range(114..121),
            kind: ZoidTokenKind::KWImportC,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_line(24).new_range(123..126),
            kind: ZoidTokenKind::OpAnd,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_line(25).new_range(127..129),
            kind: ZoidTokenKind::OpOr,
        }))
    );
    assert_eq!(
        lexer.tokenize(),
        Ok(Some(ZoidToken {
            location: base_loc.new_line(26).new_range(130..133),
            kind: ZoidTokenKind::OpNot,
        }))
    );
    assert_eq!(lexer.tokenize(), Ok(None));
}
