pub mod statements;
pub mod sql_literal;
pub mod and;
pub mod select_statement;

pub use statements::StatementsType;
pub use sql_literal::SqlLiteral;
pub use and::And;
pub use select_statement::SelectStatement;

use serde_json::{Value as Json, json};
use crate::collectors::{SqlString};
use crate::traits::ModelAble;

// #[derive(Clone, Debug)]
// pub enum StatementsType {
//     Projection,
//     Where(Json),
//     Group,
//     Having,
//     Windows,
//     Limit,
//     Lock,
//     Offset,
//     With,
// }

// #[derive(Clone, Debug)]
// pub enum NodesType {
//     SqlLiteral(SqlLiteral),
//     SelectStatement(SelectStatement),
// }

trait NodeAble<M: ModelAble> {
    fn inject_join<'a>(list: &'a Vec<StatementsType<M>>, collector: &'a mut SqlString, join_str: &str) -> &'a mut SqlString {
        for (idx, item) in list.iter().enumerate() {
            if idx != 0 {
                collector.value = format!("{}{}", collector.value, join_str)
            }

        }
        collector
    }
}