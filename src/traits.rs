use serde_json::{Value as Json};
use crate::table::Table;

/// Get Model's table name.
///
/// # Examples
///
/// ```
/// use arel::traits::ModelAble;
///
/// #[derive(Clone, Debug)]
/// struct User {}
/// impl ModelAble for User {}
/// assert_eq!(User::table_name(), "users");
/// struct Order {}
/// impl ModelAble for Order {}
/// assert_eq!(Order::table_name(), "orders");
/// ```
pub trait ModelAble: Sized {
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
    fn update_all(condition: Json) -> Table<Self> {
        let mut table = Self::table();
        table.with_update_manager().update_all(condition);
        table
    }
}