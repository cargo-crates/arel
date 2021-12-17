use serde_json::{Value as Json};

#[derive(Clone, Debug)]
pub struct SqlLiteral {
    pub raw_sql: String,
    pub prepare_value: Option<Vec<Json>>
}

impl SqlLiteral {
    pub fn new(raw_sql: String) -> Self {
        Self {
            raw_sql,
            prepare_value: None,
        }
    }
    pub fn new_with_prepare(raw_sql: String, prepare_value: Vec<Json>) -> Self {
        let mut sql_literal = Self::new(raw_sql);
        sql_literal.prepare_value = Some(prepare_value);
        sql_literal
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn it_works() {
//         let sql_literal = SqlLiteral::new("length(title)".to_string());
//         assert_eq!(sql_literal.raw_sql, "length(title)".to_string());
//     }
// }