use super::model::*;
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
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
