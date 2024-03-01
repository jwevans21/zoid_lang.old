use allocator_api2::vec::Vec;
use zoid_ast::{BinaryOperator, Expression, PostfixOperator, PrefixOperator};
use zoid_lexer::{ZoidToken, ZoidTokenKind};

use crate::ZoidParser;

impl<'arena, 'fname, 'input> ZoidParser<'arena, 'fname, 'input> {
    const EXPR_START: [ZoidTokenKind; 13] = [
        ZoidTokenKind::CStringLiteral,
        ZoidTokenKind::StringLiteral,
        ZoidTokenKind::IntLiteral,
        ZoidTokenKind::FloatLiteral,
        ZoidTokenKind::BoolLitFalse,
        ZoidTokenKind::BoolLitTrue,
        ZoidTokenKind::Identifier,
        ZoidTokenKind::LParen,
        ZoidTokenKind::LBrace,
        ZoidTokenKind::Semicolon,
        ZoidTokenKind::OpNot,
        ZoidTokenKind::OpBitNot,
        ZoidTokenKind::OpSub,
    ];

    pub(crate) fn parse_expression(
        &mut self,
        init: Option<ZoidToken<'fname>>,
    ) -> Option<&'arena Expression<'arena>> {
        let lhs = self.parse_prefix_unary(init)?;

        if self.next_is(ZoidTokenKind::Comma) {
            return Some(lhs);
        }

        self.parse_binary_op(lhs, 0)
    }

    fn parse_prefix_unary(
        &mut self,
        init: Option<ZoidToken<'fname>>,
    ) -> Option<&'arena Expression<'arena>> {
        // const PREFIX_UNARY: [ZoidTokenKind; 2] = [
        //     ZoidTokenKind::OpNot,
        //     ZoidTokenKind::OpBitNot,
        //     ZoidTokenKind::OpSub
        // ];

        let tok = match init {
            Some(tok) => tok,
            None => loop {
                match self.expect_one_of(init, &Self::EXPR_START) {
                    Some(tok) => break tok,
                    None => continue,
                }
            },
        };

        Some(match tok.kind {
            ZoidTokenKind::OpNot => {
                let inner = self
                    .arena
                    .alloc((PrefixOperator::Not, self.parse_postfix_unary(None)?));
                self.arena.alloc(Expression::UnaryPrefix(inner))
            }
            ZoidTokenKind::OpBitNot => {
                let inner = self
                    .arena
                    .alloc((PrefixOperator::BitNot, self.parse_postfix_unary(None)?));
                self.arena.alloc(Expression::UnaryPrefix(inner))
            }
            ZoidTokenKind::OpSub => {
                let inner = self
                    .arena
                    .alloc((PrefixOperator::Negate, self.parse_postfix_unary(None)?));
                self.arena.alloc(Expression::UnaryPrefix(inner))
            }
            _ => self.parse_postfix_unary(Some(tok))?,
        })
    }

    fn parse_postfix_unary(
        &mut self,
        init: Option<ZoidToken<'fname>>,
    ) -> Option<&'arena Expression<'arena>> {
        const POSTFIX_UNARY: [ZoidTokenKind; 2] = [ZoidTokenKind::OpAddr, ZoidTokenKind::OpDeref];

        let mut primary = self.parse_primary(init)?;

        if self.next_is_one(&POSTFIX_UNARY) {
            let tok = self.expect_one_of(None, &POSTFIX_UNARY)?;
            match tok.kind {
                ZoidTokenKind::OpAddr => {
                    let inner = self.arena.alloc((PostfixOperator::AddressOf, primary));
                    primary = self.arena.alloc(Expression::UnaryPostfix(inner));
                }
                ZoidTokenKind::OpDeref => {
                    let inner = self.arena.alloc((PostfixOperator::Deref, primary));
                    primary = self.arena.alloc(Expression::UnaryPostfix(inner));
                }
                _ => unreachable!(),
            }
        }

        Some(primary)
    }

