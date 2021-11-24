use serde_json::{Value as Json, json};
use crate::table::Table;
use regex::Regex;

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
    /// Get Model's table field name.
    ///
    /// # Examples
    ///
    /// ```
    /// use arel::traits::ModelAble;
    ///
    /// #[derive(Clone, Debug)]
    /// struct User {}
    /// impl ModelAble for User {}
    /// assert_eq!(User::table_column_name("*"), "`users`.*");
    /// assert_eq!(User::table_column_name("age"), "`users`.`age`");
    /// assert_eq!(User::table_column_name("users.name"), "users.name");
    /// ```
    fn table_column_name(column_name: &str) -> String {
        if column_name == "*" {
            format!("`{}`.{}", Self::table_name(), column_name)
        } else if Regex::new(r"\.").unwrap().is_match(column_name) {
            format!("{}", column_name)
        } else {
            format!("`{}`.`{}`", Self::table_name(), column_name)
        }
    }
    fn r#where(condition: Json) -> Table<Self> {
        let mut table = Self::table();
        table.r#where(condition);
        table
    }
}