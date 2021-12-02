use arel::traits::ModelAble;
use serde_json::json;

#[derive(Clone, Debug)]
struct User {}

impl ModelAble for User {}

#[cfg(test)]
mod insert {
    use super::*;
    #[test]
    fn test_insert() {
        let sql = User::create(json!({
            "name": "Tom",
            "age": 18,
        })).to_sql();
        assert_eq!(sql, "INSERT INTO `users` (`age`, `name`) VALUES (18, 'Tom')");
    }
}