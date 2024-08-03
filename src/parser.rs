use anyhow::Error;

use crate::ast::AstNode::{
    Binary, Grouping, Literal, StmtExpression, StmtPrint, StmtVariable, Unary, VariableExpression,
};
use crate::ast::{AstNode, LiteralValue};
use crate::scanner::{Token, TokenType, TokenValue};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { current: 0, tokens }
    }

    pub fn parse(&mut self) -> Vec<AstNode> {
        let mut statements: Vec<AstNode> = Vec::new();
        while !self.is_at_end() {
            statements.push(*self.declaration());
        }

        statements
    }

    fn expression(&mut self) -> Box<AstNode> {
        self.equality()
    }

    fn equality(&mut self) -> Box<AstNode> {
        let mut exp = self.comparison();
        while let Some(operator) = self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual])
        {
            let right = self.comparison();
            exp = Box::new(Binary {
                left: exp,
                operator,
                right,
            });
        }
        exp
    }

    fn match_tokens(&mut self, types: &[TokenType]) -> Option<Token> {
        for &token_type in types {
            if self.check(token_type) {
                return Some(self.advance().clone());
            }
        }
        None
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().ty == token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().ty == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current.clone()]
    }

    fn previous(&self) -> &Token {
        &self.tokens[&self.current - 1]
    }

    fn comparison(&mut self) -> Box<AstNode> {
        let mut expr = self.term();
        while let Some(operator) = self.match_tokens(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let right = self.term();
            expr = Box::new(Binary {
                left: expr,
                operator,
                right,
            });
        }
        expr
    }

    fn term(&mut self) -> Box<AstNode> {
        let mut expr = self.factor();
        while let Some(operator) = self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let right = self.factor();
            expr = Box::new(Binary {
                left: expr,
                operator,
                right,
            });
        }
        expr
    }

    fn factor(&mut self) -> Box<AstNode> {
        let mut expr = self.unary();
        while let Some(operator) = self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let right = self.unary();
            expr = Box::new(Binary {
                left: expr,
                operator,
                right,
            });
        }
        expr
    }

    fn unary(&mut self) -> Box<AstNode> {
        if let Some(operator) = self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let right = self.unary();
            return Box::new(Unary { operator, right });
        }
        self.primary()
    }

    fn primary(&mut self) -> Box<AstNode> {
        if self.match_tokens(&[TokenType::False]).is_some() {
            return Box::new(Literal {
                value: LiteralValue::False,
            });
        }
        if self.match_tokens(&[TokenType::True]).is_some() {
            return Box::new(Literal {
                value: LiteralValue::True,
            });
        }
        if self.match_tokens(&[TokenType::Nil]).is_some() {
            return Box::new(Literal {
                value: LiteralValue::Nil,
            });
        }

        if let Some(token) = self.match_tokens(&[TokenType::Number, TokenType::String]) {
            return match token.literal {
                Some(TokenValue::StringLiteral(ref value)) => Box::new(Literal {
                    value: LiteralValue::String(value.clone()),
                }),
                Some(TokenValue::NumberLiteral(value)) => Box::new(Literal {
                    value: LiteralValue::Number(value),
                }),
                _ => panic!("Unexpected token value!"), // Handle other cases or errors
            };
        }

        if self.match_tokens(&[TokenType::Ident]).is_some() {
            return VariableExpression {
                value: self.previous().clone().lexeme,
            }
            .into();
        }

        if self.match_tokens(&[TokenType::LeftParen]).is_some() {
            let expr = self.expression();
            self.consume(
                TokenType::RightParen,
                "Expect ')' after expression.".to_string(),
            )
            .unwrap();
            return Box::new(Grouping { node: expr });
        }

        panic!("Unexpected token!") // Placeholder for error handling
    }

    fn consume(&mut self, token_type: TokenType, message: String) -> Result<Token, Error> {
        if self.check(token_type) {
            let token = self.advance();
            Ok(token.to_owned())
        } else {
            self.error(self.peek().clone(), message)
        }
    }

    fn error(&mut self, token: Token, message: String) -> Result<Token, Error> {
        if token.ty == TokenType::Eof {
            println!("{} at end {}", token.line, message);
        } else {
            println!("{} at {} {}", token.line, token.lexeme, message);
        }
        Ok(token)
    }

    fn statement(&mut self) -> Box<AstNode> {
        if self.match_tokens(&[TokenType::Print]).is_some() {
            return self.print_statement();
        }

        self.expression_statement()
    }

    fn declaration(&mut self) -> Box<AstNode> {
        if self.match_tokens(&[TokenType::Var]).is_some() {
            return self.var_declaration();
        }
        self.statement()
    }

    fn var_declaration(&mut self) -> Box<AstNode> {
        let name = self
            .consume(TokenType::Ident, "Expected variable name".into())
            .unwrap();
        let mut initializer = None;
        if self.match_tokens(&[TokenType::Equal]).is_some() {
            initializer = Some(self.expression());
        }

        self.consume(
            TokenType::Semicolon,
            "Expect ; after variable declaration".into(),
        )
        .expect("Expect ; after variable declaration");

        StmtVariable {
            name: match name.literal.unwrap() {
                TokenValue::Identifier(name) => name,
                _ => unreachable!(),
            },
            initializer,
        }
        .into()
    }

    fn print_statement(&mut self) -> Box<AstNode> {
        let expression = self.expression();
        self.consume(TokenType::Semicolon, "Expect ; after value.".into())
            .unwrap();

        StmtPrint { value: expression }.into()
    }

    fn expression_statement(&mut self) -> Box<AstNode> {
        let expression = self.expression();
        self.consume(TokenType::Semicolon, "Expect ; after value.".into())
            .unwrap();

        StmtExpression { value: expression }.into()
    }
}
