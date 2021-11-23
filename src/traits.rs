use serde_json::{Value as Json, json};
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
    fn table_name() -> String {
        Table::<Self>::table_name()
    }
    fn table() -> Table<Self> {
        Table::<Self>::new()
    }
    fn r#where(condition: Json) -> Table<Self> {
        let mut table = Self::table();
        table.r#where(condition);
        table
    }
}