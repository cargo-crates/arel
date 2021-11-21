#[derive(Clone, Debug)]
pub struct SqlLiteral {
    pub raw_sql: String
}

impl SqlLiteral {
    pub fn new(raw_sql: &str) -> Self {
        Self {
            raw_sql: raw_sql.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let sql_literal = SqlLiteral::new("length(title)");
        assert_eq!(sql_literal.raw_sql, "length(title)");
    }
}