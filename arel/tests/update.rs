#[cfg(feature = "mysql")]
#[cfg(test)]
mod update {
    use arel::prelude::*;

    #[arel::arel]
    struct User {
        id: i64,
    }

    #[test]
    fn test_update() {
        let sql = User::update_all(json!({"name": "Tom"}))
            .r#where(json!({"x": 1}))
            .where_range("age", ..18)
            .where_range("login_time", 0..=3)
            .order(json!("id desc"))
            .offset(1)
            .limit(5)
            .to_sql_string().unwrap();
        assert_eq!(sql, "UPDATE `users` SET `users`.`name` = 'Tom' WHERE `users`.`x` = 1 AND `users`.`age` < 18 AND `users`.`login_time` BETWEEN 0 AND 3 ORDER BY id desc LIMIT 5 OFFSET 1");

        let sql = User::query()
            .r#where(json!({"x": 1}))
            .update_all(json!({"name": "Tom"}))
            .to_sql_string().unwrap();
        assert_eq!(sql, "UPDATE `users` SET `users`.`name` = 'Tom' WHERE `users`.`id` IN (SELECT `id` FROM (SELECT `users`.`id` FROM `users` WHERE `users`.`x` = 1) AS __arel_subquery_temp)");

        let mut query = User::query();
        query.r#where(json!({"x": 1}));
        let sql = query.clone().update_all(json!({"name": "Tom"})).to_sql_string().unwrap();
        assert_eq!(sql, "UPDATE `users` SET `users`.`name` = 'Tom' WHERE `users`.`id` IN (SELECT `id` FROM (SELECT `users`.`id` FROM `users` WHERE `users`.`x` = 1) AS __arel_subquery_temp)");

        let sql = query.to_sql_string().unwrap();
        assert_eq!(sql, "SELECT `users`.* FROM `users` WHERE `users`.`x` = 1");


        let sql = User::table().increment("x", 2).r#where(json!({"id": 1})).to_sql_string().unwrap();
        assert_eq!(sql, "UPDATE `users` SET `users`.`x` = COALESCE(`users`.`x`, 0) + 2 WHERE `users`.`id` = 1");
        let sql = User::table().decrement("x", 2).r#where(json!({"id": 1})).to_sql_string().unwrap();
        assert_eq!(sql, "UPDATE `users` SET `users`.`x` = COALESCE(`users`.`x`, 0) - 2 WHERE `users`.`id` = 1");
    }
}