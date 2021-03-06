#[allow(unused_imports)]
use std::future::Future;
#[allow(unused_imports)]
use std::pin::Pin;
use serde_json::{Value as Json};
use crate::table::Table;
#[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres", feature = "mssql"))]
use crate::collectors::row::Row;

/// Get Model's table name.
///
/// # Examples
///
/// ```
/// use arel::prelude::*;
///
/// #[arel::arel]
/// struct User {
///     id: i64,
/// }
/// assert_eq!(User::table_name(), "users");
/// #[arel::arel]
/// struct Order {
///     id: i64,
/// }
/// assert_eq!(Order::table_name(), "orders");
/// ```
#[async_trait::async_trait]
pub trait ArelAble: Sized + Send + Sync {
    // type PersistedRowRecord;

    fn id() -> &'static str { Self::primary_key() }
    fn primary_key() -> &'static str { "id" }
    fn locking_column() -> Option<&'static str> { None }
    fn get_persisted_locking_column_attr_value(&self) -> anyhow::Result<Option<i32>> {
        Err(anyhow::anyhow!("locking_version not support"))
    }
    fn set_locking_column_attr_value(&mut self, _locking_version: i32) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("locking_version not support"))
    }
    fn table_name() -> String {
        Table::<Self>::table_name()
    }
    fn table() -> Table<Self> {
        Table::<Self>::new()
    }
    fn query() -> Table<Self> {
        let mut table = Self::table();
        table.with_select_manager();
        table
    }
    fn lock() -> Table<Self> {
        let mut table = Self::query();
        table.lock();
        table
    }
    fn create(condition: Json) -> Table<Self> {
        let mut table = Self::table();
        table.with_insert_manager().create(condition);
        table
    }
    fn update_all(condition: Json) -> Table<Self> {
        let mut table = Self::table();
        table.with_update_manager().update_all(condition);
        table
    }
    fn delete_all(condition: Json) -> Table<Self> {
        let mut table = Self::table();
        table.with_delete_manager().r#where(condition);
        table
    }
    fn table_column_names() -> Vec<&'static str>;
    fn attr_names() -> Vec<&'static str>;
    fn attr_name_to_table_column_name<'a>(attr_name: &'a str) -> anyhow::Result<&'a str>;
    fn table_column_name_to_attr_name<'a>(table_column_name: &'a str) -> anyhow::Result<&'a str>;

    // validates
    fn validate(&self) -> anyhow::Result<()>;

    // === sqlx
    // provide in instance method
    // fn persisted_row_record(&self) -> Option<&Self::PersistedRowRecord>;
    fn attr_json(&self, attr: &str) -> Option<Json>;
    fn persisted_attr_json(&self, attr: &str) -> Option<Json>;
    fn changed_attrs_json(&self) -> anyhow::Result<Option<Json>>;
    fn assign_from_persisted_row_record(&mut self) -> anyhow::Result<&mut Self>;
    fn assign_to_persisted_row_record(&mut self) -> anyhow::Result<&mut Self>;
    #[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres", feature = "mssql"))]
    fn new_from_db_row(db_row: Row<Self>) -> anyhow::Result<Self>;
    #[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres", feature = "mssql"))]
    async fn fetch_one_with_executor<'c, E>(&mut self, executor: E) -> anyhow::Result<Self> where E: sqlx::Executor<'c, Database=sqlx::Any> {
        Self::query().fetch_one_with_executor(executor).await
    }
    #[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres", feature = "mssql"))]
    async fn fetch_one() -> anyhow::Result<Self> { Self::query().fetch_one().await }
    #[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres", feature = "mssql"))]
    async fn fetch_first_with_executor<'c, E>(&mut self, executor: E) -> anyhow::Result<Self> where E: sqlx::Executor<'c, Database=sqlx::Any> {
        Self::query().fetch_first_with_executor(executor).await
    }
    #[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres", feature = "mssql"))]
    async fn fetch_first() -> anyhow::Result<Self> { Self::query().fetch_first().await }
    #[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres", feature = "mssql"))]
    async fn fetch_last_with_executor<'c, E>(&mut self, executor: E) -> anyhow::Result<Self> where E: sqlx::Executor<'c, Database=sqlx::Any> {
        Self::query().fetch_last_with_executor(executor).await
    }
    #[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres", feature = "mssql"))]
    async fn fetch_last() -> anyhow::Result<Self> { Self::query().fetch_last().await }
    #[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres", feature = "mssql"))]
    async fn fetch_count_with_executor<'c, E>(&mut self, executor: E) -> anyhow::Result<i64>
        where E: sqlx::Executor<'c, Database = sqlx::Any> {
        Self::query().fetch_count_with_executor(executor).await
    }
    #[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres", feature = "mssql"))]
    async fn fetch_self_with_executor<'c, E>(&self, executor: E) -> anyhow::Result<Self> where E: sqlx::Executor<'c, Database=sqlx::Any> {
        let primary_key = Self::primary_key();
        let attr_primary_key = Self::table_column_name_to_attr_name(Self::primary_key())?;
        if let Some(attr_primary_key_json) = self.persisted_attr_json(attr_primary_key) {
            let mut map = serde_json::Map::new();
            map.insert(primary_key.to_string(), attr_primary_key_json);
            Self::query().r#where(serde_json::Value::Object(map)).fetch_one_with_executor(executor).await
        } else {
            Err(anyhow::anyhow!("model not persisted"))
        }
    }
    #[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres", feature = "mssql"))]
    async fn fetch_self(&self) -> anyhow::Result<Self> {
        let db_state = crate::visitors::get_db_state()?;
        self.fetch_self_with_executor(db_state.pool()).await
    }
    #[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres", feature = "mssql"))]
    async fn fetch_count() -> anyhow::Result<i64> { Self::query().fetch_count().await }
    #[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres", feature = "mssql"))]
    async fn save_with_executor<'c, E>(&mut self, executor: E) -> anyhow::Result<()> where E: sqlx::Executor<'c, Database=sqlx::Any>;
    #[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres", feature = "mssql"))]
    async fn save(&mut self) -> anyhow::Result<()>;
    #[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres", feature = "mssql"))]
    async fn delete_with_executor<'c, E>(&mut self, executor: E) -> anyhow::Result<sqlx::any::AnyQueryResult> where E: sqlx::Executor<'c, Database=sqlx::Any>;
    #[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres", feature = "mssql"))]
    async fn delete(&mut self) -> anyhow::Result<sqlx::any::AnyQueryResult>;
    #[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres", feature = "mssql"))]
    async fn transaction_start() -> anyhow::Result<sqlx::Transaction<'static, sqlx::Any>> {
        let db_state = crate::visitors::get_db_state()?;
        Ok(db_state.pool().begin().await?)
    }
    #[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres", feature = "mssql"))]
    async fn transaction_auto_commit<F: Send>(callback: F, tx: sqlx::Transaction<'static, sqlx::Any>) -> anyhow::Result<Option<Self>>
        where for<'c> F: FnOnce(&'c mut sqlx::Transaction<sqlx::Any>) -> Pin<Box<dyn Future<Output = anyhow::Result<Option<Self>>> + Send + 'c >> {
        crate::visitors::db_state::DbState::transaction_auto_commit::<Self, _>(callback, tx).await
    }
    #[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres", feature = "mssql"))]
    async fn with_transaction<F: Send>(callback: F) -> anyhow::Result<Option<Self>>
        where for<'c> F: FnOnce(&'c mut sqlx::Transaction<sqlx::Any>) -> Pin<Box<dyn Future<Output = anyhow::Result<Option<Self>>> + Send + 'c >>
    {
        let db_state = crate::visitors::get_db_state()?;
        db_state.with_transaction::<Self, _>(callback).await
    }
    #[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres", feature = "mssql"))]
    async fn lock_self_with_executor(&self, executor: &mut sqlx::Transaction<sqlx::Any>) -> anyhow::Result<()> {
        let primary_key = Self::primary_key();
        let attr_primary_key = Self::table_column_name_to_attr_name(Self::primary_key())?;
        if let Some(attr_primary_key_json) = self.persisted_attr_json(attr_primary_key) {
            let mut map = serde_json::Map::new();
            map.insert(primary_key.to_string(), attr_primary_key_json);
            Self::lock().r#where(serde_json::Value::Object(map)).limit(1).execute_with_executor(executor).await?;
            Ok(())
        }  else {
            Err(anyhow::anyhow!("model not persisted"))
        }
    }
    #[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres", feature = "mssql"))]
    async fn with_lock<F: Send>(&self, callback: F) -> anyhow::Result<Option<Self>>
        where for<'c> F: FnOnce(&'c mut sqlx::Transaction<sqlx::Any>) -> Pin<Box<dyn Future<Output = anyhow::Result<Option<Self>>> + Send + 'c >>
    {
        let mut tx = Self::transaction_start().await?;
        if let Err(e) = self.lock_self_with_executor(&mut tx).await {
            tx.rollback().await?;
            return Err(e)
        }
        Self::transaction_auto_commit(callback, tx).await
    }
    // async fn with_lock<F: Send + 'static>(&self, callback: F) -> anyhow::Result<()>
    //     where for<'c>
    //           F: FnOnce(&'c mut sqlx::Transaction<sqlx::Any>) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send + 'c >>,
    //           Self: Send
    // {
    //     let primary_key = Self::primary_key();
    //     let attr_primary_key = Self::table_column_name_to_attr_name(Self::primary_key())?;
    //     if let Some(attr_primary_key_json) = self.persisted_attr_json(attr_primary_key) {
    //         let mut map = serde_json::Map::new();
    //         map.insert(primary_key.to_string(), attr_primary_key_json);
    //         Self::with_transaction(|tx| Box::pin(async move {
    //             Self::lock().r#where(serde_json::Value::Object(map)).limit(1).execute_with_executor(&mut *tx).await?;
    //             callback(&mut *tx).await
    //         })).await
    //     } else {
    //         Err(anyhow::anyhow!("model not persisted"))
    //     }
    // }
}