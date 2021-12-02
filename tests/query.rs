use arel::traits::ModelAble;
use serde_json::json;

#[derive(Clone, Debug)]
struct User {}

impl ModelAble for User {}

#[cfg(test)]
mod query {
    use super::*;
    #[test]
    fn test_select() {
        let sql = User::query().to_sql();
        assert_eq!(sql, "SELECT `users`.* FROM `users`");
        let sql = User::query().distinct().to_sql();
        assert_eq!(sql, "SELECT DISTINCT `users`.* FROM `users`");
        let sql = User::query().select(json!(["name", "age"])).to_sql();
        assert_eq!(sql, "SELECT `users`.`name`, `users`.`age` FROM `users`");
        let sql = User::query().select(json!(["name", "age"])).distinct().to_sql();
        assert_eq!(sql, "SELECT DISTINCT `users`.`name`, `users`.`age` FROM `users`");
        let sql = User::query().select(json!("name, age")).to_sql();
        assert_eq!(sql, "SELECT name, age FROM `users`");
        let sql = User::query().select(json!("name, age")).distinct().to_sql();
        assert_eq!(sql, "SELECT DISTINCT name, age FROM `users`");
        // count
        let sql = User::query().count().to_sql();
        assert_eq!(sql, "SELECT COUNT(`users`.*) FROM `users`");
        // sum
        let sql = User::query().sum("price").to_sql();
        assert_eq!(sql, "SELECT SUM(`users`.`price`) FROM `users`");
        // avg
        let sql = User::query().avg("price").to_sql();
        assert_eq!(sql, "SELECT AVG(`users`.`price`) FROM `users`");
        // min
        let sql = User::query().min("price").to_sql();
        assert_eq!(sql, "SELECT MIN(`users`.`price`) FROM `users`");
        // min
        let sql = User::query().max("price").to_sql();
        assert_eq!(sql, "SELECT MAX(`users`.`price`) FROM `users`");
    }
    #[test]
    fn test_where() {
        let sql = User::query()
            .r#where(json!({"name": "Tom"}))
            .r#where(json!(["active = ?", true]))
            .to_sql();
        assert_eq!(sql, "SELECT `users`.* FROM `users` WHERE `users`.`name` = 'Tom' AND (active = 1)");

        let sql = User::query()
            .r#where_not(json!({"name": "Tom", "status": [1, 2, 3]}))
            .r#where(json!(["active = ?", true]))
            .to_sql();
        assert_eq!(sql, "SELECT `users`.* FROM `users` WHERE `users`.`name` != 'Tom' AND `users`.`status` NOT IN (1, 2, 3) AND (active = 1)");

        let sql = User::query()
            .joins(json!("left join orders on users.id = orders.user_id"))
            .r#where(json!({"name": "Tom"}))
            .to_sql();
        assert_eq!(sql, "SELECT `users`.* FROM `users` left join orders on users.id = orders.user_id WHERE `users`.`name` = 'Tom'");
    }
    #[test]
    fn test_lock() {
        let sql = User::lock().r#where(json!({"x": 1})).to_sql();
        assert_eq!(sql, "SELECT `users`.* FROM `users` WHERE `users`.`x` = 1 FOR UPDATE");
    }
    #[test]
    fn test_group_having() {
        let sql = User::query().group(json!(["name", "email"])).group(json!("age")).to_sql();
        assert_eq!(sql, "SELECT `users`.* FROM `users` GROUP BY `users`.`name`, `users`.`email`, age");

        let sql = User::query().group(json!("age"))
            .having_not(json!({"x": 1}))
            .having(json!(["y > ?", 2])).to_sql();
        assert_eq!(sql, "SELECT `users`.* FROM `users` GROUP BY age HAVING `users`.`x` != 1 AND (y > 2)");
    }
    #[test]
    fn test_order() {
        let sql = User::query().order(json!({
            "name": "desc"
        })).order(json!("age ASC")).to_sql();
        assert_eq!(sql, "SELECT `users`.* FROM `users` ORDER BY `users`.`name` DESC, age ASC");
    }
    #[test]
    fn test_limit_offset() {
        let sql = User::query().limit(10).to_sql();
        assert_eq!(sql, "SELECT `users`.* FROM `users` LIMIT 10");
        let sql = User::query().offset(10).to_sql();
        assert_eq!(sql, "SELECT `users`.* FROM `users` OFFSET 10");
        let sql = User::query().paginate(5, 10).to_sql();
        assert_eq!(sql, "SELECT `users`.* FROM `users` LIMIT 10 OFFSET 40");
    }
}