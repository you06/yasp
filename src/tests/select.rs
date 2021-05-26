#[cfg(test)]
mod tests {
    use crate::tests::util::*;
    
    use yasp_ast::{dml::*, expr::*};
    use yasp_datum::Datum;
    use yasp_parser_lalrpop::Parser;

    fn new_select(table: &str, fields: Vec<TestField>) -> Expr<Datum> {
        Expr::Select(SelectStmt{
            fields: fields.into_iter().map(|field| {
                match field {
                    TestField::All=>{
                        Field::new_all()
                    }
                    TestField::Field( c) => {
                        Field::new_column(c.into())
                    }
                    TestField::TableField(t, c) => {
                        Field::new_column(c.into()).with_table(t.into())
                    }
                }
            }).collect(),
            result_table: table.into(),
        })
    }

    #[test]
    fn test_select() {
        let parser = Parser::<Datum>::new();

        // Single statement
        let expr1 = parser.parse_expr("select * from sakura").unwrap();
        assert_eq!(
            expr1,
            new_select("sakura", vec![TestField::All])
        );
        assert_eq!(&format!("{}", expr1), "select * from sakura");

        let expr2 = parser
            .parse_expr("select Sakura.ShiZuKu, rin, * from Sakura")
            .unwrap();
        assert_eq!(
            expr2,
            new_select("Sakura", vec![TestField::TableField("Sakura", "ShiZuKu"), TestField::Field("rin"), TestField::All])
        );
        assert_eq!(
            &format!("{}", expr2),
            "select Sakura.ShiZuKu, rin, * from Sakura"
        );

        // Multi statement
        let exprs = parser
            .parse("select * from sakura;    select Sakura.ShiZuKu, rin, * from Sakura")
            .unwrap();
        assert_eq!(exprs, vec![expr1.clone(), expr2.clone()]);

        let exprs = parser
            .parse("SeLecT * fRom sakura;    sElect Sakura.ShiZuKu, rin, * FroM Sakura")
            .unwrap();
        assert_eq!(exprs, vec![expr1, expr2]);
    }
}
