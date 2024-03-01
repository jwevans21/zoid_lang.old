use allocator_api2::vec::Vec;
use bumpalo::Bump;
use hashbrown::{hash_map::DefaultHashBuilder, HashMap};
use thiserror::Error;
use zoid_ast::{TopLevel, Type};
use zoid_diagnostic::{ZoidDiagnostic, ZoidErrorCode};
use zoid_lexer::{ZoidLexer, ZoidToken, ZoidTokenKind};

mod expression;
mod statement;
mod top_level;

#[derive(Debug, Error)]
pub enum ZoidParseError<'fname, 'source, 'arena> {
    #[error("allocation error: {0}")]
    AllocError(bumpalo::AllocErr),
    #[error("{0}")]
    SyntaxError(ZoidDiagnostic<'fname, 'source, 'arena>),
}

pub type ZoidParseResult<'fname, 'source, 'arena, T> =
    Result<T, ZoidParseError<'fname, 'source, 'arena>>;

impl From<bumpalo::AllocErr> for ZoidParseError<'_, '_, '_> {
    fn from(e: bumpalo::AllocErr) -> Self {
        Self::AllocError(e)
    }
}

impl<'fname, 'source, 'arena> From<ZoidDiagnostic<'fname, 'source, 'arena>>
    for ZoidParseError<'fname, 'source, 'arena>
{
    fn from(e: ZoidDiagnostic<'fname, 'source, 'arena>) -> Self {
        Self::SyntaxError(e)
    }
}

#[derive(Debug)]
pub struct ZoidParser<'arena, 'fname, 'input> {
    arena: &'arena Bump,
    #[allow(unused)]
    fname: &'fname str,
    input: &'input str,
    lexer: ZoidLexer<'fname, 'input>,
    program: Vec<TopLevel<'arena>, &'arena Bump>,
    #[allow(unused)]
    symbol_table: HashMap<&'arena str, (), DefaultHashBuilder, &'arena Bump>,
    diagnostics: Vec<ZoidDiagnostic<'fname, 'input, 'arena>, &'arena Bump>,
}

impl<'arena, 'fname, 'input> ZoidParser<'arena, 'fname, 'input> {
    const SKIP: [ZoidTokenKind; 2] = [ZoidTokenKind::BlockComment, ZoidTokenKind::LineComment];

    pub fn new(arena: &'arena Bump, fname: &'fname str, input: &'input str) -> Self {
        Self {
            arena,
            fname,
            input,
            lexer: ZoidLexer::new(fname, input),
            program: Vec::new_in(arena),
            symbol_table: HashMap::new_in(arena),
            diagnostics: Vec::new_in(arena),
        }
    }

    pub fn pretty_print(&self) {
        for top_level in &self.program {
            println!("{}", top_level.pretty_print(0));
        }
    }

