use allocator_api2::vec::Vec;
use zoid_ast::{FunctionParams, TopLevel, Type};
use zoid_lexer::ZoidTokenKind;

use crate::ZoidParser;

impl<'arena, 'fname, 'input> ZoidParser<'arena, 'fname, 'input> {
    pub(crate) fn parse_top_level(&mut self) -> Option<bool> {
        const ALLOWED: [ZoidTokenKind; 3] = [
            ZoidTokenKind::EOF,
            ZoidTokenKind::KWExtern,
            ZoidTokenKind::KWFn,
            // ZoidTokenKind::KWImport,
            // ZoidTokenKind::KWImportC,
            // ZoidTokenKind::KWStruct,
        ];

        let tok = loop {
            match self.expect_one_of(None, &ALLOWED) {
                Some(t) => break t,
                None if self.lexer.location().start == self.input.len() => return Some(false),
                None => continue,
            }
        };

        let kind = tok.kind;
        match kind {
            ZoidTokenKind::EOF => return Some(false),
            ZoidTokenKind::KWExtern => {
                self.parse_extern()?;
            }
            ZoidTokenKind::KWFn => {
                self.parse_function()?;
            }
            ZoidTokenKind::KWImport => todo!("handle imports"),
            ZoidTokenKind::KWImportC => todo!("handle imports for c code"),
            ZoidTokenKind::KWStruct => todo!("handle structures"),
            _ => {
                todo!("Other stuff")
            }
        }

        Some(true)
    }

    fn parse_extern(&mut self) -> Option<()> {
        self.expect(ZoidTokenKind::StringLiteral)?;
        self.expect(ZoidTokenKind::KWFn)?;

        let name = self.expect(ZoidTokenKind::Identifier)?;

        self.expect(ZoidTokenKind::LParen)?;
        let (args, va_args) = self.parse_extern_args()?;
        // self.expect(ZoidTokenKind::RParen)?;

        self.expect(ZoidTokenKind::Colon)?;

        let ret = self.parse_type(None)?;

        self.expect(ZoidTokenKind::Semicolon)?;

        let name: &'arena str = self
            .arena
            .alloc_str(&self.input[name.location.start..name.location.end]);

        let inner = self.arena.alloc((name, args, ret, va_args));

        self.program.push(TopLevel::ExternFunction(inner));

        Some(())
    }

    fn parse_function(&mut self) -> Option<()> {
        let name = self.expect(ZoidTokenKind::Identifier)?;

        self.expect(ZoidTokenKind::LParen)?;

        let (args, _va_args) = self.parse_args()?;

        self.expect(ZoidTokenKind::Colon)?;

        let ret = self.parse_type(None)?;

        self.expect(ZoidTokenKind::LBrace)?;

        let body = self.parse_block()?;

        let name: &'arena str = self
            .arena
            .alloc_str(&self.input[name.location.start..name.location.end]);

        let inner = self.arena.alloc((name, args, ret, body));

        self.program.push(TopLevel::Function(inner));

        Some(())
    }

    fn parse_extern_args(&mut self) -> Option<(&'arena [&'arena Type<'arena>], bool)> {
        let mut args = Vec::new_in(self.arena);
        let mut va_args = false;

        loop {
            let tok = self.lexer.next()?; //.ok_or("expected token, found EOF")?;

            match tok.kind {
                ZoidTokenKind::RParen => {
                    break;
                }
                ZoidTokenKind::Identifier => {
                    self.expect(ZoidTokenKind::Colon)?;

                    let ty = self.parse_type(None)?;
                    args.push(ty);

                    let t =
                        self.expect_one_of(None, &[ZoidTokenKind::Comma, ZoidTokenKind::RParen])?;

                    if t.kind == ZoidTokenKind::RParen {
                        break;
                    }
                }
                ZoidTokenKind::VaArgs => {
                    va_args = true;
                    self.expect(ZoidTokenKind::RParen)?;
                    break;
                }
                _ => {
                    let ty = self.parse_type(Some(tok))?;
                    args.push(ty);

                    let t =
                        self.expect_one_of(None, &[ZoidTokenKind::Comma, ZoidTokenKind::RParen])?;

                    if t.kind == ZoidTokenKind::RParen {
                        break;
                    }
                }
            }
        }

        Some((args.leak(), va_args))
    }

    fn parse_args(&mut self) -> Option<FunctionParams<'arena>> {
        let mut args = Vec::new_in(self.arena);
        let mut va_args = false;

        loop {
            let tok = self.lexer.next()?;

            match tok.kind {
                ZoidTokenKind::RParen => {
                    break;
                }
                ZoidTokenKind::Identifier => {
                    let name: &'arena str = self
                        .arena
                        .alloc_str(&self.input[tok.location.start..tok.location.end]);

                    self.expect(ZoidTokenKind::Colon)?;

                    let ty = self.parse_type(None)?;

                    let t =
                        self.expect_one_of(None, &[ZoidTokenKind::Comma, ZoidTokenKind::RParen])?;

                    args.push((name, ty));

                    if t.kind == ZoidTokenKind::RParen {
                        break;
                    }
                }
                ZoidTokenKind::VaArgs => {
                    va_args = true;
                    self.expect(ZoidTokenKind::RParen)?;
                    break;
                }
                _ => {
                    // let err = format!("unexpected token {:?} {}", tok.kind, line!());
                    // self.print_diagnostic(tok.location, &err);
                    // return Err(err);
                }
            }
        }

        Some((args.leak(), va_args))
    }
}
