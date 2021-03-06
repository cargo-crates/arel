pub mod select_manager;
pub mod insert_manager;
pub mod update_manager;
pub mod delete_manager;
pub use select_manager::SelectManager;
pub use select_manager::select_statement::SelectStatement;
pub use insert_manager::InsertManager;
pub use update_manager::UpdateManager;
pub use delete_manager::DeleteManager;

use serde_json::{Value as Json, json};
use crate::methods::type_to_pluralize_string;
use crate::traits::ArelAble;
use std::marker::PhantomData;
use crate::collectors::{Sql};
use crate::visitors;
use crate::statements::{r#where, having};
use crate::methods;
#[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres", feature = "mssql"))]
use crate::collectors::row::Row;
// pub trait ManagerStatement<M: ArelAble> {}

#[derive(Clone, Debug)]
pub struct Table<M: ArelAble> {
    pub select_manager: Option<SelectManager<M>>,
    pub insert_manager: Option<InsertManager<M>>,
    pub update_manager: Option<UpdateManager<M>>,
    pub delete_manager: Option<DeleteManager<M>>,
    _marker: PhantomData<M>,
}

impl<M> Table<M> where M: ArelAble {
    /// Get Model's table name.
    ///
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::table::Table;
    ///
    /// #[arel::arel]
    /// struct User {
    ///     id: i64,
    /// }
    /// assert_eq!(User::table_name(), "users");
    /// ```
    pub fn table_name() -> String {
        type_to_pluralize_string::<M>()
    }
    pub fn new() -> Self {
        Self {
            select_manager: None,
            insert_manager: None,
            update_manager: None,
            delete_manager: None,
            _marker: PhantomData
        }
    }
    fn _where_statement(&mut self, condition: Json, ops: r#where::Ops) -> &mut Self {
        if let Some(select_manager) = &mut self.select_manager {
            select_manager.r#where(condition, ops);
        } else if let Some(update_manager) = &mut self.update_manager {
            update_manager.r#where(condition, ops);
        } else if let Some(delete_manager) = &mut self.delete_manager {
            delete_manager.r#where(condition, ops);
        } else {
            panic!("Not support");
        }
        self
    }
    pub fn r#where(&mut self, condition: Json) -> &mut Self {
        self._where_statement(condition, r#where::Ops::new(r#where::JoinType::And, false, false, false))
    }
    pub fn where_prepare(&mut self, condition: Json) -> &mut Self {
        self._where_statement(condition, r#where::Ops::new(r#where::JoinType::And, false, false, true))
    }
    pub fn where_not(&mut self, condition: Json) -> &mut Self {
        self._where_statement(condition, r#where::Ops::new(r#where::JoinType::And, true, false, false))
    }
    pub fn where_between(&mut self, condition: Json) -> &mut Self {
        self._where_statement(condition, r#where::Ops::new(r#where::JoinType::And, false, true, false))
    }
    pub fn where_not_between(&mut self, condition: Json) -> &mut Self {
        self._where_statement(condition, r#where::Ops::new(r#where::JoinType::And, true, true, false))
    }
    pub fn where_or(&mut self, condition: Json) -> &mut Self {
        self._where_statement(condition, r#where::Ops::new(r#where::JoinType::Or, false, false, false))
    }
    pub fn where_or_not(&mut self, condition: Json) -> &mut Self {
        self._where_statement(condition, r#where::Ops::new(r#where::JoinType::Or, true, false, false))
    }
    pub fn where_or_between(&mut self, condition: Json) -> &mut Self {
        self._where_statement(condition, r#where::Ops::new(r#where::JoinType::Or, false, true, false))
    }
    pub fn where_or_not_between(&mut self, condition: Json) -> &mut Self {
        self._where_statement(condition, r#where::Ops::new(r#where::JoinType::Or, true, true, false))
    }
    pub fn where_range<T: serde::Serialize>(&mut self, column_name: &str, range: impl std::ops::RangeBounds<T>) -> &mut Self {
        let table_column_name = methods::table_column_name::<M>(column_name);
        let raw_sql = r#where::help_range_to_sql(&table_column_name, range).expect("Error: Not Support");
        self._where_statement(json!(raw_sql), r#where::Ops::new(r#where::JoinType::And, false, true, false));
        self
    }
    pub fn with_select_manager(&mut self) -> &mut Self {
        if self.select_manager.is_none() {
            self.select_manager = Some(SelectManager::<M>::default());
        }
        self
    }
    pub fn select(&mut self, condition: Json) -> &mut Self {
        if let Some(select_manager) = &mut self.select_manager {
            select_manager.select(condition);
        } else {
            panic!("Not support");
        }
        self
    }
    pub fn count(&mut self) -> &mut Self {
        if let Some(select_manager) = &mut self.select_manager {
            select_manager.count();
        } else {
            panic!("Not support");
        }
        self
    }
    pub fn sum(&mut self, column_name: &str) -> &mut Self {
        if let Some(select_manager) = &mut self.select_manager {
            select_manager.sum(column_name);
        } else {
            panic!("Not support");
        }
        self
    }
    pub fn avg(&mut self, column_name: &str) -> &mut Self {
        if let Some(select_manager) = &mut self.select_manager {
            select_manager.avg(column_name);
        } else {
            panic!("Not support");
        }
        self
    }
    pub fn min(&mut self, column_name: &str) -> &mut Self {
        if let Some(select_manager) = &mut self.select_manager {
            select_manager.min(column_name);
        } else {
            panic!("Not support");
        }
        self
    }
    pub fn max(&mut self, column_name: &str) -> &mut Self {
        if let Some(select_manager) = &mut self.select_manager {
            select_manager.max(column_name);
        } else {
            panic!("Not support");
        }
        self
    }
    pub fn distinct(&mut self) -> &mut Self {
        if let Some(select_manager) = &mut self.select_manager {
            select_manager.distinct();
        } else {
            panic!("Not support");
        }
        self
    }
    pub fn lock(&mut self) -> &mut Self {
        #[cfg(not(feature = "sqlite"))]
        if let Some(select_manager) = &mut self.select_manager {
            select_manager.lock(json!("FOR UPDATE"));
        } else {
            panic!("Not support");
        }
        self
    }
    pub fn joins(&mut self, condition: Json) -> &mut Self {
        if let Some(select_manager) = &mut self.select_manager {
            select_manager.joins(condition);
        } else {
            panic!("Not support");
        }
        self
    }
    pub fn group(&mut self, condition: Json) -> &mut Self {
        if let Some(select_manager) = &mut self.select_manager {
            select_manager.group(condition);
        } else {
            panic!("Not support");
        }
        self
    }
    pub fn _having_statement(&mut self, condition: Json, ops: having::Ops) -> &mut Self {
        if let Some(select_manager) = &mut self.select_manager {
            select_manager.having(condition, ops);
        } else {
            panic!("Not support");
        }
        self
    }
    pub fn having(&mut self, condition: Json) -> &mut Self {
        self._having_statement(condition, having::Ops::new(r#where::JoinType::And, false, false, false))
    }
    pub fn having_not(&mut self, condition: Json) -> &mut Self {
        self._having_statement(condition, having::Ops::new(r#where::JoinType::And, true, false, false))
    }
    pub fn having_between(&mut self, condition: Json) -> &mut Self {
        self._having_statement(condition, having::Ops::new(r#where::JoinType::And, false, true, false))
    }
    pub fn having_not_between(&mut self, condition: Json) -> &mut Self {
        self._having_statement(condition, having::Ops::new(r#where::JoinType::And, true, true, false))
    }
    pub fn having_or(&mut self, condition: Json) -> &mut Self {
        self._having_statement(condition, having::Ops::new(r#where::JoinType::Or, false, false, false))
    }
    pub fn having_or_not(&mut self, condition: Json) -> &mut Self {
        self._having_statement(condition, having::Ops::new(r#where::JoinType::Or, true, false, false))
    }
    pub fn having_or_between(&mut self, condition: Json) -> &mut Self {
        self._having_statement(condition, having::Ops::new(r#where::JoinType::Or, false, false, false))
    }
    pub fn having_or_not_between(&mut self, condition: Json) -> &mut Self {
        self._having_statement(condition, having::Ops::new(r#where::JoinType::Or, true, true, false))
    }
    pub fn having_range<T: serde::Serialize>(&mut self, column_name: &str, range: impl std::ops::RangeBounds<T>) -> &mut Self {
        let table_column_name = methods::table_column_name::<M>(column_name);
        let raw_sql = having::help_range_to_sql(&table_column_name, range).expect("Error: Not Support");
        self._having_statement(json!(raw_sql), having::Ops::new(r#where::JoinType::And, false, true, false));
        self
    }
    pub fn order(&mut self, condition: Json) -> &mut Self {
        if let Some(select_manager) = &mut self.select_manager {
            select_manager.order(condition);
        } else if let Some(update_manager) = &mut self.update_manager {
            update_manager.order(condition);
        } else if let Some(delete_manager) = &mut self.delete_manager {
            delete_manager.order(condition);
        } else {
            panic!("Not support");
        }
        self
    }
    pub fn limit(&mut self, condition: usize) -> &mut Self {
        if let Some(select_manager) = &mut self.select_manager {
            select_manager.limit(condition);
        } else if let Some(update_manager) = &mut self.update_manager {
            update_manager.limit(condition);
        } else if let Some(delete_manager) = &mut self.delete_manager {
            delete_manager.limit(condition);
        } else {
            panic!("Not support");
        }
        self
    }
    pub fn offset(&mut self, condition: usize) -> &mut Self {
        if let Some(select_manager) = &mut self.select_manager {
            select_manager.offset(condition);
        } else if let Some(update_manager) = &mut self.update_manager {
            update_manager.offset(condition);
        } else if let Some(delete_manager) = &mut self.delete_manager {
            delete_manager.offset(condition);
        } else {
            panic!("Not support");
        }
        self
    }
    pub fn paginate(&mut self, page: usize, page_size: usize) -> &mut Self {
        let offset = (page - 1) * page_size;
        self.limit(page_size);
        self.offset(offset);
        self
    }
    pub fn with_update_manager(&mut self) -> &mut Self {
        if self.update_manager.is_none() {
            self.update_manager = Some(UpdateManager::<M>::default());
            // if let Some(select_manager) = &mut self.select_manager {
            //     if let Some(update_manager) = &mut self.update_manager {
            //         update_manager.ctx_mut().wheres.append(&mut select_manager.ctx_mut().wheres);
            //         self.select_manager = None;
            //     }
            // }
        }
        self
    }
    pub fn update_all(&mut self, condition: Json) -> &mut Self {
        self.with_update_manager();
        self.update_manager.as_mut().unwrap().update(condition);
        self
    }
    pub fn increment(&mut self, column_name: &str, by: isize) -> &mut Self {
        self.with_update_manager();
        self.update_manager.as_mut().unwrap().increment(column_name, by);
        self
    }
    pub fn decrement(&mut self, column_name: &str, by: isize) -> &mut Self {
        self.with_update_manager();
        self.update_manager.as_mut().unwrap().decrement(column_name, by);
        self
    }
    pub fn with_insert_manager(&mut self) -> &mut Self {
        if self.insert_manager.is_none() {
            self.insert_manager = Some(InsertManager::<M>::default());
        }
        self
    }
    pub fn create(&mut self, condition: Json) -> &mut Self {
        self.with_insert_manager();
        self.insert_manager.as_mut().unwrap().insert(condition);
        self
    }
    pub fn with_delete_manager(&mut self) -> &mut Self {
        if self.delete_manager.is_none() {
            self.delete_manager = Some(DeleteManager::<M>::default());
        }
        self
    }
    pub fn delete_all(&mut self, condition: Json) -> &mut Self {
        self.with_delete_manager();
        self.r#where(condition);
        self
    }
    pub fn to_sql(&mut self) -> anyhow::Result<Sql> {
        let mut collector = Sql::default();
        if let Some(insert_manager) = &self.insert_manager {
            visitors::to_sql::accept_insert_manager(insert_manager, &mut collector)?;
        } else if let Some(update_manager) = &self.update_manager {
            let mut for_update_select_manager = None;
            if let Some(select_manager) = &mut self.select_manager {
                select_manager.select(json!([M::primary_key()]));
                for_update_select_manager = Some(select_manager);
            }
            visitors::to_sql::accept_update_manager(update_manager, for_update_select_manager, &mut collector)?;
        } else if let Some(delete_manager) = &self.delete_manager {
            let mut for_update_select_manager = None;
            if let Some(select_manager) = &mut self.select_manager {
                select_manager.select(json!([M::primary_key()]));
                for_update_select_manager = Some(select_manager);
            }
            visitors::to_sql::accept_delete_manager(delete_manager, for_update_select_manager, &mut collector)?;
        } else if let Some(select_manager) = &self.select_manager {
            visitors::to_sql::accept_select_manager(select_manager, &mut collector)?;
        }  else {
            return Err(anyhow::anyhow!("Not support"));
        }
        Ok(collector)
    }
    pub fn to_sql_string(&mut self) -> anyhow::Result<String> {
        self.to_sql()?.to_sql_string()
    }
}

// sqlx
#[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres", feature = "mssql"))]
impl<M> Table<M> where M: ArelAble {
    pub async fn fetch_one_with_executor<'c, E>(&mut self, executor: E) -> anyhow::Result<M>
        where E: sqlx::Executor<'c, Database = sqlx::Any>
    {
        let sql = self.to_sql()?;
        let sqlx_row = sql.fetch_one_with_executor(executor).await?;
        Ok(M::new_from_db_row(Row::<M>::new(sqlx_row))?)
    }
    pub async fn fetch_one(&mut self) -> anyhow::Result<M> {
        let db_state = crate::visitors::get_db_state()?;
        self.fetch_one_with_executor(db_state.pool()).await
    }
    pub async fn fetch_first_with_executor<'c, E>(&mut self, executor: E) -> anyhow::Result<M>
        where E: sqlx::Executor<'c, Database = sqlx::Any>
    {
        self.fetch_one_with_executor(executor).await
    }
    pub async fn fetch_first(&mut self) -> anyhow::Result<M> {
        let db_state = crate::visitors::get_db_state()?;
        self.fetch_first_with_executor(db_state.pool()).await
    }
    pub async fn fetch_last_with_executor<'c, E>(&mut self, executor: E) -> anyhow::Result<M>
        where E: sqlx::Executor<'c, Database = sqlx::Any>
    {
        let mut map = serde_json::Map::new();
        map.insert(M::primary_key().to_string(), json!("DESC"));
        self.order(Json::Object(map)).fetch_one_with_executor(executor).await
    }
    pub async fn fetch_last(&mut self) -> anyhow::Result<M> {
        let db_state = crate::visitors::get_db_state()?;
        self.fetch_last_with_executor(db_state.pool()).await
    }
    pub async fn fetch_all_with_executor<'c, E>(&mut self, executor: E) -> anyhow::Result<Vec<M>>
        where E: sqlx::Executor<'c, Database = sqlx::Any>
    {
        let sql = self.to_sql()?;
        let sqlx_rows = sql.fetch_all_with_executor(executor).await?;
        sqlx_rows.into_iter().map(|sqlx_row| M::new_from_db_row(Row::<M>::new(sqlx_row))).collect()
    }
    pub async fn fetch_all(&mut self) -> anyhow::Result<Vec<M>> {
        let db_state = crate::visitors::get_db_state()?;
       self.fetch_all_with_executor(db_state.pool()).await
    }
    pub async fn fetch_count_with_executor<'c, E>(&mut self, executor: E) -> anyhow::Result<i64>
        where E: sqlx::Executor<'c, Database = sqlx::Any>
    {
        let sqlx_row: sqlx::any::AnyRow = self.count().to_sql()?.fetch_one_with_executor(executor).await?;
        let row = Row::<M>::new(sqlx_row);
        match row.get_column_value_i64(row.column_names().get(0).ok_or(anyhow::anyhow!("Column is Blank"))?) {
            Ok(count) => Ok(count),
            Err(e) => Err(anyhow::anyhow!("{:?}", e.to_string())),
        }
    }
    pub async fn fetch_count(&mut self) -> anyhow::Result<i64> {
        let db_state = crate::visitors::get_db_state()?;
       self.fetch_count_with_executor(db_state.pool()).await
    }
    pub async fn execute_with_executor<'c, E>(&mut self, executor: E) -> anyhow::Result<sqlx::any::AnyQueryResult>
        where E: sqlx::Executor<'c, Database = sqlx::Any>
    {
        self.to_sql()?.execute_with_executor(executor).await
    }
    pub async fn execute(&mut self) -> anyhow::Result<sqlx::any::AnyQueryResult> {
        let db_state = crate::visitors::get_db_state()?;
        self.execute_with_executor(db_state.pool()).await
    }
}

impl<M: ArelAble> From<Table<M>> for String {
    fn from(mut table: Table<M>) -> Self {
        table.to_sql_string().unwrap()
    }
}