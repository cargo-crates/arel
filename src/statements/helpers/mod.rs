pub mod and;

use crate::statements::{StatementAble};
// use crate::collectors::{SqlString};
use crate::traits::ModelAble;

pub fn inject_join<M: ModelAble, S: StatementAble<M>>(list: &Vec<S>, join_str: &str) -> anyhow::Result<String> {
    // list.iter().map(|i| i.to_sql_literals()).flatten().map(|sql_literal| sql_literal.raw_sql).collect::<Vec<String>>().join(&format!("{}", join_str))
    Ok(list.iter().map(|i| i.to_sql()).collect::<anyhow::Result<Vec<String>>>()?.join(&format!("{}", join_str)))
}