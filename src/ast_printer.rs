use crate::ast::AstNode;

pub struct AstPrinter {

}

impl AstPrinter {
    pub fn print(node: Box<AstNode>) {
        match *node {
            AstNode::Binary { .. } => {}
            AstNode::Unary { .. } => {}
            AstNode::Grouping { .. } => {}
            AstNode::Literal { .. } => {}
        }
    }
}
