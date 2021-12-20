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
use crate::traits::ArelAble;
use crate::collectors::Sql;

pub trait StatementAble<M: ArelAble> {
    fn json_value(&self) -> Option<&Json>;
    fn default_value_sql_string_from_json(json_value: &Json) -> anyhow::Result<String> {
        match json_value {
            Json::Array(json_array) => {
                let mut values = vec![];
                for json_value in json_array.iter() {
                    values.push(Self::value_sql_string_from_json(json_value)?);
                }
                Ok(format!("({})", values.join(", ")))
            },
            _ => core_value_sql_string_from_json(json_value)
        }
    }
    fn value_sql_string_from_json(json_value: &Json) -> anyhow::Result<String> {
        Self::default_value_sql_string_from_json(json_value)
    }
    fn default_to_sub_sqls(&self) -> anyhow::Result<Vec<Sql>> {
        let mut vec = vec![];
        if let Some(json_value) = self.json_value() {
            match json_value {
                Json::String(json_string) => {
                    let mut sql = Sql::default();
                    sql.push_str(json_string);
                    vec.push(sql);
                },
                Json::Array(json_array) if json_array.len() >= 1 => {
                    if let Json::String(raw_sql) = json_array.get(0).unwrap() {
                        let mut sql = Sql::default();
                        sql.push_str_with_prepare_value(raw_sql, &json_array[1..].to_vec());
                        vec.push(sql);
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
    fn to_sub_sqls(&self) -> anyhow::Result<Vec<Sql>> {
        self.default_to_sub_sqls()
    }
    fn to_sql_with_concat(&self, concat: &str) -> anyhow::Result<Sql> {
        let mut collector = Sql::default();
        let sub_sqls = self.to_sub_sqls()?;
        collector.push_from_sqls(&sub_sqls, concat);
        Ok(collector)
    }
    fn to_sql(&self) -> anyhow::Result<Sql>;
    // fn to_sql_string_with_concat(&self, concat: &str) -> anyhow::Result<String> {
    //     Ok(self.to_sql_literals()?.into_iter().map(|sql_literal| sql_literal.to_sql()).collect::<anyhow::Result<Vec<String>>>()?.join(&format!("{}", concat)))
    // }
    fn to_sql_string(&self) -> anyhow::Result<String> {
        self.to_sql()?.to_sql_string()
    }
}

pub fn core_value_sql_string_from_json(json_value: &Json) -> anyhow::Result<String> {
    match json_value {
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
        Json::Null => { Ok(format!("{}", "NULL")) },
        _ => Err(anyhow::anyhow!("Error: Not Support"))
    }
}