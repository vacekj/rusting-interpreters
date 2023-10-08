use core::num::flt2dec::Sign;
use std::ops::{Neg, Not};
use crate::{Token, TokenType};

#[derive(Debug)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
    Expression(Box<AstNode>),
}

impl Not for LiteralValue {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            LiteralValue::False | LiteralValue::Nil => {
                LiteralValue::False
            }
            _ => LiteralValue::True
        }
    }
}

#[derive(Debug)]
pub enum AstNode {
    Binary { left: Box<AstNode>, operator: Token, right: Box<AstNode> },
    Unary {
        operator: Token,
        right: Box<AstNode>,
    },
    Grouping {
        node: Box<AstNode>
    },
    Literal {
        value: LiteralValue
    },
}

impl AstNode {
    fn to_string(&self) -> String {
        match &self {
            AstNode::Binary { left, operator, right } => {
                AstNode::parenthesize(operator.lexeme.clone(), &[left, right])
            }
            AstNode::Unary { .. } => {
                todo!()
            }
            AstNode::Grouping { node } => {
                node.to_string()
            }
            AstNode::Literal { value } => {
                match value {
                    LiteralValue::Number(number) => {
                        format!("{}", number)
                    }
                    LiteralValue::String(str) => {
                        str.clone()
                    }
                    LiteralValue::True => {
                        true.to_string()
                    }
                    LiteralValue::False => {
                        false.to_string()
                    }
                    LiteralValue::Nil => {
                        "nil".into()
                    }
                    LiteralValue::Expression(exp) => {
                        exp.to_string()
                    }
                }
            }
        }
    }

    fn evaluate(self) -> LiteralValue {
        match self {
            AstNode::Binary { right, operator, left } => {
                let left = left.evaluate();
                let right = right.evaluate();
                match operator.ty {
                    TokenType::Minus => {
                        match (left, right) {
                            (LiteralValue::Number(left), LiteralValue::Number(right)) => {
                                return LiteralValue::Number(left - right)
                            }
                            _ => panic!("Cannot subtract non-numbers")
                        }
                    }
                    TokenType::Slash => {
                        match (left, right) {
                            (LiteralValue::Number(left), LiteralValue::Number(right)) => {
                                return LiteralValue::Number(left / right)
                            }
                            _ => panic!("Cannot divide non-numbers")
                        }
                    }
                    TokenType::Star => {
                        match (left, right) {
                            (LiteralValue::Number(left), LiteralValue::Number(right)) => {
                                return LiteralValue::Number(left * right)
                            }
                            _ => panic!("Cannot product non-numbers")
                        }
                    }
                    TokenType::Plus => {
                        match (left, right) {
                            (LiteralValue::String(left), LiteralValue::String(right)) => {
                                return LiteralValue::String(left.clone() + &*right)
                            }
                            (LiteralValue::Number(left), LiteralValue::Number(right)) => {
                                return LiteralValue::Number(left + right)
                            }
                            _ => panic!("Can only add numbers or strings")
                        }
                    }
                }

                todo!()
            }
            AstNode::Unary { operator, right } => {
                let right = right.evaluate();

                match operator.ty {
                    TokenType::Bang => {
                        !right
                    }
                    TokenType::Minus => {
                        match right {
                            LiteralValue::Number(value) => {
                                LiteralValue::Number(-value)
                            }
                            _ => panic!("Cannot negate non-numbers")
                        }
                    }
                    _ => todo!()
                }
            }
            AstNode::Grouping { node } => { node.evaluate() }
            AstNode::Literal { value } => { value }
        }
    }

    pub fn parenthesize(name: String, exprs: &[&AstNode]) -> String {
        let mut builder = String::new();

        builder.push('(');
        builder.push_str(&name);
        for expr in exprs {
            builder.push(' ');
            builder.push_str(&expr.to_string());
        }
        builder.push(')');

        builder
    }
}