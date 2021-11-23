pub mod methods;
pub mod traits;
pub mod collectors;
pub mod nodes;
pub mod table;

pub use nodes::{SqlLiteral};

pub fn sql(sql_raw: &str) -> SqlLiteral {
    SqlLiteral::new(sql_raw.to_string())
}