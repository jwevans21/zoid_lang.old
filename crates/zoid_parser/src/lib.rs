use core::panic;

use zoid_ast::{
    BinaryOperator, Expression, Literal, Parameter, Program, Statement, TopLevelExpression, Type,
};
use zoid_lexer::{
    token::{Token, TokenKind},
    Lexer,
};

pub struct Parser<'fname, 'source> {
    #[allow(unused)]
    file: &'fname str,
    lexer: Lexer<'fname, 'source>,
    program: Program<'source>,
}

impl<'fname, 'source> Parser<'fname, 'source> {
    pub fn new(file: &'fname str, source: &'source str) -> Self {
        Self {
            file,
            lexer: Lexer::new(file, source),
            program: Program(Vec::new()),
        }
    }

    pub fn reset(&mut self) {
        self.lexer.reset();
    }

    fn expect(&mut self, kind: TokenKind) -> Token<'fname, 'source> {
        if let Some(tok) = self.lexer.next() {
            if tok.kind != kind {
                panic!("Expected {:?}, got {:?}", kind, tok.kind);
            }
            tok
        } else {
            panic!("Expected {:?}, got EOF", kind);
        }
    }

    fn expect_one_of(&mut self, kinds: &[TokenKind]) -> Token<'fname, 'source> {
        if let Some(tok) = self.lexer.next() {
            if !kinds.contains(&tok.kind) {
                panic!("Expected one of {:?}, got {:?}", kinds, tok.kind);
            }
            tok
        } else {
            panic!("Expected one of {:?}, got EOF", kinds);
        }
    }

    fn next_is(&mut self, kind: TokenKind) -> bool {
        if let Some(tok) = self.lexer.clone().peekable().peek() {
            tok.kind == kind
        } else {
            false
        }
    }

    #[allow(dead_code)]
    fn next_is_one_of(&mut self, kinds: &[TokenKind]) -> bool {
        if let Some(tok) = self.lexer.clone().peekable().peek() {
            kinds.contains(&tok.kind)
        } else {
            false
        }
    }

    pub fn parse(&mut self) -> Program<'source> {
        while let Some(tok) = self.lexer.next() {
            match tok.kind {
                TokenKind::KeywordFn => self.parse_function(),
                _ => panic!("Unexpected token: {:?}", tok),
            }
        }

