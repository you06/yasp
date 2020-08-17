use super::dml::SelectNode;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Expr {
    Select(SelectNode),
    UnKnown,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Select(select_node) => write!(f, "{}", select_node),
            Expr::UnKnown => write!(f, "unknown expression"),
        }
    }
}
