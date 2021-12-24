#[cfg(test)]
mod insert {
    use arel::prelude::*;

    #[arel::arel]
    struct User {
        id: i64,
    }

    #[test]
    #[cfg(feature = "mysql")]
    fn test_insert_mysql() {
        let sql = User::create(json!({})).to_sql_string().unwrap();
        assert_eq!(sql, "INSERT INTO `users` VALUES ()");

        let sql = User::create(json!({
            "name": "Tom",
            "age": 18,
        })).to_sql_string().unwrap();
        #[cfg(feature = "mysql")]
        assert_eq!(sql, "INSERT INTO `users` (`age`, `name`) VALUES (18, 'Tom')");
    }

    #[test]
    #[cfg(feature = "sqlite")]
    fn test_insert_sqlite() {
        let sql = User::create(json!({})).to_sql_string().unwrap();
        assert_eq!(sql, "INSERT INTO \"users\" DEFAULT VALUES");
    }
}