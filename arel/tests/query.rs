#[cfg(feature = "mysql")]
#[cfg(test)]
mod query {
    use arel::prelude::*;
    use chrono::prelude::*;

    #[arel]
    struct User {
        id: i64,
    }

    #[test]
    fn test_select() {
        let sql: String = User::query().into();
        assert_eq!(sql, "SELECT `users`.* FROM `users`");
        let sql = User::query().distinct().to_sql_string().unwrap();
        assert_eq!(sql, "SELECT DISTINCT `users`.* FROM `users`");
        let sql = User::query().select(json!(["name", "age"])).to_sql_string().unwrap();
        assert_eq!(sql, "SELECT `users`.`name`, `users`.`age` FROM `users`");
        let sql = User::query().select(json!(["name", "age"])).distinct().to_sql_string().unwrap();
        assert_eq!(sql, "SELECT DISTINCT `users`.`name`, `users`.`age` FROM `users`");
        let sql = User::query().select(json!("name, age")).to_sql_string().unwrap();
        assert_eq!(sql, "SELECT name, age FROM `users`");
        let sql = User::query().select(json!("name, age")).distinct().to_sql_string().unwrap();
        assert_eq!(sql, "SELECT DISTINCT name, age FROM `users`");
        // count
        let sql = User::query().count().to_sql_string().unwrap();
        assert_eq!(sql, "SELECT COUNT(`users`.*) FROM `users`");
        // sum
        let sql = User::query().sum("price").to_sql_string().unwrap();
        assert_eq!(sql, "SELECT SUM(`users`.`price`) FROM `users`");
        // avg
        let sql = User::query().avg("price").to_sql_string().unwrap();
        assert_eq!(sql, "SELECT AVG(`users`.`price`) FROM `users`");
        // min
        let sql = User::query().min("price").to_sql_string().unwrap();
        assert_eq!(sql, "SELECT MIN(`users`.`price`) FROM `users`");
        // max
        let sql = User::query().max("price").to_sql_string().unwrap();
        assert_eq!(sql, "SELECT MAX(`users`.`price`) FROM `users`");
    }
    #[test]
    fn test_where() {
        let sql = User::query()
            .r#where(json!({"name": "Tom"}))
            .r#where(json!(["active = ?", true]))
            .r#where_not(json!({"status": [1, 2, 3]}))
            .where_between(json!({"created_at": ["2021-12-01 00:00:00", "2021-12-31 23:59:59"]}))
            .where_or(json!({"login": false, "phone": null}))
            .to_sql_string().unwrap();
        assert_eq!(sql, "SELECT `users`.* FROM `users` WHERE `users`.`name` = 'Tom' AND active = 1 AND `users`.`status` NOT IN (1, 2, 3) AND `users`.`created_at` BETWEEN '2021-12-01 00:00:00' AND '2021-12-31 23:59:59' AND (`users`.`login` = 0 OR `users`.`phone` IS NULL)");

        let sql = User::query()
            .r#where_not(json!({"name": "Tom", "status": [1, 2, 3]}))
            .r#where(json!(["active = ?", true]))
            .to_sql_string().unwrap();
        assert_eq!(sql, "SELECT `users`.* FROM `users` WHERE `users`.`name` != 'Tom' AND `users`.`status` NOT IN (1, 2, 3) AND active = 1");

        let sql = User::query()
            .joins(json!("left join orders on users.id = orders.user_id"))
            .r#where(json!({"name": "Tom"}))
            .to_sql_string().unwrap();
        assert_eq!(sql, "SELECT `users`.* FROM `users` left join orders on users.id = orders.user_id WHERE `users`.`name` = 'Tom'");

        // range
        let sql = User::query().where_range("age", 18..25).to_sql_string().unwrap();
        assert_eq!(sql, "SELECT `users`.* FROM `users` WHERE `users`.`age` >= 18 AND `users`.`age` < 25");
        let sql = User::query().where_range("age", 18..).to_sql_string().unwrap();
        assert_eq!(sql, "SELECT `users`.* FROM `users` WHERE `users`.`age` >= 18");
        let sql = User::query().where_range("age", ..=25).to_sql_string().unwrap();
        assert_eq!(sql, "SELECT `users`.* FROM `users` WHERE `users`.`age` <= 25");
        // range_between
        let sql = User::query().where_range("age", 18..=25).to_sql_string().unwrap();
        assert_eq!(sql, "SELECT `users`.* FROM `users` WHERE `users`.`age` BETWEEN 18 AND 25");

        let start = Utc.ymd(2021, 1, 1).and_hms(0, 0, 0);
        let end = Utc.ymd(2021, 12, 31).and_hms(23, 59, 59);
        let sql = User::query().where_range("created_at", start..=end).to_sql_string().unwrap();
        assert_eq!(sql, "SELECT `users`.* FROM `users` WHERE `users`.`created_at` BETWEEN '2021-01-01T00:00:00Z' AND '2021-12-31T23:59:59Z'");
    }
    #[test]
    fn test_lock() {
        let sql = User::lock().r#where(json!({"x": 1})).to_sql_string().unwrap();
        assert_eq!(sql, "SELECT `users`.* FROM `users` WHERE `users`.`x` = 1 FOR UPDATE");
    }
    #[test]
    fn test_group_having() {
        let sql = User::query().group(json!(["name", "email"])).group(json!("age")).to_sql_string().unwrap();
        assert_eq!(sql, "SELECT `users`.* FROM `users` GROUP BY `users`.`name`, `users`.`email`, age");

        let sql = User::query().group(json!("age"))
            .having_not(json!({"x": 1}))
            .having(json!(["y > ?", 2]))
            .having_range("z", 18..)
            .to_sql_string().unwrap();
        assert_eq!(sql, "SELECT `users`.* FROM `users` GROUP BY age HAVING `users`.`x` != 1 AND y > 2 AND `users`.`z` >= 18");
    }
    #[test]
    fn test_order() {
        let sql = User::query().order(json!({
            "name": "desc"
        })).order(json!("age ASC")).to_sql_string().unwrap();
        assert_eq!(sql, "SELECT `users`.* FROM `users` ORDER BY `users`.`name` DESC, age ASC");
    }
    #[test]
    fn test_limit_offset() {
        let sql = User::query().limit(10).to_sql_string().unwrap();
        assert_eq!(sql, "SELECT `users`.* FROM `users` LIMIT 10");
        let sql = User::query().offset(10).to_sql_string().unwrap();
        assert_eq!(sql, "SELECT `users`.* FROM `users` OFFSET 10");
        let sql = User::query().paginate(5, 10).to_sql_string().unwrap();
        assert_eq!(sql, "SELECT `users`.* FROM `users` LIMIT 10 OFFSET 40");
    }
}