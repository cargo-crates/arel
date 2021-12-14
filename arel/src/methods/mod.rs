use std::any::type_name;
use inflector::{string::{demodulize, pluralize}, cases::snakecase};
use regex::Regex;
use crate::traits::ArelAble;

pub fn type_to_pluralize_string<M>() -> String where M: ?Sized {
    // eg: arel::UserTable
    let full_namespace = type_name::<M>();
    // eg: UserTable
    let struct_name = demodulize::demodulize(&full_namespace);
    // eg: user_table
    let snake_struct_name = snakecase::to_snake_case(&struct_name);
    // eg: user_tables
    pluralize::to_plural(Regex::new(r"_arel$").unwrap().replace(&snake_struct_name, "").as_ref())
}

/// Get Model's table field name.
///
/// # Examples
///
/// ```
/// use arel::prelude::*;
/// use arel::methods::table_column_name;
///
/// #[arel::arel]
/// struct User {
///     id: i64,
/// }
/// assert_eq!(table_column_name::<User>("*"), "`users`.*");
/// assert_eq!(table_column_name::<User>("age"), "`users`.`age`");
/// assert_eq!(table_column_name::<User>("users.name"), "users.name");
/// ```
pub fn table_column_name<M: ArelAble>(column_name: &str) -> String {
    if column_name == "*" {
        format!("`{}`.{}", M::table_name(), column_name)
    } else if Regex::new(r"\.").unwrap().is_match(column_name) {
        format!("{}", column_name)
    } else {
        format!("`{}`.`{}`", M::table_name(), column_name)
    }
}

pub fn quote_table_name(table_name: &str) -> String {
    format!("`{}`", table_name)
}