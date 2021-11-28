use arel::traits::ModelAble;
use serde_json::json;

#[derive(Clone, Debug)]
struct User {}

impl ModelAble for User {}

#[cfg(test)]
mod query {
    use super::*;
    #[test]
    fn test_query() {
        let sql = User::query()
            .r#where(json!({"name": "Tom"}))
            .r#where(json!(["active = ?", true]))
            .to_sql();
        assert_eq!(sql, "SELECT `users`.* FROM `users` WHERE `users`.`name` = 'Tom' AND active = 1");

        let sql = User::query()
            .joins(json!("left join orders on users.id = orders.user_id"))
            .r#where(json!({"name": "Tom"}))
            .to_sql();
        assert_eq!(sql, "SELECT `users`.* FROM `users` left join orders on users.id = orders.user_id WHERE `users`.`name` = 'Tom'");
    }
    #[test]
    fn test_update() {
        let sql = User::update_all(json!({
            "name": "Tom"
        })).to_sql();
        assert_eq!(sql, "UPDATE `users` SET `users`.`name` = 'Tom'");
    }
}