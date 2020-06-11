#[macro_use]
extern crate pest_derive;

#[allow(unused_imports)]
use pest::Parser;

#[allow(dead_code)]
#[derive(Parser)]
#[grammar = "lexer.pest"]
struct SQLLexer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let pairs = SQLLexer::parse(Rule::expressions, "select e from v");
        println!("{:?}", pairs);
    }
}
