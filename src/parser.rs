use anyhow::Error;
use crate::expressions::expression::Expression;
use crate::{Token, TokenType, TokenValue};
use crate::expressions::binary::Binary;
use crate::expressions::grouping::Grouping;
use crate::expressions::literal::{Literal, LiteralValue};
use crate::expressions::unary::Unary;

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            current: 0,
            tokens,
        }
    }

    fn expression(&mut self) -> Box<dyn Expression> {
        self.equality()
    }

    fn equality(&mut self) -> Box<dyn Expression> {
        let mut exp = self.comparison();
        while let Some(operator) = self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let right = self.comparison();
            exp = Box::new(Binary::new(exp, operator, right));
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
        if self.is_at_end() { return false; }
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
        &self.tokens[&self.current - 1 + 1]
    }

    fn previous(&self) -> &Token {
        &self.tokens[&self.current - 1]
    }

    fn comparison(&mut self) -> Box<dyn Expression> {
        let mut expr = self.term();
        while let Some(operator) = self.match_tokens(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let right = self.term();
            expr = Box::new(Binary::new(expr, operator, right));
        }
        expr
    }

    fn term(&mut self) -> Box<dyn Expression> {
        let mut expr = self.factor();
        while let Some(operator) = self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let right = self.factor();
            expr = Box::new(Binary::new(expr, operator, right));
        }
        expr
    }

    fn factor(&mut self) -> Box<dyn Expression> {
        let mut expr = self.unary();
        while let Some(operator) = self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let right = self.unary();
            expr = Box::new(Binary::new(expr, operator, right));
        }
        expr
    }

    fn unary(&mut self) -> Box<dyn Expression> {
        if let Some(operator) = self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let right = self.unary();
            return Box::new(Unary::new(operator, right));
        }
        self.primary()
    }

    fn primary(&mut self) -> Box<dyn Expression> {
        if self.match_tokens(&[TokenType::False]).is_some() {
            return Box::new(Literal::new(LiteralValue::False));
        }
        if self.match_tokens(&[TokenType::True]).is_some() {
            return Box::new(Literal::new(LiteralValue::True));
        }
        if self.match_tokens(&[TokenType::Nil]).is_some() {
            return Box::new(Literal::new(LiteralValue::Nil));
        }

        if let Some(token) = self.match_tokens(&[TokenType::Number, TokenType::String]) {
            return match token.literal {
                Some(TokenValue::StringLiteral(ref value)) => {
                    Box::new(Literal::new(LiteralValue::String(value.clone())))
                }
                Some(TokenValue::NumberLiteral(value)) => {
                    Box::new(Literal::new(LiteralValue::Number(value)))
                }
                _ => panic!("Unexpected token value!"),  // Handle other cases or errors
            };
        }

        if self.match_tokens(&[TokenType::LeftParen]).is_some() {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.".to_string()).unwrap();
            return Box::new(Grouping::new(expr));
        }

        panic!("Unexpected token!")  // Placeholder for error handling
    }

    fn consume(&mut self, token_type: TokenType, message: String) -> Result<(), Error> {
        if self.check(token_type) {
            self.advance();
            Ok(())
        } else {
            self.error(self.peek().clone(), message)
        }
    }

    fn error(&mut self, token: Token, message: String) -> Result<(), Error> {
        if token.ty == TokenType::Eof {
            println!("{} at end {}", token.line, message);
        } else {
            println!("{} at {} {}", token.line, token.lexeme, message);
        }
        Ok(())
    }
}
