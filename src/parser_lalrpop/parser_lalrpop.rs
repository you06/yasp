use crate::parsers::grammar::{ExprParser, ExprsParser, Token};
use yasp_ast::expr::Expr;
use yasp_error::{Error, Result};

use lalrpop_util::ParseError;

mod parsers {
    #[allow(clippy::all)]
    pub mod grammar;
}

type Exprs = Vec<Expr>;

fn lalrpop_err(e: ParseError<usize, Token, &str>) -> Error {
    // FIXME: encapsulate error better
    Error::new(format!("parse error: {:?}", e))
}

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Parser {}
    }

    pub fn parse_expr<T: ToString>(&self, sql: T) -> Result<Expr> {
        let sql = sql.to_string();
        let parser = ExprParser::new();
        let ast = parser.parse(&sql);
        let ast = ast.map_err(lalrpop_err)?;
        Ok(ast)
    }

    pub fn parse<T: ToString>(&self, sql: T) -> Result<Exprs> {
        let sql = sql.to_string();
        let parser = ExprsParser::new();
        let ast = parser.parse(&sql);
        let ast = ast.map_err(lalrpop_err)?;
        Ok(ast)
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}
