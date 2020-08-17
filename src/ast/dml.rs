use super::model::*;
use std::fmt;

#[allow(dead_code)]
pub enum DMLNode {
    Select(SelectNode),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SelectNode {
    pub fields: Vec<Field>,
    pub result_table: CIStr,
}

impl fmt::Display for SelectNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "select ")?;
        let l = self.fields.len();
        for index in 0..l {
            write!(f, "{}", self.fields[index])?;
            if index == l - 1 {
                break;
            }
            write!(f, ",")?;
        }
        write!(f, " from {}", self.result_table)?;
        Ok(())
    }
}

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
