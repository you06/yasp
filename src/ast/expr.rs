use super::dml::{SelectStmt, UpdateStmt, Field};
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
            Expr::Select(stmt) => write!(f, "{}", stmt),
            Expr::Update(stmt) => write!(f, "{}", stmt),
            Expr::UnKnown => write!(f, "unknown expression"),
        }
    }
}
