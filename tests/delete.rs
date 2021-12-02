use arel::traits::ModelAble;
use serde_json::json;

#[derive(Clone, Debug)]
struct User {}

impl ModelAble for User {}

#[cfg(test)]
mod delete {
    use super::*;
    #[test]
    fn test_delete() {
        let sql = User::delete_all(json!({
            "name": "Tom",
            "age": 18,
        })).order(json!("id desc")).offset(1).limit(5).to_sql();
        assert_eq!(sql, "DELETE FROM `users` WHERE `users`.`age` = 18 AND `users`.`name` = 'Tom' ORDER BY id desc LIMIT 5 OFFSET 1");
    }
}