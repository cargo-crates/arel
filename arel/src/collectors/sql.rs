use std::default::Default;
use serde_json::{Value as Json};
use crate::statements;

pub fn value_sql_string_from_json(json_value: &Json) -> anyhow::Result<String> {
    match json_value {
        Json::Array(json_array) => {
            let mut values = vec![];
            for json_value in json_array.iter() {
                values.push(value_sql_string_from_json(json_value)?);
            }
            Ok(format!("({})", values.join(", ")))
        },
        _ => statements::core_value_sql_string_from_json(json_value)
    }
}

#[derive(Clone, Debug)]
pub struct Sql {
    pub value: String,
    pub prepare_value: Option<Vec<Json>>,
}

impl Default for Sql {
    fn default() -> Self {
        Self {
            value: "".to_string(),
            prepare_value: None,
        }
    }
}

impl Sql {
    pub fn new(value: String) -> Self {
        Self {
            value,
            prepare_value: None,
        }
    }
    pub fn new_with_prepare(value: String, prepare_value: Vec<Json>) -> Self {
        let mut sql = Self::new(value);
        sql.prepare_value = Some(prepare_value);
        sql
    }
    pub fn push(&mut self, char: char) -> &mut Self {
        self.value.push(char);
        self
    }
    pub fn push_str(&mut self, sub_str: &str) -> &mut Self {
        self.value.push_str(sub_str);
        self
    }
    pub fn push_prepare_value(&mut self, sub_prepare_value: &Vec<Json>) -> &mut Self {
        if let Some(prepare_value) = &mut self.prepare_value {
            prepare_value.extend_from_slice(sub_prepare_value);
        } else {
            self.prepare_value = Some(sub_prepare_value.clone());
        }
        self
    }
    pub fn push_str_with_prepare_value(&mut self, sub_str: &str, sub_prepare_value: &Vec<Json>) -> &mut Self {
        self.value.push_str(sub_str);
        self.push_prepare_value(sub_prepare_value);
        self
    }
    pub fn push_from_sql(&mut self, sql: &Sql) -> &mut Self {
        if let Some(prepare_value) = &sql.prepare_value {
            self.push_str_with_prepare_value(&sql.value, prepare_value);
        } else {
            self.push_str(&sql.value);
        }
        self
    }
    pub fn push_from_sqls(&mut self, sqls: &Vec<Sql>, join_str: &str) -> &mut Self {
        let len = sqls.len();
        for (idx, sql) in sqls.iter().enumerate() {
            self.push_from_sql(sql);
            if idx != len - 1 {
                self.push_str(join_str);
            }
        }
        self
    }
    pub fn to_sql_string(&self) -> anyhow::Result<String> {
        if let Some(prepare_value) = &self.prepare_value {
            let mut replace_idx = 0;
            let raw_sql = self.value.chars().map(|char|
                match char {
                    '?' => {
                        let use_replace_value = prepare_value.get(replace_idx).expect("参数不足");
                        replace_idx += 1;
                        value_sql_string_from_json(use_replace_value)
                    },
                    _ => Ok(char.to_string())
                }).collect::<anyhow::Result<String>>()?;
            if replace_idx == prepare_value.len() {
                Ok(raw_sql)
            } else {
                Err(anyhow::anyhow!("prepare sql params count not match: {}", raw_sql))
            }
        } else {
            Ok(self.value.clone())
        }
    }
    #[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres", feature = "mssql"))]
    pub(crate) async fn fetch_one(&self) -> anyhow::Result<sqlx::any::AnyRow> {
        let db_state = crate::visitors::get_db_state()?;
        let mut query = sqlx::query(&self.value);
        if let Some(prepare_value) = &self.prepare_value {
            for prepare_item in prepare_value.iter() {
                query = query.bind(value_sql_string_from_json(prepare_item)?);
            }
        }
        let row = query.fetch_one(db_state.pool()).await?;
        Ok(row)
    }
    #[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres", feature = "mssql"))]
    pub(crate) async fn fetch_all(&self) -> anyhow::Result<Vec<sqlx::any::AnyRow>> {
        let db_state = crate::visitors::get_db_state()?;
        let mut query = sqlx::query(&self.value);
        if let Some(prepare_value) = &self.prepare_value {
            for prepare_item in prepare_value.iter() {
                query = query.bind(value_sql_string_from_json(prepare_item)?);
            }
        }
        let rows = query.fetch_all(db_state.pool()).await?;
        Ok(rows)
    }
    #[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres", feature = "mssql"))]
    pub(crate) async fn execute(&self) -> anyhow::Result<sqlx::any::AnyQueryResult> {
        let db_state = crate::visitors::get_db_state()?;
        let mut query = sqlx::query(&self.value);
        if let Some(prepare_value) = &self.prepare_value {
            for prepare_item in prepare_value.iter() {
                query = query.bind(value_sql_string_from_json(prepare_item)?);
            }
        }
        let query_result = query.execute(db_state.pool()).await?;
        Ok(query_result)
    }
}