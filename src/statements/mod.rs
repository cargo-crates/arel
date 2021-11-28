pub mod helpers;
pub mod r#where;
pub mod join;
pub mod update;

pub use r#where::Where;
pub use join::Join;
pub use update::Update;

use serde_json::{Value as Json};
use crate::nodes::SqlLiteral;
use crate::traits::ModelAble;

pub trait StatementAble<M: ModelAble> {
    fn value(&self) -> &Json;
    fn to_sql_literals_default(&self) -> Vec<SqlLiteral> {
        let mut vec = vec![];
        match self.value() {
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
                            _ => char.to_string()
                        }).collect::<String>();
                    vec.push(SqlLiteral::new(raw_sql));
                } else {
                    panic!("Error:  Not Support, 第一个元素必须为字符串")
                }
            }
            _ => {
                println!("==={:?}", self.value());
                panic!("Error: Not Support!")
            }
        }
        vec
    }
    fn to_sql_literals(&self) -> Vec<SqlLiteral> {
        self.to_sql_literals_default()
    }
    fn to_sql_with_concat(&self, concat: &str) -> String {
        self.to_sql_literals().into_iter().map(|sql_literal| sql_literal.raw_sql).collect::<Vec<String>>().join(&format!("{}", concat))
    }
    fn to_sql(&self) -> String;
    fn json_value_sql_default(&self, json_value: &Json) -> String {
        match json_value {
            Json::Array(json_array) => {
                let mut values = vec![];
                for json_value in json_array.iter() {
                    values.push(self.json_value_sql(json_value));
                }
                format!("({})", values.join(", "))
            },
            Json::String(json_string) => {
                format!("'{}'", json_string)
            },
            Json::Number(json_number) => {
                format!("{}", json_number)
            },
            Json::Bool(json_bool) => {
                let value = if *json_bool {1} else {0};
                format!("{}", value)
            },
            Json::Null => { format!("{}", "null") },
            _ => panic!("Error: Not Support")
        }
    }
    fn json_value_sql(&self, json_value: &Json) -> String {
        self.json_value_sql_default(json_value)
    }
}