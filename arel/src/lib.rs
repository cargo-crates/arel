pub mod methods;
pub mod traits;
pub mod collectors;
pub mod nodes;
pub mod statements;
pub mod table;
pub mod visitors;

pub use nodes::{SqlLiteral};

pub fn sql(sql_raw: &str) -> SqlLiteral {
    SqlLiteral::new(sql_raw.to_string())
}
pub use traits::ArelAble;

pub use arel_macro::arel;

pub use async_trait;
pub use serde_json;
pub use anyhow;
pub use sqlx;
pub mod prelude;