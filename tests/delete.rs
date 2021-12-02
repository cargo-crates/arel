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
        let sql = User::query().delete_all(json!({"name": "Tom2"})).to_sql();
        assert_eq!(sql, "DELETE FROM `users` WHERE `users`.`id` IN (SELECT `id` FROM (SELECT `users`.`id` FROM `users` WHERE `users`.`name` = 'Tom2') AS __arel_subquery_temp)");
        let sql = User::query().r#where(json!({"x": 1})).delete_all(json!({"name": "Tom"})).to_sql();
        assert_eq!(sql, "DELETE FROM `users` WHERE `users`.`id` IN (SELECT `id` FROM (SELECT `users`.`id` FROM `users` WHERE `users`.`x` = 1 AND `users`.`name` = 'Tom') AS __arel_subquery_temp)");
    }
}