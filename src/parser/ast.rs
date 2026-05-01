// defines how tokens *can* be grouped together.

#[derive(Debug, PartialEq)]
pub enum Expr {
    EdgeList(Vec<String>),
}
