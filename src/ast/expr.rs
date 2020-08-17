use super::dml::{SelectStmt, UpdateStmt};
use std::fmt;
use yasp_datum::DatumTrait;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Expr<T: DatumTrait> {
    Datum(T),
    Select(SelectStmt),
    Update(UpdateStmt<T>),
    UnKnown,
}

impl<T: DatumTrait> fmt::Display for Expr<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Datum(datum) => write!(f, "{}", datum),
            Expr::Select(node) => write!(f, "{}", node),
            Expr::Update(node) => write!(f, "{}", node),
            Expr::UnKnown => write!(f, "unknown expression"),
        }
    }
}
