pub trait Expression {
    fn to_string(&self) -> String;
}
pub struct Printer;
impl Printer {
    pub fn parenthesize(name: String, exprs: &[&Box<dyn Expression>]) -> String {
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