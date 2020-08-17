use crate::parsers::grammar::{ExprParser, ExprsParser, Token};
use std::marker::PhantomData;
use yasp_ast::expr::Expr;
use yasp_datum::DatumTrait;
use yasp_error::{Error, Result};

use lalrpop_util::ParseError;

mod parsers {
    #[allow(clippy::all)]
    pub mod grammar;
}

type Exprs<T> = Vec<Expr<T>>;

fn lalrpop_err(e: ParseError<usize, Token, &str>) -> Error {
    // FIXME: encapsulate error better
    Error::new(format!("parse error: {:?}", e))
}

pub struct Parser<T: DatumTrait>(PhantomData<T>);

impl<T: DatumTrait> Parser<T> {
    pub fn new() -> Parser<T> {
        Parser(PhantomData)
    }

    pub fn parse_expr(&self, sql: &str) -> Result<Expr<T>> {
        let parser = ExprParser::new();
        let ast = parser.parse(sql);
        let ast = ast.map_err(lalrpop_err)?;
        Ok(ast)
    }

    pub fn parse(&self, sql: &str) -> Result<Exprs<T>> {
        let parser = ExprsParser::new();
        let ast = parser.parse(sql);
        let ast = ast.map_err(lalrpop_err)?;
        Ok(ast)
    }
}

impl<T: DatumTrait> Default for Parser<T> {
    fn default() -> Self {
        Self::new()
    }
}
