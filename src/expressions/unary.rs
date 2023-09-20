use crate::expressions::expression::Expression;
use crate::Token;

pub struct Unary {
    operator: Token,
    right: Box<dyn Expression>,
}

impl Unary {
    pub fn new(
        operator: Token,
        right: Box<dyn Expression>) -> Unary {
        Unary {
            operator,
            right,
        }
    }
}

impl Expression for Unary {}

