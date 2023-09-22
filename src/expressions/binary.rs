use crate::expressions::expression::{Expression, Printer};
use crate::{Token};

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

impl Expression for Binary {
    fn to_string(&self) -> String {
        Printer::parenthesize(self.operator.lexeme.clone(), &[&self.left, &self.right])
    }
}

