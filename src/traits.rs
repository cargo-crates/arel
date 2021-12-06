use serde_json::{Value as Json};
use crate::table::Table;

/// Get Model's table name.
///
/// # Examples
///
/// ```
/// use arel::traits::ArelAble;
///
/// #[derive(Clone, Debug)]
/// struct User {}
/// impl ArelAble for User {}
/// assert_eq!(User::table_name(), "users");
/// struct Order {}
/// impl ArelAble for Order {}
/// assert_eq!(Order::table_name(), "orders");
/// ```
pub trait ArelAble: Sized {
    fn id() -> &'static str { Self::primary_key() }
    fn primary_key() -> &'static str { "id" }
    fn locking_column() -> &'static str { "lock_version" }
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
}