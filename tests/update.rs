use arel::traits::ModelAble;
use serde_json::json;

#[derive(Clone, Debug)]
struct User {}

impl ModelAble for User {}

#[cfg(test)]
mod update {
    use super::*;
    #[test]
    fn test_update() {
        let sql = User::update_all(json!({
                "name": "Tom"
            })).r#where(json!({
                "x": 1
            })).to_sql();
        assert_eq!(sql, "UPDATE `users` SET `users`.`name` = 'Tom' WHERE `users`.`x` = 1");

        let sql = User::query()
            .r#where(json!({
                "x": 1
            })).update_all(json!({
                "name": "Tom"
            })).to_sql();
        assert_eq!(sql, "UPDATE `users` SET `users`.`name` = 'Tom' WHERE `users`.`x` = 1");

        let mut query = User::query();
        query.r#where(json!({"x": 1}));
        let sql = query.clone().update_all(json!({
                "name": "Tom"
            })).to_sql();
        assert_eq!(sql, "UPDATE `users` SET `users`.`name` = 'Tom' WHERE `users`.`x` = 1");

        let sql = query.to_sql();
        assert_eq!(sql, "SELECT `users`.* FROM `users` WHERE `users`.`x` = 1");
    }
}