use std::any::type_name;
use inflector::{string::{demodulize, pluralize}, cases::snakecase};
use regex::Regex;
use crate::traits::ModelAble;

pub fn type_to_pluralize_string<M>() -> String where M: ?Sized {
    // eg: arel::UserTable
    let full_namespace = type_name::<M>();
    // eg: UserTable
    let struct_name = demodulize::demodulize(&full_namespace);
    // eg: user_table
    let snake_struct_name = snakecase::to_snake_case(&struct_name);
    // eg: user_tables
    pluralize::to_plural(&snake_struct_name)
}

/// Get Model's table field name.
///
/// # Examples
///
/// ```
/// use arel::traits::ModelAble;
/// use arel::methods::table_column_name;
///
/// #[derive(Clone, Debug)]
/// struct User {}
/// impl ModelAble for User {}
/// assert_eq!(table_column_name::<User>("*"), "`users`.*");
/// assert_eq!(table_column_name::<User>("age"), "`users`.`age`");
/// assert_eq!(table_column_name::<User>("users.name"), "users.name");
/// ```
pub fn table_column_name<M: ModelAble>(column_name: &str) -> String {
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