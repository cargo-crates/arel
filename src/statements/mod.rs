pub mod helpers;
pub mod select;
pub mod join;
pub mod r#where;
pub mod group;
pub mod having;
pub mod order;
pub mod limit;
pub mod offset;
pub mod lock;
pub mod insert;
pub mod update;

pub use select::Select;
pub use join::Join;
pub use r#where::Where;
pub use group::Group;
pub use having::Having;
pub use order::Order;
pub use limit::Limit;
pub use offset::Offset;
pub use lock::Lock;
pub use insert::Insert;
pub use update::Update;

use serde_json::{Value as Json};
use crate::nodes::SqlLiteral;
use crate::traits::ModelAble;

pub trait StatementAble<M: ModelAble> {
    fn json_value(&self) -> Option<&Json>;
    fn to_sql_literals_default(&self) -> anyhow::Result<Vec<SqlLiteral>> {
        let mut vec = vec![];
        if let Some(json_value) = self.json_value() {
            match json_value {
                Json::String(json_string) => {
                    vec.push(SqlLiteral::new(json_string.to_string()));
                },
                Json::Array(json_array) if json_array.len() >= 1 => {
                    if let Json::String(raw_sql) = json_array.get(0).unwrap() {
                        let mut replace_idx = 1;
                        let raw_sql = raw_sql.chars().map(|char|
                            match char {
                                '?' => {
                                    let use_replace_value = json_array.get(replace_idx).expect("参数不足");
                                    replace_idx += 1;
                                    self.json_value_sql(use_replace_value)
                                },
                                _ => Ok(char.to_string())
                            }).collect::<anyhow::Result<String>>()?;
                        vec.push(SqlLiteral::new(raw_sql));
                    } else {
                        return Err(anyhow::anyhow!("Error: {:?} Not Support, 第一个元素必须为字符串", self.json_value()))
                    }
                }
                _ => {
                    return Err(anyhow::anyhow!("Error: {:?} Not Support", self.json_value()))
                }
            }
        }
        Ok(vec)
    }
    fn to_sql_literals(&self) -> anyhow::Result<Vec<SqlLiteral>> {
        self.to_sql_literals_default()
    }
    fn to_sql_with_concat(&self, concat: &str) -> anyhow::Result<String> {
        Ok(self.to_sql_literals()?.into_iter().map(|sql_literal| sql_literal.raw_sql).collect::<Vec<String>>().join(&format!("{}", concat)))
    }
    fn to_sql(&self) -> anyhow::Result<String>;
    fn json_value_sql_default(&self, json_value: &Json) -> anyhow::Result<String> {
        match json_value {
            Json::Array(json_array) => {
                let mut values = vec![];
                for json_value in json_array.iter() {
                    values.push(self.json_value_sql(json_value)?);
                }
                Ok(format!("({})", values.join(", ")))
            },
            Json::String(json_string) => {
                Ok(format!("'{}'", json_string))
            },
            Json::Number(json_number) => {
                Ok(format!("{}", json_number))
            },
            Json::Bool(json_bool) => {
                let value = if *json_bool {1} else {0};
                Ok(format!("{}", value))
            },
            Json::Null => { Ok(format!("{}", "null")) },
            _ => Err(anyhow::anyhow!("Error: Not Support"))
        }
    }
    fn json_value_sql(&self, json_value: &Json) -> anyhow::Result<String> {
        self.json_value_sql_default(json_value)
    }
}