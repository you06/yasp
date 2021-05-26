#[cfg(test)]
mod tests {
    use crate::tests::util::*;

    use yasp_ast::{dml::*, expr::*};
    use yasp_datum::{Datum, DatumTrait};
    use yasp_parser_lalrpop::Parser;

    fn new_update(table: &str, fields: Vec<(TestField, &str)>) -> Expr<Datum> {
        Expr::Update(UpdateStmt{
            table: table.into(),
            list: fields.into_iter().map(|(field, val)| {
                let datum = Expr::Datum(Datum::from_raw(val));
                match field {
                    TestField::All => {
                        panic!("should not update with Field::All")
                    },
                    TestField::Field(c) => {
                        Assignment{
                            field: Field::new_column(c.into()),
                            expr: datum,
                        }
                    },
                    TestField::TableField(t, c) =>{
                        Assignment{
                            field: Field::new_column(c.into()).with_table(t.into()),
                            expr: datum,
                        }
                    }
                }   
            }).collect(),
        })
    }

    #[test]
    fn test_update() {
        let parser = Parser::<Datum>::new();

        let expr1 = parser
            .parse_expr("update sakura set rin=7, shizuku = \"13\"")
            .unwrap();
        assert_eq!(
            expr1,
            new_update("sakura", vec![(TestField::Field("rin"), "7"), (TestField::Field("shizuku"), "\"13\"")])
        );
        assert_eq!(
            &format!("{}", expr1),
            "update sakura set rin=7, shizuku=\"13\""
        );
    }
}
