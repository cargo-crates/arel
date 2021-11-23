pub mod r#where;
pub use r#where::Where;

use serde_json::{Value as Json, json};
use regex::Regex;
use crate::nodes::SqlLiteral;
use crate::traits::ModelAble;

#[derive(Clone, Debug)]
pub enum StatementsType<M: ModelAble> {
    Where(Where<M>),
}

pub trait StatementAble<M: ModelAble> {
    fn full_column_name(column_name: &str) -> String {
        if column_name == "*" {
            format!("`{}`.{}", M::table_name(), column_name)
        } else if Regex::new(r"\.").unwrap().is_match(column_name) {
            format!("{}", column_name)
        } else {
            format!("`{}`.`{}`", M::table_name(), column_name)
        }
    }
    fn value(&self) -> &Json;
    fn to_sql(&self) -> Vec<SqlLiteral>;
}