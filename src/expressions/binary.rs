use crate::expressions::expression::Expression;
use crate::Token;

pub struct Binary {
    left: Box<dyn Expression>,
    operator: Token,
    right: Box<dyn Expression>,
}

impl Binary {
    pub fn new(left: Box<dyn Expression>,
           operator: Token,
           right: Box<dyn Expression>) -> Binary {
        Binary {
            left,
            operator,
            right,
        }
    }
}

impl Expression for Binary {}

