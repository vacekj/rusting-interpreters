use crate::Token;

#[derive(Debug)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
    Expression(Box<AstNode>),
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

    pub fn parenthesize(name: String, exprs: &[&Box<AstNode>]) -> String {
        let mut builder = String::new();

        builder.push_str("(");
        builder.push_str(&name);
        for expr in exprs {
            builder.push_str(" ");
            builder.push_str(&expr.to_string());
        }
        builder.push_str(")");

        builder
    }
}