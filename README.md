# Arel &emsp; [![Latest Version]][crates.io]

[Latest Version]: https://img.shields.io/crates/v/arel.svg
[crates.io]: https://crates.io/crates/arel

```rust
use arel::traits::ModelAble;
use serde_json::json;

#[derive(Clone, Debug)]
struct User {}
impl ModelAble for User {}

let sql = User::query().r#where(json!({"name": "Tom"})).r#where(json!(["active = ?", true])).to_sql();
assert_eq!(sql, "SELECT `users`.* FROM `users` WHERE `users`.`name` = 'Tom' AND active = 1");
```