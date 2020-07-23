use yasp_error::{Result, Error};
use yasp_ast::expr::{Expr};
use crate::parsers::grammar::{ExprParser,ExprsParser, Token};

use lalrpop_util::{ParseError};

mod parsers {
    pub mod grammar;
}

type Exprs = Vec<Expr>;

fn lalrpop_err(e: ParseError<usize, Token, &str>) -> Error {
    // FIXME: encapsulate error better
    Error::new(format!("parse error: {:?}", e))
}

pub fn parse_expr<T: ToString>(sql: T) -> Result<Expr> {
    let sql = sql.to_string();
    let parser = ExprParser::new();
    let ast = parser.parse(&sql);
    let ast = ast.map_err(lalrpop_err)?;
    Ok(ast)
}

pub fn parse<T: ToString>(sql: T) -> Result<Exprs> {
    let sql = sql.to_string();
    let parser = ExprsParser::new();
    let ast = parser.parse(&sql);
    let ast = ast.map_err(lalrpop_err)?;
    Ok(ast)
}
