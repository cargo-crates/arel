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

---


### Query

<details>
<summary>select</summary>

```rust
let sql = User::query().to_sql();
assert_eq!(sql, "SELECT `users`.* FROM `users`");
let sql = User::query().select(json!(["name", "age"])).to_sql();
assert_eq!(sql, "SELECT `users`.`name`, `users`.`age` FROM `users`");
```
</details>

<details>
<summary>distinct & count & sum & avg & min & max</summary>

```rust
// distinct
let sql = User::query().distinct().to_sql();
assert_eq!(sql, "SELECT DISTINCT `users`.* FROM `users`");
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
// max
let sql = User::query().max("price").to_sql();
assert_eq!(sql, "SELECT MAX(`users`.`price`) FROM `users`");
```
</details>

<details>
<summary>where</summary>

```rust
let sql = User::query()
.r#where(json!({"name": "Tom"}))
.r#where(json!(["active = ?", true]))
.to_sql();
assert_eq!(sql, "SELECT `users`.* FROM `users` WHERE `users`.`name` = 'Tom' AND (active = 1)");
// where_not
let sql = User::query()
.r#where_not(json!({"name": "Tom", "status": [1, 2, 3]}))
.r#where(json!(["active = ?", true]))
.to_sql();
assert_eq!(sql, "SELECT `users`.* FROM `users` WHERE `users`.`name` != 'Tom' AND `users`.`status` NOT IN (1, 2, 3) AND (active = 1)");
```
</details>

<details>
<summary>joins</summary>

```rust
let sql = User::query()
.joins(json!("left join orders on users.id = orders.user_id"))
.r#where(json!({"name": "Tom"}))
.to_sql();
assert_eq!(sql, "SELECT `users`.* FROM `users` left join orders on users.id = orders.user_id WHERE `users`.`name` = 'Tom'");
```
</details>

<details>
<summary>lock</summary>

```rust
let sql = User::lock().r#where(json!({"x": 1})).to_sql();
assert_eq!(sql, "SELECT `users`.* FROM `users` WHERE `users`.`x` = 1 FOR UPDATE");
```
</details>

<details>
<summary>group & having</summary>

```rust
let sql = User::query().group(json!(["name", "email"])).group(json!("age")).to_sql();
assert_eq!(sql, "SELECT `users`.* FROM `users` GROUP BY `users`.`name`, `users`.`email`, age");

let sql = User::query().group(json!("age"))
.having_not(json!({"x": 1}))
.having(json!(["y > ?", 2])).to_sql();
assert_eq!(sql, "SELECT `users`.* FROM `users` GROUP BY age HAVING `users`.`x` != 1 AND (y > 2)");
```
</details>

<details>
<summary>order</summary>

```rust
let sql = User::query().order(json!({
            "name": "desc"
        })).order(json!("age ASC")).to_sql();
assert_eq!(sql, "SELECT `users`.* FROM `users` ORDER BY `users`.`name` DESC, age ASC");
```
</details>

<details>
<summary>limit & offset</summary>

```rust
let sql = User::query().limit(10).to_sql();
assert_eq!(sql, "SELECT `users`.* FROM `users` LIMIT 10");
let sql = User::query().offset(10).to_sql();
assert_eq!(sql, "SELECT `users`.* FROM `users` OFFSET 10");
let sql = User::query().paginate(5, 10).to_sql();
assert_eq!(sql, "SELECT `users`.* FROM `users` LIMIT 10 OFFSET 40");
```
</details>

### Insert

```rust
let sql = User::create(json!({
            "name": "Tom",
            "age": 18,
        })).to_sql();
assert_eq!(sql, "INSERT INTO `users` (`age`, `name`) VALUES (18, 'Tom')");
```

### Update

```rust
let sql = User::update_all(json!({
                "name": "Tom"
            })).r#where(json!({
                "x": 1
            })).to_sql();
assert_eq!(sql, "UPDATE `users` SET `users`.`name` = 'Tom' WHERE `users`.`x` = 1");
```

### Delete

```rust
let sql = User::delete_all(json!({
            "name": "Tom",
            "age": 18,
        })).order(json!("id desc")).offset(1).limit(5).to_sql();
assert_eq!(sql, "DELETE FROM `users` WHERE `users`.`age` = 18 AND `users`.`name` = 'Tom' ORDER BY id desc LIMIT 5 OFFSET 1");
```