use std::fmt;

#[allow(dead_code)]
pub enum DMLNode {
    Select(SelectNode),
}

pub enum Field {
    Name(String),
    All,
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Field::Name(name) => write!(f, "{}", name),
            Field::All => write!(f, "*"),
        }
    }
}

pub struct SelectNode {
    pub fields: Vec<Field>,
    pub result_table: String,
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
            write!(f, "{}", ",")?;
        }
        write!(f, " from {}", self.result_table)?;
        Ok(())
    }
}
