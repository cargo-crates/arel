use arel::prelude::*;

#[arel::arel]
struct User {
    id: i64,
}

#[cfg(test)]
mod insert {
    use super::*;
    #[test]
    fn test_insert() {
        let sql = User::create(json!({
            "name": "Tom",
            "age": 18,
        })).to_sql().unwrap();
        assert_eq!(sql, "INSERT INTO `users` (`age`, `name`) VALUES (18, 'Tom')");
    }
}