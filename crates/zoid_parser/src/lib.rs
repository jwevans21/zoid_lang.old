#![feature(allocator_api)]

use bumpalo::Bump;
use zoid_ast::{BinaryOperator, Expression, FunctionParams, Statement, TopLevel, Type};
use zoid_lexer::{ZoidLexer, ZoidToken, ZoidTokenKind};
use zoid_location::ZoidLocation;

#[derive(Debug)]
pub struct ZoidParser<'arena, 'fname, 'input> {
    arena: &'arena Bump,
    fname: &'fname str,
    input: &'input str,
    lexer: ZoidLexer<'fname, 'input>,
    program: Vec<TopLevel<'arena>, &'arena Bump>,
    // symbol_table: HashMap<&'arena str, (), _, &'arena Bump>
}

impl<'arena, 'fname, 'input> ZoidParser<'arena, 'fname, 'input> {
    pub fn new(arena: &'arena Bump, fname: &'fname str, input: &'input str) -> Self {
        Self {
            arena,
            fname,
            input,
            lexer: ZoidLexer::new(fname, input),
            program: Vec::new_in(arena),
            // symbol_table:
        }
    }

    pub fn pretty_print(&self) {
        for top_level in &self.program {
            println!("{}", top_level.pretty_print(0));
        }
    }

    fn print_diagnostic(&self, location: ZoidLocation<'fname>, message: &str) {
        dbg!(&self.program);

        eprintln!(
            "{}:{}:{}: error: {}",
            self.fname, location.line, location.column, message
        );
        for (num, line) in self
            .input
            .lines()
            .enumerate()
            .skip(location.line.saturating_sub(2))
            .take(3)
        {
            eprintln!("{:>4} | {}", num + 1, line);
            if num + 1 == location.line {
                eprintln!(
                    "{:>4} | {:>col$}{}",
                    "",
                    "^",
                    message,
                    col = location.column
                );
            }
        }
    }

