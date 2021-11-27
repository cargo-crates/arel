pub mod and;

use crate::statements::{StatementAble};
use crate::collectors::{SqlString};
use crate::traits::ModelAble;

pub fn inject_join<'a, M: ModelAble, S: StatementAble<M>>(list: &'a Vec<S>, collector: &'a mut SqlString, join_str: &str) -> &'a mut SqlString {
    collector.value = list.iter().map(|i| i.to_sql_literals()).flatten().map(|sql_literal| sql_literal.raw_sql).collect::<Vec<String>>().join(&format!("{}", join_str));
    collector
}