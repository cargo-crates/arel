use arel::traits::ModelAble;
use serde_json::json;

#[derive(Clone, Debug)]
struct User {}

impl ModelAble for User {}

#[cfg(test)]
mod query {
    use super::*;
    #[test]
    fn test_where() {
        let sql = User::query().r#where(json!({"name": "Tom"})).r#where(json!(["active = ?", true])).to_sql();
        assert_eq!(sql, "SELECT `users`.* FROM `users` WHERE `users`.`name` = 'Tom' AND active = 1");
    }
}