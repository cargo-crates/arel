use serde_json::{Value as Json};
use crate::table::Table;

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
pub trait ArelAble: Sized {
    type PersistedRowRecord;

    fn id() -> &'static str { Self::primary_key() }
    fn primary_key() -> &'static str { "id" }
    fn locking_column() -> &'static str { "lock_version" }
    fn table_name() -> String {
        Table::<Self>::table_name()
    }
    fn table_column_names() -> Vec<&'static str>;
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
    // sqlx
    fn persisted_row_record(&self) -> Option<&Self::PersistedRowRecord>;
    fn attr_json(&self, attr: &str) -> Option<Json>;
    fn persisted_attr_json(&self, attr: &str) -> Option<Json>;
    fn changed_attrs_json(&self) -> std::option::Option<Json>;
    fn assign_from_persisted_row_record(&mut self) -> anyhow::Result<&mut Self>;
    #[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres", feature = "mssql"))]
    fn new_from_db_row(db_row: sqlx::any::AnyRow) -> anyhow::Result<Self>;
    fn assign_to_persisted_row_record(&mut self) -> anyhow::Result<&mut Self>;
    #[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres", feature = "mssql"))]
    async fn save(&mut self) -> anyhow::Result<()>;
    #[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres", feature = "mssql"))]
    async fn delete(&mut self) -> anyhow::Result<sqlx::any::AnyQueryResult>;
    #[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres", feature = "mssql"))]
    async fn with_transaction() -> anyhow::Result<()> {
        let db_state = crate::visitors::get_db_state()?;
        db_state.transaction().await
    }
}