#[allow(dead_code)]
pub enum TestField<'a> {
    All,
    Field(&'a str),
    TableField(&'a str, &'a str),
}