    fn parse_primary(
        &mut self,
        init: Option<ZoidToken<'fname>>,
    ) -> Option<&'arena Expression<'arena>> {
        const ALLOWED: [ZoidTokenKind; 10] = [
            ZoidTokenKind::CStringLiteral,
            ZoidTokenKind::StringLiteral,
            ZoidTokenKind::IntLiteral,
            ZoidTokenKind::FloatLiteral,
            ZoidTokenKind::BoolLitFalse,
            ZoidTokenKind::BoolLitTrue,
            ZoidTokenKind::Identifier,
            ZoidTokenKind::LParen,
            ZoidTokenKind::LBrace,
            ZoidTokenKind::Semicolon,
        ];

        let tok = loop {
            match self.expect_one_of(init, &ALLOWED) {
                Some(ZoidToken { kind, .. })
                    if [ZoidTokenKind::LBrace, ZoidTokenKind::Semicolon].contains(&kind) =>
                {
                    continue;
                }
                Some(tok) => break tok,
                None => continue,
            }
        };

        Some(match tok.kind {
            ZoidTokenKind::CStringLiteral => {
                let s = &self.input[tok.location.start..tok.location.end];
                let s = s.strip_prefix("c\"").expect("invalid C string literal");
                let s = s.strip_suffix('"').expect("invalid C string literal");

                let s = s.replace("\\n", "\n");

                let s = self.arena.alloc_str(&s);

                self.arena.alloc(Expression::LiteralCString(s))
            }
            ZoidTokenKind::StringLiteral => {
                let s = &self.input[tok.location.start..tok.location.end];
                let s = s.strip_prefix('"').expect("invalid string literal");
                let s = s.strip_suffix('"').expect("invalid string literal");
                let s = self.arena.alloc_str(s);
                self.arena.alloc(Expression::LiteralString(s))
            }
            ZoidTokenKind::IntLiteral => {
                let s = &self.input[tok.location.start..tok.location.end];
                let s = self.arena.alloc_str(s);
                self.arena.alloc(Expression::LiteralInteger(s))
            }
            ZoidTokenKind::FloatLiteral => {
                let s = &self.input[tok.location.start..tok.location.end];
                let s = self.arena.alloc_str(s);
                self.arena.alloc(Expression::LiteralFloat(s))
            }
            ZoidTokenKind::BoolLitFalse => self.arena.alloc(Expression::LiteralBool(false)),
            ZoidTokenKind::BoolLitTrue => self.arena.alloc(Expression::LiteralBool(true)),
            ZoidTokenKind::Identifier => {
                let name: &'arena str = self
                    .arena
                    .alloc_str(&self.input[tok.location.start..tok.location.end]);

                if self.next_is(ZoidTokenKind::LParen) {
                    self.parse_function_call(name)?
                } else {
                    self.arena.alloc(Expression::Variable(name))
                }
            }
            ZoidTokenKind::LParen => {
                let expr = self.parse_expression(None)?;
                self.expect(ZoidTokenKind::RParen)?;
                expr
            }
            _ => unreachable!(),
        })
    }

    fn parse_binary_op(
        &mut self,
        lhs: &'arena Expression<'arena>,
        current_precedence: u8,
    ) -> Option<&'arena Expression<'arena>> {
        const ACCEPTED_BIN_OP: [ZoidTokenKind; 19] = [
            ZoidTokenKind::OpMul,
            ZoidTokenKind::OpDiv,
            ZoidTokenKind::OpRem,
            ZoidTokenKind::OpAdd,
            ZoidTokenKind::OpSub,
            ZoidTokenKind::OpShl,
            ZoidTokenKind::OpShr,
            ZoidTokenKind::OpBitAnd,
            ZoidTokenKind::OpBitOr,
            ZoidTokenKind::OpLeq,
            ZoidTokenKind::OpLt,
            ZoidTokenKind::OpGeq,
            ZoidTokenKind::OpGt,
            ZoidTokenKind::OpEq,
            ZoidTokenKind::OpNe,
            ZoidTokenKind::OpAnd,
            ZoidTokenKind::OpOr,
            ZoidTokenKind::OpAssign,
            ZoidTokenKind::Colon,
        ];

        let mut lhs = lhs;

        loop {
            let tok_precedence = self.get_next_precedence();

            if tok_precedence < current_precedence || tok_precedence == 0 {
                return Some(lhs);
            }

            let tok = loop {
                match self.expect_one_of(None, &ACCEPTED_BIN_OP) {
                    Some(tok) => break tok,
                    None => continue,
                }
            };

            if tok.kind == ZoidTokenKind::Colon {
                let ty = self.parse_type(None)?;
                let inner = self.arena.alloc((ty, lhs));
                return Some(self.arena.alloc(Expression::Cast(inner)));
            }

            let op = Self::token_to_bin_op(tok.kind);

            if op.is_none() {
                return Some(lhs);
            }

            let op = op.expect("Binary operator is None");

            let mut rhs = self.parse_primary(None)?;

            let next_precedence = self.get_next_precedence();

            if tok_precedence < next_precedence {
                rhs = self.parse_binary_op(rhs, tok_precedence + 1)?;
            }

            let inner = self.arena.alloc((op, lhs, rhs));
            lhs = self.arena.alloc(Expression::Binary(inner));
        }
    }