        self.program.clone()
    }

    fn parse_type(&mut self) -> Type {
        match self.lexer.by_ref().next() {
            Some(tok) => match tok.kind {
                TokenKind::TypeI8 => Type::I8,
                TokenKind::TypeI16 => Type::I16,
                TokenKind::TypeI32 => Type::I32,
                TokenKind::TypeI64 => Type::I64,
                TokenKind::TypeI128 => Type::I128,
                TokenKind::TypeU8 => Type::U8,
                TokenKind::TypeU16 => Type::U16,
                TokenKind::TypeU32 => Type::U32,
                TokenKind::TypeU64 => Type::U64,
                TokenKind::TypeU128 => Type::U128,
                TokenKind::TypeF32 => Type::F32,
                TokenKind::TypeF64 => Type::F64,
                TokenKind::TypeVoid => Type::Void,
                _ => panic!("Expected type, got {:?}", tok),
            },
            None => panic!("Expected type, got EOF"),
        }
    }

    fn parse_function(&mut self) {
        let name = self
            .expect(TokenKind::Identifier)
            .value
            .expect("Expected non-empty identifier");

        self.expect(TokenKind::LParen);
        let parameters = self.parse_parameters();
        self.expect(TokenKind::RParen);

        let return_type = if self.next_is(TokenKind::Colon) {
            self.expect(TokenKind::Colon);
            Some(self.parse_type())
        } else {
            None
        };

        self.expect(TokenKind::LBrace);

        let mut body = Vec::new();

        while !self.next_is(TokenKind::RBrace) {
            body.push(self.parse_statement());
        }

        self.expect(TokenKind::RBrace);

        self.program.0.push(TopLevelExpression::Function {
            name,
            parameters,
            return_type,
            body,
        });
    }

    fn parse_parameters(&mut self) -> Vec<Parameter<'source>> {
        let mut parameters = Vec::new();

        if self.next_is(TokenKind::RParen) {
            return parameters;
        }

        loop {
            let name = self
                .expect(TokenKind::Identifier)
                .value
                .expect("Expected non-empty identifier");

            self.expect(TokenKind::Colon);
            let ty = self.parse_type();

            parameters.push(Parameter { name, ty });

            if self.next_is(TokenKind::RParen) {
                break;
            }

            self.expect(TokenKind::Comma);
        }

        parameters
    }

    fn parse_statement(&mut self) -> Statement<'source> {
        let tok = self.expect_one_of(&[TokenKind::KeywordLet, TokenKind::KeywordReturn]);

        match tok.kind {
            TokenKind::KeywordLet => self.parse_variable_declaration(),
            TokenKind::KeywordReturn => self.parse_return(),
            _ => panic!("Expected statement, got {:?}", tok),
        }
    }

    fn parse_variable_declaration(&mut self) -> Statement<'source> {
        let name = self
            .expect(TokenKind::Identifier)
            .value
            .expect("Expected non-empty identifier");

        let ty = if self.next_is(TokenKind::Colon) {
            self.expect(TokenKind::Colon);
            Some(self.parse_type())
        } else {
            None
        };

        self.expect(TokenKind::OpAssign);

        let value = self.parse_expression();

        self.expect(TokenKind::Semicolon);

        Statement::VariableDeclaration { name, ty, value }
    }

    fn parse_return(&mut self) -> Statement<'source> {
        let value = if self.next_is(TokenKind::Semicolon) {
            None
        } else {
            Some(self.parse_expression())
        };

        self.expect(TokenKind::Semicolon);

        Statement::Return(value)
    }

    /// This method is heavily inspired by the Kaleidoscope tutorial:
    /// https://www.llvm.org/docs/tutorial/MyFirstLanguageFrontend/LangImpl02.html
    fn parse_expression(&mut self) -> Expression<'source> {
        let lhs = self.parse_expression_primary();

        self.parse_binary_operation(lhs, 0)
    }

    fn parse_expression_primary(&mut self) -> Expression<'source> {
        let tok = self.lexer.next().expect("Expected expression, got EOF");

        match tok.kind {
            TokenKind::IntegerLiteral => {
                let value = tok.value.expect("Expected non-empty integer literal");
                Expression::Literal(Literal::Integer(value))
            }
            TokenKind::Identifier => Expression::Variable(
                tok.value
                    .expect("Expected non-empty identifier for variable expression"),
            ),
            TokenKind::LParen => self.parse_paren_expression(),
            _ => panic!("Expected expression, got {:?}", tok),
        }
    }

    fn parse_paren_expression(&mut self) -> Expression<'source> {
        let expr = self.parse_expression();
        self.expect(TokenKind::RParen);
        expr
    }

    fn get_tok_precedence(&mut self) -> u8 {
        match self.lexer.clone().peekable().peek() {
            Some(tok) => match tok.kind {
                TokenKind::OpAdd | TokenKind::OpSub => 20,
                TokenKind::OpMul | TokenKind::OpDiv | TokenKind::OpRem => 40,
                _ => 0,
            },
            None => 0,
        }
    }

    fn parse_binary_operation(
        &mut self,
        lhs: Expression<'source>,
        current_precision: u8,
    ) -> Expression<'source> {
        let mut lhs = lhs;
        loop {
            let tok_precision = self.get_tok_precedence();

            if tok_precision <= current_precision {
                return lhs;
            }

            let op = match self.lexer.by_ref().next() {
                Some(tok) => match tok.kind {
                    TokenKind::OpAdd => BinaryOperator::Add,
                    TokenKind::OpSub => BinaryOperator::Sub,
                    TokenKind::OpMul => BinaryOperator::Mul,
                    TokenKind::OpDiv => BinaryOperator::Div,
                    TokenKind::OpRem => BinaryOperator::Rem,
                    _ => panic!("Expected binary operator, got {:?}", tok),
                },
                None => panic!("Expected binary operator, got EOF"),
            };

            let mut rhs = self.parse_expression_primary();

            let next_precision = self.get_tok_precedence();

            if tok_precision < next_precision {
                rhs = self.parse_binary_operation(rhs, tok_precision + 1);
            }

            lhs = Expression::BinaryOperation {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            };
        }
    }
}
