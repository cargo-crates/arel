pub mod helpers;
pub mod r#where;
pub use r#where::Where;


use serde_json::{Value as Json, json};
use crate::nodes::SqlLiteral;
use crate::traits::ModelAble;

pub trait StatementAble<M: ModelAble> {
    fn value(&self) -> &Json;
    fn to_sql_literals(&self) -> Vec<SqlLiteral>;
    fn to_sql(&self) -> String;
}