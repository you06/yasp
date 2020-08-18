use super::expr::Expr;
use super::model::*;
use std::fmt;
use yasp_datum::DatumTrait;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Field {
    pub all: bool,
    pub table: Option<CIStr>,
    pub column: Option<CIStr>,
}

impl Field {
    pub fn new_all() -> Self {
        Field {
            all: true,
            table: None,
            column: None,
        }
    }
    pub fn new_column(column: CIStr) -> Self {
        Field {
            all: false,
            table: None,
            column: Some(column),
        }
    }
    pub fn with_table(mut self, table: CIStr) -> Self {
        self.table = Some(table);
        self
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.all {
            write!(f, "*")?;
            return Ok(());
        }
        if let Some(table) = &self.table {
            write!(f, "{}.", table)?;
        }
        if let Some(column) = &self.column {
            write!(f, "{}", column)?;
        }
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Assignment<T: DatumTrait> {
    pub field: Field,
    pub expr: Expr<T>,
}

impl<T: DatumTrait> fmt::Display for Assignment<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}={}", self.field, self.expr)?;
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SelectStmt {
    pub fields: Vec<Field>,
    pub result_table: CIStr,
}

impl fmt::Display for SelectStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "select ")?;
        let l = self.fields.len();
        for i in 0..l {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", self.fields[i])?;
        }
        write!(f, " from {}", self.result_table)?;
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct UpdateStmt<T: DatumTrait> {
    pub list: Vec<Assignment<T>>,
    pub table: CIStr,
}

impl<T: DatumTrait> fmt::Display for UpdateStmt<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "update {} set ", self.table)?;
        let l = self.list.len();
        for i in 0..l {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", self.list[i])?;
        }
        Ok(())
    }
}
