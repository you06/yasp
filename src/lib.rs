mod ast;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammar);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select() {
        let exprs = grammar::ExprsParser::new()
            .parse("select * from sakura")
            .unwrap();
        assert_eq!(&format!("{:?}", exprs), "[select * from sakura]");

        let exprs = grammar::ExprsParser::new()
            .parse("select * from sakura; select rin, shizuku, * from uta")
            .unwrap();
        assert_eq!(
            &format!("{:?}", exprs),
            "[select * from sakura, select rin,shizuku,* from uta]"
        );
    }
}
