use crate::expressions::expression::Expression;
use crate::Token;

pub struct Grouping {
    expression: Box<dyn Expression>,
}

impl Grouping {
    pub fn new(
        expression: Box<dyn Expression>) -> Grouping {
        Grouping {
            expression,
        }
    }
}

impl Expression for Grouping {}

