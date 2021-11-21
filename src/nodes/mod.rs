pub mod statements;
pub mod sql_literal;
pub mod and;
pub mod select_statement;

pub use statements::StatementsType;
pub use sql_literal::SqlLiteral;
pub use and::And;
pub use select_statement::SelectStatement;

use serde_json::{Value as Json, json};

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