    fn expect(&mut self, kind: ZoidTokenKind) -> Result<ZoidToken<'fname>, String> {
        match self.lexer.next() {
            Some(tok) if tok.kind == kind => Ok(tok),
            Some(tok) => {
                let err = format!("expected token of kind {:?}, found {:?}", kind, tok.kind);
                self.print_diagnostic(tok.location, &err);
                Err(err)
            }
            None => {
                let err = format!("expected token of kind {:?}, found EOF", kind);
                self.print_diagnostic(
                    ZoidLocation {
                        file_name: self.fname,
                        start: self.input.len(),
                        end: self.input.len(),
                        line: 0,
                        column: 0,
                    },
                    &err,
                );
                Err(err)
            }
        }
    }

    fn expect_one_of(&mut self, kinds: &[ZoidTokenKind]) -> Result<ZoidToken<'fname>, String> {
        match self.lexer.next() {
            Some(tok) if kinds.contains(&tok.kind) => Ok(tok),
            Some(tok) => {
                let err = format!("expected token of kind {:?}, found {:?}", kinds, tok.kind);
                self.print_diagnostic(tok.location, &err);
                Err(err)
            }
            None => {
                let err = format!("expected token of kind {:?}, found EOF", kinds);
                self.print_diagnostic(
                    ZoidLocation {
                        file_name: self.fname,
                        start: self.input.len(),
                        end: self.input.len(),
                        line: 0,
                        column: 0,
                    },
                    &err,
                );
                Err(err)
            }
        }
    }

    fn next_is(&mut self, kind: ZoidTokenKind) -> bool {
        self.lexer
            .clone()
            .next()
            .is_some_and(|tok| tok.kind == kind)
    }

    pub fn parse(&mut self) -> Result<(), String> {
        while let Some(tok) = self.lexer.by_ref().next() {
            let kind = tok.kind;

            match kind {
                ZoidTokenKind::KWExtern => {
                    self.parse_extern()?;
                }
                ZoidTokenKind::KWFn => {
                    self.parse_function()?;
                }
                ZoidTokenKind::BlockComment | ZoidTokenKind::LineComment => {}
                _ => {
                    let err = format!("unexpected token {:?}", kind);
                    self.print_diagnostic(tok.location, &err);
                    return Err(err);
                }
            }
        }

        Ok(())
    }

    fn parse_extern(&mut self) -> Result<(), String> {
        self.expect(ZoidTokenKind::StringLiteral)?;
        self.expect(ZoidTokenKind::KWFn)?;

        let name = self.expect(ZoidTokenKind::Identifier)?;

        self.expect(ZoidTokenKind::LParen)?;
        let (args, va_args) = self.parse_extern_args()?;
        // self.expect(ZoidTokenKind::RParen)?;

        self.expect(ZoidTokenKind::Colon)?;

        let ret = self.parse_type()?;

        self.expect(ZoidTokenKind::Semicolon)?;

        let name: &'arena str = self
            .arena
            .alloc_str(&self.input[name.location.start..name.location.end]);

        let inner = self.arena.alloc((name, args, ret, va_args));

        self.program.push(TopLevel::ExternFunction(inner));

        Ok(())
    }

    fn parse_function(&mut self) -> Result<(), String> {
        let name = self.expect(ZoidTokenKind::Identifier)?;

        self.expect(ZoidTokenKind::LParen)?;

        let (args, _va_args) = self.parse_args()?;

        self.expect(ZoidTokenKind::Colon)?;

        let ret = self.parse_type()?;

        self.expect(ZoidTokenKind::LBrace)?;

        let body = self.parse_block()?;

        let name: &'arena str = self
            .arena
            .alloc_str(&self.input[name.location.start..name.location.end]);

        let inner = self.arena.alloc((name, args, ret, body));

        self.program.push(TopLevel::Function(inner));

        Ok(())
    }

    fn parse_extern_args(&mut self) -> Result<(&'arena [&'arena Type<'arena>], bool), String> {
        let mut args = Vec::new_in(self.arena);
        let mut va_args = false;

        loop {
            let tok = self.lexer.next().ok_or("expected token, found EOF")?;

            match tok.kind {
                ZoidTokenKind::RParen => {
                    break;
                }
                ZoidTokenKind::Identifier => {
                    self.expect(ZoidTokenKind::Colon)?;

                    let ty = self.parse_type()?;

                    let t = self.expect_one_of(&[ZoidTokenKind::Comma, ZoidTokenKind::RParen])?;

                    args.push(ty);

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
                    let err = format!("unexpected token {:?} {}", tok.kind, line!());
                    self.print_diagnostic(tok.location, &err);
                    return Err(err);
                }
            }
        }

        Ok((args.leak(), va_args))
    }

    fn parse_args(&mut self) -> Result<FunctionParams<'arena>, String> {
        let mut args = Vec::new_in(self.arena);
        let mut va_args = false;

        loop {
            let tok = self.lexer.next().ok_or("expected token, found EOF")?;

            match tok.kind {
                ZoidTokenKind::RParen => {
                    break;
                }
                ZoidTokenKind::Identifier => {
                    let name: &'arena str = self
                        .arena
                        .alloc_str(&self.input[tok.location.start..tok.location.end]);

                    self.expect(ZoidTokenKind::Colon)?;

                    let ty = self.parse_type()?;

                    let t = self.expect_one_of(&[ZoidTokenKind::Comma, ZoidTokenKind::RParen])?;

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
                    let err = format!("unexpected token {:?} {}", tok.kind, line!());
                    self.print_diagnostic(tok.location, &err);
                    return Err(err);
                }
            }
        }

        Ok((args.leak(), va_args))
    }

    fn parse_type(&mut self) -> Result<&'arena Type<'arena>, String> {
        let tok = self.lexer.next().ok_or("expected token, found EOF")?;

        Ok(match tok.kind {
            ZoidTokenKind::BlockComment | ZoidTokenKind::LineComment => self.parse_type()?,
            ZoidTokenKind::OpMul => {
                let ty = self.parse_type()?;
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
                        return Err(format!("unknown type {:?}", name));
                    }
                }
            }
            ZoidTokenKind::KWConst => {
                let ty = self.parse_type()?;
                self.arena.alloc(Type::Const(ty))
            }
            _ => {
                let err = format!("unexpected token {:?} {}", tok.kind, line!());
                self.print_diagnostic(tok.location, &err);
                return Err(err);
                // return Err(format!("expected type, found {:?}", tok.kind));
            }
        })
    }

    fn parse_block(&mut self) -> Result<&'arena [&'arena Statement<'arena>], String> {
        let mut stmts = Vec::new_in(self.arena);

        loop {
            let tok: ZoidToken<'_> = self.lexer.next().ok_or("expected token, found EOF")?;

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

        Ok(stmts.leak())
    }

    fn parse_statement(
        &mut self,
        tok: ZoidToken<'fname>,
    ) -> Result<&'arena Statement<'arena>, String> {
        Ok(match tok.kind {
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

                let tok = self
                    .lexer
                    .by_ref()
                    .next()
                    .ok_or("expected token, found EOF")?;

                let then = self.parse_statement(tok)?;

                let els = if self.next_is(ZoidTokenKind::KWElse) {
                    self.lexer.next();
                    let tok = self
                        .lexer
                        .by_ref()
                        .next()
                        .ok_or("expected token, found EOF")?;

                    Some(self.parse_statement(tok)?)
                } else {
                    None
                };

                let inner: &'arena _ = self.arena.alloc((cond, then, els));
                self.arena.alloc(Statement::If(inner))
            }
            ZoidTokenKind::KWWhile => {
                let cond = self.parse_expression(None)?;

                let tok = self
                    .lexer
                    .by_ref()
                    .next()
                    .ok_or("expected token, found EOF")?;

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
                    self.expect_one_of(&[ZoidTokenKind::Semicolon, ZoidTokenKind::OpAssign])?;

                let ty = if next.kind == ZoidTokenKind::Colon {
                    let res = Some(self.parse_type()?);
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

    fn parse_expression(
        &mut self,
        init: Option<ZoidToken<'fname>>,
    ) -> Result<&'arena Expression<'arena>, String> {
        let lhs = self.parse_primary(init)?;

        if self.next_is(ZoidTokenKind::Comma) {
            return Ok(lhs);
        }

        self.parse_binary_op(lhs, 0)
    }

    fn parse_primary(
        &mut self,
        init: Option<ZoidToken<'fname>>,
    ) -> Result<&'arena Expression<'arena>, String> {
        let tok = match init {
            Some(tok) => tok,
            None => self.lexer.next().ok_or("expected token, found EOF")?,
        };

        Ok(match tok.kind {
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
                let s = s.strip_prefix('"').unwrap();
                let s = s.strip_suffix('"').unwrap();
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
            _ => {
                let err = format!("unexpected token {:?} {}", tok.kind, line!());
                self.print_diagnostic(tok.location, &err);
                return Err(err);
            }
        })
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

    fn get_next_precedence(&self) -> u8 {
        self.lexer
            .clone()
            .next()
            .and_then(|tok| self.get_precedence(tok.kind))
            .or(Some(0))
            .expect("expected a Some value, found None")
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

    fn parse_binary_op(
        &mut self,
        lhs: &'arena Expression<'arena>,
        current_precedence: u8,
    ) -> Result<&'arena Expression<'arena>, String> {
        let mut lhs = lhs;

        loop {
            let tok_precedence = self.get_next_precedence();

            if tok_precedence < current_precedence || tok_precedence == 0 {
                return Ok(lhs);
            }

            let tok = self.lexer.next().ok_or("expected token, found EOF")?;

            if tok.kind == ZoidTokenKind::Colon {
                let ty = self.parse_type()?;
                let inner = self.arena.alloc((ty, lhs));
                return Ok(self.arena.alloc(Expression::Cast(inner)));
            }

            let op = Self::token_to_bin_op(tok.kind);

            if op.is_none() {
                return Ok(lhs);
            }

            let op = op.unwrap();

            let mut rhs = self.parse_primary(None)?;

            let next_precedence = self.get_next_precedence();

            if tok_precedence < next_precedence {
                rhs = self.parse_binary_op(rhs, tok_precedence + 1)?;
            }

            let inner = self.arena.alloc((op, lhs, rhs));
            lhs = self.arena.alloc(Expression::Binary(inner));
        }
    }

    fn parse_function_call(
        &mut self,
        name: &'arena str,
    ) -> Result<&'arena Expression<'arena>, String> {
        self.expect(ZoidTokenKind::LParen)?;

        let mut args = Vec::new_in(self.arena);

        loop {
            let tok = self.lexer.next().ok_or("expected token, found EOF")?;
            match tok.kind {
                ZoidTokenKind::RParen => {
                    break;
                }
                _ => {
                    let expr = self.parse_expression(Some(tok))?;
                    let tok = self.expect_one_of(&[ZoidTokenKind::Comma, ZoidTokenKind::RParen])?;
                    args.push(expr);

                    if tok.kind == ZoidTokenKind::RParen {
                        break;
                    }
                }
            }
        }

        let name: &'arena _ = self.arena.alloc(Expression::Variable(name));
        let inner = self.arena.alloc((name, args.leak() as &'arena [_]));

        Ok(self.arena.alloc(Expression::Call(inner)))
    }
}
