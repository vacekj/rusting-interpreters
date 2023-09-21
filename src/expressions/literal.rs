use crate::expressions::expression::Expression;

pub struct Literal {
    value: LiteralValue
}

pub enum LiteralValue {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
    Expressions(Box<dyn Expression>)
}

impl Literal {
    pub fn new(
        value: LiteralValue) -> Literal {
        Literal {
            value
        }
    }
}

impl Expression for Literal {}

