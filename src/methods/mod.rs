use std::any::type_name;
use inflector::{string::{demodulize, pluralize}, cases::snakecase};

pub fn table_name<M>() -> String where M: ?Sized {
    // eg: arel::UserTable
    let full_namespace = type_name::<M>();
    // eg: UserTable
    let struct_name = demodulize::demodulize(&full_namespace);
    // eg: user_table
    let snake_struct_name = snakecase::to_snake_case(&struct_name);
    // eg: user_tables
    pluralize::to_plural(&snake_struct_name)
}