    fn parse_function_call(&mut self, name: &'arena str) -> Option<&'arena Expression<'arena>> {
        self.expect(ZoidTokenKind::LParen)?;

        let mut args = Vec::new_in(self.arena);

        loop {
            let tok = self.lexer.next()?; //.ok_or("expected token, found EOF")?;
            match tok.kind {
                ZoidTokenKind::RParen => {
                    break;
                }
                _ => {
                    let expr = self.parse_expression(Some(tok))?;
                    let tok = loop {
                        match self
                            .expect_one_of(None, &[ZoidTokenKind::Comma, ZoidTokenKind::RParen])
                        {
                            Some(tok) => break tok,
                            None => continue,
                        }
                    };
                    args.push(expr);

                    if tok.kind == ZoidTokenKind::RParen {
                        break;
                    }
                }
            }
        }

        let name: &'arena _ = self.arena.alloc(Expression::Variable(name));
        let inner = self.arena.alloc((name, args.leak() as &'arena [_]));

        Some(self.arena.alloc(Expression::Call(inner)))
    }

    fn token_to_bin_op(kind: ZoidTokenKind) -> Option<BinaryOperator> {
        match kind {
            ZoidTokenKind::OpMul => Some(BinaryOperator::Mul),
            ZoidTokenKind::OpDiv => Some(BinaryOperator::Div),
            ZoidTokenKind::OpRem => Some(BinaryOperator::Rem),
            ZoidTokenKind::OpAdd => Some(BinaryOperator::Add),
            ZoidTokenKind::OpSub => Some(BinaryOperator::Sub),
            ZoidTokenKind::OpShl => Some(BinaryOperator::Shl),
            ZoidTokenKind::OpShr => Some(BinaryOperator::Shr),
            ZoidTokenKind::OpBitAnd => Some(BinaryOperator::BitAnd),
            ZoidTokenKind::OpBitOr => Some(BinaryOperator::BitOr),
            ZoidTokenKind::OpLeq => Some(BinaryOperator::Le),
            ZoidTokenKind::OpLt => Some(BinaryOperator::Lt),
            ZoidTokenKind::OpGeq => Some(BinaryOperator::Ge),
            ZoidTokenKind::OpGt => Some(BinaryOperator::Gt),
            ZoidTokenKind::OpEq => Some(BinaryOperator::Eq),
            ZoidTokenKind::OpNe => Some(BinaryOperator::Ne),
            ZoidTokenKind::OpAnd => Some(BinaryOperator::And),
            ZoidTokenKind::OpOr => Some(BinaryOperator::Or),
            ZoidTokenKind::OpAssign => Some(BinaryOperator::Assign),
            _ => None,
        }
    }

    fn get_next_precedence(&self) -> u8 {
        self.lexer
            .clone()
            .next()
            .and_then(|tok| self.get_precedence(tok.kind))
            .or(Some(0))
            .expect("expected a Some value, found None")
    }

    fn get_precedence(&self, kind: ZoidTokenKind) -> Option<u8> {
        match kind {
            ZoidTokenKind::Colon => Some(100),
            ZoidTokenKind::OpMul | ZoidTokenKind::OpDiv | ZoidTokenKind::OpRem => Some(60),
            ZoidTokenKind::OpAdd | ZoidTokenKind::OpSub => Some(50),
            ZoidTokenKind::OpShl | ZoidTokenKind::OpShr => Some(40),
            ZoidTokenKind::OpBitAnd | ZoidTokenKind::OpBitOr => Some(30),
            ZoidTokenKind::OpLeq
            | ZoidTokenKind::OpLt
            | ZoidTokenKind::OpGeq
            | ZoidTokenKind::OpGt => Some(20),
            ZoidTokenKind::OpEq | ZoidTokenKind::OpNe => Some(20),
            ZoidTokenKind::OpAnd | ZoidTokenKind::OpOr => Some(10),
            ZoidTokenKind::OpAssign => Some(5),
            _ => None,
        }
    }
}
