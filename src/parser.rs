use std::io::BufRead;
use anyhow::Error;
use crate::expressions::expression::Expression;
use crate::{report, Token, TokenType, TokenValue};
use crate::expressions::binary::Binary;
use crate::expressions::grouping::Grouping;
use crate::expressions::literal::{Literal, LiteralValue};
use crate::expressions::unary::Unary;
use crate::TokenType::{Bang, BangEqual, EqualEqual, Minus};

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

        while self.match_tokens(&[BangEqual, EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            exp = Box::new(Binary::new(exp, operator.clone(), right));
        }

        return exp;
    }

    fn match_tokens(&mut self, types: &[TokenType]) -> bool {
        for &token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    // Assume the methods `check` and `advance` look something like this:
    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() { return false; }
        return self.peek().ty == token_type;
    }

    fn advance(&mut self) -> &Token {
        if self.is_at_end() {
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

    fn comparison(&mut self) -> Box<dyn Expression> {
        let mut expr = self.term();

        while self.match_tokens(&[TokenType::Greater, TokenType::GreaterEqual,
            TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous();
            let right = self.term();
            expr = Box::new(Binary::new(expr, operator.clone(), right));
        }

        expr
    }

    fn term(&mut self) -> Box<dyn Expression> {
        let mut expr = self.factor();

        while self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Box::new(Binary::new(expr, operator.clone(), right));
        }

        expr
    }

    fn factor(&mut self) -> Box<dyn Expression> {
        let mut expr = self.unary();

        while self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Box::new(Binary::new(expr, operator.clone(), right));
        }

        expr
    }

    fn unary(&mut self) -> Box<dyn Expression> {
        if self.match_tokens(&[Bang, Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Box::new(Unary::new(operator.clone(), right));
        }

        self.primary()
    }

    fn primary(&mut self) -> Box<dyn Expression> {
        if self.match_tokens(&[TokenType::False]) {
            return Box::new(Literal::new(LiteralValue::False));
        }
        if self.match_tokens(&[TokenType::True]) {
            return Box::new(Literal::new(LiteralValue::True));
        }
        if self.match_tokens(&[TokenType::Nil]) {
            return Box::new(Literal::new(LiteralValue::Nil));
        }

        if self.match_tokens(&[TokenType::Number, TokenType::String]) {
            let token_value = self.previous().literal.unwrap();
            return match token_value {
                TokenValue::StringLiteral(value) => {
                    Box::new(
                        Literal::new(
                            LiteralValue::String(value)))
                }
                TokenValue::NumberLiteral(value) => {
                    Box::new(
                        Literal::new(
                            LiteralValue::Number(value)))
                }
            };
        }

        if self.match_tokens(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.".to_string());
            return Box::new(Grouping::new(expr));
        }

        // Handle unexpected token error or add a default return type here
        panic!("Unexpected token!"); // This is a placeholder, adapt error handling to your needs
    }

    fn consume(&mut self, token_type: TokenType, message: String) -> Result<(), Error> {
        if self.check(token_type) {
            self.advance();
            return Ok(());
        }

        self.error(self.peek(), message)
    }

    fn error(&mut self, token: &Token, message: String) -> Result<(), Error> {
        if token.ty == TokenType::Eof {
            println!("{} at end {}", &token.line, &message);
        } else {
            println!("{} at {} {}", &token.line, &token.lexeme, &message);
        }

        Ok(())
    }
}
