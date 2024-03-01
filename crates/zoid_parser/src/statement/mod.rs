use allocator_api2::vec::Vec;
use zoid_ast::Statement;
use zoid_lexer::{ZoidToken, ZoidTokenKind};

use crate::ZoidParser;

impl<'arena, 'fname, 'input> ZoidParser<'arena, 'fname, 'input> {
    pub(crate) fn parse_block(&mut self) -> Option<&'arena [&'arena Statement<'arena>]> {
        let mut stmts = Vec::new_in(self.arena);

        loop {
            let tok: ZoidToken<'_> = self.lexer.next()?; //.ok_or("expected token, found EOF")?;

            match tok.kind {
                ZoidTokenKind::RBrace => {
                    break;
                }
                ZoidTokenKind::BlockComment | ZoidTokenKind::LineComment => {}
                _ => {
                    let stmt = self.parse_statement(tok)?;
                    stmts.push(stmt);
                }
            }
        }

        Some(stmts.leak())
    }

    fn parse_statement(&mut self, tok: ZoidToken<'fname>) -> Option<&'arena Statement<'arena>> {
        const ACCEPTED: [ZoidTokenKind; 18] = [
            ZoidTokenKind::LBrace,
            ZoidTokenKind::KWBreak,
            ZoidTokenKind::KWContinue,
            ZoidTokenKind::KWIf,
            ZoidTokenKind::KWWhile,
            ZoidTokenKind::KWReturn,
            ZoidTokenKind::KWLet,
            ZoidTokenKind::Identifier,
            ZoidTokenKind::LParen,
            ZoidTokenKind::CStringLiteral,
            ZoidTokenKind::StringLiteral,
            ZoidTokenKind::IntLiteral,
            ZoidTokenKind::FloatLiteral,
            ZoidTokenKind::BoolLitFalse,
            ZoidTokenKind::BoolLitTrue,
            ZoidTokenKind::OpSub,
            ZoidTokenKind::OpNot,
            ZoidTokenKind::OpBitNot,
        ];

        let tok = loop {
            match self.expect_one_of(Some(tok), &ACCEPTED) {
                Some(tok) => break tok,
                None => continue
            }};

        Some(match tok.kind {
            ZoidTokenKind::LBrace => {
                let stmts = self.parse_block()?;
                self.arena.alloc(Statement::Block(stmts))
            }
            ZoidTokenKind::KWBreak => {
                self.expect(ZoidTokenKind::Semicolon)?;
                self.arena.alloc(Statement::Break)
            }
            ZoidTokenKind::KWContinue => {
                self.expect(ZoidTokenKind::Semicolon)?;
                self.arena.alloc(Statement::Continue)
            }
            ZoidTokenKind::KWIf => {
                let cond = self.parse_expression(None)?;

                let tok = self.lexer.by_ref().next()?;
                // .ok_or("expected token, found EOF")?;

                let then = self.parse_statement(tok)?;

                let els = if self.next_is(ZoidTokenKind::KWElse) {
                    self.expect(ZoidTokenKind::KWElse)?;
                    let tok = self.lexer.by_ref().next()?;
                    // .ok_or("expected token, found EOF")?;

                    Some(self.parse_statement(tok)?)
                } else {
                    None
                };

                let inner: &'arena _ = self.arena.alloc((cond, then, els));
                self.arena.alloc(Statement::If(inner))
            }
            ZoidTokenKind::KWWhile => {
                let cond = self.parse_expression(None)?;

                let tok = self.lexer.by_ref().next()?;
                // .ok_or("expected token, found EOF")?;

                let body = self.parse_statement(tok)?;

                let inner = self.arena.alloc((cond, body));
                self.arena.alloc(Statement::While(inner))
            }
            ZoidTokenKind::KWReturn => {
                let res = if self.next_is(ZoidTokenKind::Semicolon) {
                    self.lexer.next();
                    self.arena.alloc(Statement::Return(None))
                } else {
                    let expr = self.parse_expression(None)?;
                    self.arena.alloc(Statement::Return(Some(expr)))
                };

                self.expect(ZoidTokenKind::Semicolon)?;

                res
            }
            ZoidTokenKind::KWLet => {
                let name = self.expect(ZoidTokenKind::Identifier)?;

                let next =
                    self.expect_one_of(None, &[ZoidTokenKind::Semicolon, ZoidTokenKind::OpAssign])?;

                let ty = if next.kind == ZoidTokenKind::Colon {
                    let res = Some(self.parse_type(None)?);
                    self.expect(ZoidTokenKind::OpAssign)?;
                    res
                } else {
                    None
                };

                let expr = self.parse_expression(None)?;

                self.expect(ZoidTokenKind::Semicolon)?;

                let name: &'arena str = self
                    .arena
                    .alloc_str(&self.input[name.location.start..name.location.end]);

                let inner = self.arena.alloc((name, expr, ty));
                self.arena.alloc(Statement::VariableDeclaration(inner))
            }
            _ => {
                let expr = self.parse_expression(Some(tok))?;
                self.expect(ZoidTokenKind::Semicolon)?;

                self.arena.alloc(Statement::Expression(expr))
            }
        })
    }
}
