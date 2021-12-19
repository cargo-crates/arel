pub use sqlx::{Row as SqlxRow};
use crate::traits::ArelAble;
use std::marker::PhantomData;
use sqlx::Column;

pub struct Row<M: ArelAble> {
    pub sqlx_row: sqlx::any::AnyRow,
    _marker: PhantomData<M>,
}

impl<M> Row<M> where M: ArelAble {
    pub fn table_column_names() -> Vec<&'static str> {
        M::table_column_names()
    }
    pub fn new(sqlx_row: sqlx::any::AnyRow) -> Self {
        Self {
            sqlx_row,
            _marker: PhantomData,
        }
    }
    pub fn columns(&self) -> &[sqlx::any::AnyColumn] {
        self.sqlx_row.columns()
    }
    pub fn column_names(&self) -> Vec<&str> {
        self.columns().into_iter().map(|column| column.name()).collect()
    }
    pub fn get_column_value_i64(&self, column_name: &str) -> anyhow::Result<i64> {
        match self.sqlx_row.try_get::<i64, _>(column_name) {
            Ok(value) => Ok(value),
            Err(e) => Err(anyhow::anyhow!("{}", e.to_string()))
        }
    }
    pub fn get_column_value_bool(&self, column_name: &str) -> anyhow::Result<bool> {
        match self.sqlx_row.try_get::<bool, _>(column_name) {
            Ok(value) => Ok(value),
            Err(e) => Err(anyhow::anyhow!("{}", e.to_string()))
        }
    }
    pub fn get_column_value_string(&self, column_name: &str) -> anyhow::Result<String> {
        match self.sqlx_row.try_get::<String, _>(column_name) {
            Ok(value) => Ok(value),
            Err(e) => Err(anyhow::anyhow!("{}", e.to_string()))
        }
    }
}