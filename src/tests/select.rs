#[cfg(test)]
mod tests {
    // use crate::ast::{dml::*, expr::*, model::*};
    // use crate::*;

    // #[test]
    // fn test_select() {
    //     let expr1 = grammar::ExprParser::new()
    //         .parse("select * from sakura")
    //         .unwrap();
    //     assert_eq!(
    //         expr1,
    //         Expr::Select(SelectNode {
    //             fields: vec![Field {
    //                 all: true,
    //                 table: None,
    //                 column: None
    //             }],
    //             result_table: "sakura".into(),
    //         })
    //     );
    //     assert_eq!(&format!("{}", expr1), "select * from sakura");

    //     let expr2 = grammar::ExprParser::new()
    //         .parse("select Sakura.ShiZuKu, rin, * from Sakura")
    //         .unwrap();
    //     assert_eq!(
    //         expr2,
    //         Expr::Select(SelectNode {
    //             fields: vec![
    //                 Field {
    //                     all: false,
    //                     table: Some(CIStr {
    //                         l: "sakura".to_owned(),
    //                         o: "Sakura".to_owned()
    //                     }),
    //                     column: Some(CIStr {
    //                         l: "shizuku".to_owned(),
    //                         o: "ShiZuKu".to_owned()
    //                     })
    //                 },
    //                 Field {
    //                     all: false,
    //                     table: None,
    //                     column: Some(CIStr {
    //                         l: "rin".to_owned(),
    //                         o: "rin".to_owned()
    //                     })
    //                 },
    //                 Field {
    //                     all: true,
    //                     table: None,
    //                     column: None
    //                 },
    //             ],
    //             result_table: CIStr {
    //                 l: "sakura".to_owned(),
    //                 o: "Sakura".to_owned()
    //             },
    //         })
    //     );
    //     assert_eq!(
    //         &format!("{}", expr2),
    //         "select Sakura.ShiZuKu,rin,* from Sakura"
    //     );

    //     let exprs = grammar::ExprsParser::new()
    //         .parse("select * from sakura;    select Sakura.ShiZuKu, rin, * from Sakura")
    //         .unwrap();
    //     assert_eq!(exprs, vec![expr1, expr2]);
    // }
}
