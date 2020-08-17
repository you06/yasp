#[cfg(test)]
mod tests {
    use yasp_ast::{dml::*, expr::*};
    use yasp_datum::{Datum, Kind};
    use yasp_parser_lalrpop::Parser;

    #[test]
    fn test_update() {
        let parser = Parser::<Datum>::new();

        let expr1 = parser
            .parse_expr("update sakura set rin=7, shizuku = 13")
            .unwrap();
        assert_eq!(
            expr1,
            Expr::Update(UpdateStmt {
                table: "sakura".into(),
                list: vec![
                    Assignment {
                        field: Field::new_column("rin".into()),
                        expr: Expr::Datum(Datum {
                            kind: Kind::Int64(7),
                        }),
                    },
                    Assignment {
                        field: Field::new_column("shizuku".into()),
                        expr: Expr::Datum(Datum {
                            kind: Kind::Int64(13),
                        }),
                    }
                ],
            })
        );
        assert_eq!(&format!("{}", expr1), "update sakura set rin=7, shizuku=13");
    }
}
