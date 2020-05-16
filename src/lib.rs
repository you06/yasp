mod ast;

// use crate::ast::dml::SelectNode;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(#[allow(clippy::all)] pub grammar);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select() {
        let exprs = grammar::ExprsParser::new()
            .parse("select * from sakura")
            .unwrap();
        assert_eq!(exprs.len(), 1);
        assert_eq!(&format!("{}", exprs[0]), "select * from sakura");

        let exprs = grammar::ExprsParser::new()
            .parse("select * from sakura; select rin, shizuku, * from uta")
            .unwrap();
        assert_eq!(exprs.len(), 2);
        assert_eq!(&format!("{}", exprs[0]), "select * from sakura");
        assert_eq!(&format!("{}", exprs[1]), "select rin,shizuku,* from uta");
    }
}
