#[cfg(feature = "mysql")]
#[cfg(test)]
mod delete {
    use arel::prelude::*;

    #[arel::arel]
    struct User {
        id: i64,
    }

    #[test]
    fn test_delete() {
        let sql = User::delete_all(json!({
                "name": "Tom",
            }))
            .where_range("age", 18..)
            .where_range("login_time", 0..=3)
            .order(json!("id desc"))
            .offset(1)
            .limit(5)
            .to_sql_string().unwrap();
        assert_eq!(sql, "DELETE FROM `users` WHERE `users`.`name` = 'Tom' AND `users`.`age` >= 18 AND `users`.`login_time` BETWEEN 0 AND 3 ORDER BY id desc LIMIT 5 OFFSET 1");
        let sql = User::query().delete_all(json!({"name": "Tom2"})).to_sql_string().unwrap();
        assert_eq!(sql, "DELETE FROM `users` WHERE `users`.`id` IN (SELECT `id` FROM (SELECT `users`.`id` FROM `users` WHERE `users`.`name` = 'Tom2') AS __arel_subquery_temp)");
        let sql = User::query().r#where(json!({"x": 1})).delete_all(json!({"name": "Tom"})).to_sql_string().unwrap();
        assert_eq!(sql, "DELETE FROM `users` WHERE `users`.`id` IN (SELECT `id` FROM (SELECT `users`.`id` FROM `users` WHERE `users`.`x` = 1 AND `users`.`name` = 'Tom') AS __arel_subquery_temp)");
    }
}