    fn expect(&mut self, expected: ZoidTokenKind) -> Option<ZoidToken<'fname>> {
        match self.lexer.tokenize() {
            Ok(tok) => match tok {
                Some(tok) if tok.kind == expected => Some(tok),
                Some(ZoidToken { kind, .. }) if Self::SKIP.contains(&kind) => self.expect(expected),
                Some(tok) => {
                    let msg = self.arena.alloc_str(&format!(
                        "unexpected token `{:?}`, expected `{:?}`",
                        tok.kind, expected
                    ));
                    self.diagnostics.push(ZoidDiagnostic::error(
                        tok.location,
                        self.input.lines(),
                        ZoidErrorCode::UnexpectedToken,
                        msg,
                    ));
                    None
                }
                None if expected == ZoidTokenKind::EOF => Some(ZoidToken {
                    location: self.lexer.location(),
                    kind: ZoidTokenKind::EOF,
                }),
                None => {
                    let msg = self
                        .arena
                        .alloc_str(&format!("unexpected EOF, expected `{:?}`", expected));
                    self.diagnostics.push(ZoidDiagnostic::error(
                        self.lexer.location(),
                        self.input.lines(),
                        ZoidErrorCode::UnexpectedEOF,
                        msg,
                    ));
                    None
                }
            },
            Err(e) => {
                let msg = self.arena.alloc_str(&e);
                self.diagnostics.push(ZoidDiagnostic::error(
                    self.lexer.location(),
                    self.input.lines(),
                    ZoidErrorCode::UnknownToken,
                    msg,
                ));
                None
            }
        }
    }

    fn expect_one_of(
        &mut self,
        init: Option<ZoidToken<'fname>>,
        expected: &[ZoidTokenKind],
    ) -> Option<ZoidToken<'fname>> {
        let tok = match init {
            Some(tok) => Ok(Some(tok)),
            None => self.lexer.tokenize(),
        };

        match tok {
            Ok(tok) => match tok {
                Some(tok) if expected.contains(&tok.kind) => Some(tok),
                Some(ZoidToken { kind, .. }) if Self::SKIP.contains(&kind) => {
                    self.expect_one_of(None, expected)
                }
                Some(tok) => {
                    let msg = self.arena.alloc_str(&format!(
                        "unexpected token `{}`, expected one of {}",
                        tok.kind,
                        expected
                            .iter()
                            .map(|k| format!("`{}`", k))
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                    self.diagnostics.push(ZoidDiagnostic::error(
                        tok.location,
                        self.input.lines(),
                        ZoidErrorCode::UnexpectedToken,
                        msg,
                    ));
                    None
                }
                None if expected.contains(&ZoidTokenKind::EOF) => Some(ZoidToken {
                    location: self.lexer.location(),
                    kind: ZoidTokenKind::EOF,
                }),
                None => {
                    let msg = self.arena.alloc_str(&format!(
                        "unexpected EOF, expected {}",
                        expected
                            .iter()
                            .map(|k| format!("`{}`", k))
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                    self.diagnostics.push(ZoidDiagnostic::error(
                        self.lexer.location(),
                        self.input.lines(),
                        ZoidErrorCode::UnexpectedEOF,
                        msg,
                    ));
                    None
                }
            },
            Err(e) => {
                let msg = self.arena.alloc_str(&e);
                self.diagnostics.push(ZoidDiagnostic::error(
                    self.lexer.location(),
                    self.input.lines(),
                    ZoidErrorCode::UnknownToken,
                    msg,
                ));
                None
            }
        }
    }

    fn next_is(&mut self, kind: ZoidTokenKind) -> bool {
        self.lexer
            .clone()
            .tokenize()
            .is_ok_and(|o| o.is_some_and(|tok| tok.kind == kind))
    }

    fn next_is_one(&mut self, kinds: &[ZoidTokenKind]) -> bool {
        self.lexer
            .clone()
            .tokenize()
            .is_ok_and(|o| o.is_some_and(|tok| kinds.contains(&tok.kind)))
    }

    pub fn parse(&mut self) -> Option<()> {
        while let Some(true) = self.parse_top_level() {}

        for diag in &self.diagnostics {
            eprintln!("{}", diag);
        }

        Some(())
    }

    fn parse_type(&mut self, init: Option<ZoidToken<'fname>>) -> Option<&'arena Type<'arena>> {
        let tok = match init {
            Some(tok) => tok,
            None => self.lexer.next()?, //.ok_or("expected token, found EOF")?,
        };

        Some(match tok.kind {
            ZoidTokenKind::BlockComment | ZoidTokenKind::LineComment => self.parse_type(None)?,
            ZoidTokenKind::OpMul => {
                let ty = self.parse_type(None)?;
                self.arena.alloc(Type::Pointer(ty))
            }
            ZoidTokenKind::Identifier => {
                let name: &'arena str = self
                    .arena
                    .alloc_str(&self.input[tok.location.start..tok.location.end]);
                match name {
                    "void" => self.arena.alloc(Type::Void),
                    "bool" => self.arena.alloc(Type::Bool),
                    "char" => self.arena.alloc(Type::Char),
                    "u8" => self.arena.alloc(Type::U8),
                    "u16" => self.arena.alloc(Type::U16),
                    "u32" => self.arena.alloc(Type::U32),
                    "u64" => self.arena.alloc(Type::U64),
                    "usize" => self.arena.alloc(Type::Usize),
                    "i8" => self.arena.alloc(Type::I8),
                    "i16" => self.arena.alloc(Type::I16),
                    "i32" => self.arena.alloc(Type::I32),
                    "i64" => self.arena.alloc(Type::I64),
                    "isize" => self.arena.alloc(Type::Isize),
                    "f32" => self.arena.alloc(Type::F32),
                    "f64" => self.arena.alloc(Type::F64),
                    _ => {
                        return None;
                    }
                }
            }
            ZoidTokenKind::KWConst => {
                let ty = self.parse_type(None)?;
                self.arena.alloc(Type::Const(ty))
            }
            _ => {
                return None;
            }
        })
    }
}
