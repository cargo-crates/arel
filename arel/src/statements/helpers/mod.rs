pub mod and;

use crate::statements::{StatementAble};
use crate::collectors::{Sql};
use crate::traits::ArelAble;

// pub fn inject_join<M: ArelAble, S: StatementAble<M>>(list: &Vec<S>, join_str: &str) -> anyhow::Result<String> {
//     // list.iter().map(|i| i.to_sql_literals()).flatten().map(|sql_literal| sql_literal.raw_sql).collect::<Vec<String>>().join(&format!("{}", join_str))
//     Ok(list.iter().map(|i| i.to_sql()).collect::<anyhow::Result<Vec<String>>>()?.join(&format!("{}", join_str)))
// }

pub fn inject_join<M: ArelAble, S: StatementAble<M>>(list: &Vec<S>, join_str: &str) -> anyhow::Result<Sql> {
    let sql_list = list.iter().map(|i| i.to_sql()).collect::<anyhow::Result<Vec<Sql>>>()?;
    let mut collector = Sql::default();
    let len = sql_list.len();
    for (idx, sql_string) in sql_list.iter().enumerate() {
        collector.push_from_sql(&sql_string);
        if idx < len - 1 {
            collector.push_str(join_str);
        }
    }
    Ok(collector)

}