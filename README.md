# Arel &emsp; 
[![ci](https://github.com/cargo-crates/arel/workflows/Rust/badge.svg)](https://github.com/cargo-crates/arel/actions)
[![Latest Version]][crates.io]
![downloads](https://img.shields.io/crates/d/arel.svg?style=flat-square)

[Latest Version]: https://img.shields.io/crates/v/arel.svg
[crates.io]: https://crates.io/crates/arel

* Install
```Cargo.toml
# db features: sqlite|mysql|postgres|mssql
arel = { version = "0.1", features = ["sqlite"]}
```

* Demo
```rust
use arel::prelude::*;
use chrono::{TimeZone};

#[arel(table_name="users", primary_key="id")]
struct User {
    id: Option<i64>,
    name: String,
    #[arel(table_column_name="type")]
    r#type: Option<i32>,
    #[arel(table_column_name="desc")]
    desc2: String,
    expired_at: chrono::DateTime<chrono::Utc>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_state = arel::visitors::get_or_init_db_state(|| Box::pin(async {
        sqlx::any::AnyPoolOptions::new().max_connections(5).connect("sqlite::memory:").await
    })).await?;

    let sql: String = User::query()
        .r#where(json!({"name": "Tom"})) // use .where_prepare can prevent sql injection  (sqlx .bind())
        .r#where(json!(["active = ?", true])) // can prevent sql injection  (sqlx .bind())
        .where_not(json!({"status": [1, 2, 3]}))
        .where_or(json!({"login": false, "phone": null}))
        .where_between(json!({"age": [18, 35]}))
        .where_range("expired_at", ..=chrono::Utc.ymd(2021, 12, 31).and_hms(23, 59, 59))
        .distinct()
        .into();
    assert_eq!(sql, "SELECT DISTINCT `users`.* FROM `users` WHERE `users`.`name` = 'Tom' AND active = 1 AND `users`.`status` NOT IN (1, 2, 3) AND (`users`.`login` = 0 OR `users`.`phone` IS NULL) AND `users`.`age` BETWEEN 18 AND 35 AND `users`.`expired_at` <= '2021-12-31T23:59:59Z'");

    // query batch vec<User>
    let users = User::query().fetch_all().await?;
    println!("users: {:#?}", users);
    // update batch
    User::update_all(json!({"name": "update_1"})).execute().await?;
    // delete batch
    User::delete_all(json!(["id > ?", 5])).execute().await?;
    
    // query one User
    let user = User::query().fetch_one().await?;
    println!("user: {:#?}", user);
    // create one
    let mut user = User::new();
    user.set_name("lily".to_string()).save().await?;
    println!("user: {:#?}", user);
    // update one
    let mut user = User::query().fetch_one().await?;
    user.set_name("Tom".to_string()).save().await?;
    println!("user: {:#?}", user);
    // delete one
    let mut user = User::query().fetch_one().await?;
    let result = user.delete().await?;
    println!("{:?}", result);

    // Transaction Support 
    User::with_transaction(|tx| Box::pin(async {
        let mut u1 = User::query().fetch_one_with_executor(&mut *tx).await?;
        let mut u2 = User::query().fetch_last_with_executor(&mut *tx).await?;
        u1.set_name("tx1".to_string()).save_with_executor(&mut *tx).await?;
        u2.set_name("tx2".to_string()).save_with_executor(&mut *tx).await?;
        Ok(None)
    })).await?;

    // With Lock In Transaction Support
    let mut u1 = User::query().fetch_one().await?;
    let mut u2 = User::query().fetch_one().await?;
    u1.clone().with_lock(|tx| Box::pin(async move {
        u1.set_name("with_lock1".to_string()).save_with_executor(&mut *tx).await?;
        u2.set_name("with_lock2".to_string()).save_with_executor(&mut *tx).await?;
        Ok(None)
    })).await?;
    
    Ok(())
}
```

---


### Query

<details>
<summary>select</summary>

```rust
let sql = User::query().to_sql_string().unwrap();
assert_eq!(sql, "SELECT `users`.* FROM `users`");
let sql = User::query().select(json!(["name", "age"])).to_sql_string().unwrap();
assert_eq!(sql, "SELECT `users`.`name`, `users`.`age` FROM `users`");
```
</details>

<details>
<summary>distinct & count & sum & avg & min & max</summary>

```rust
// distinct
let sql = User::query().distinct().to_sql_string().unwrap();
assert_eq!(sql, "SELECT DISTINCT `users`.* FROM `users`");
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
```
</details>

<details>
<summary>where</summary>

```rust
let sql = User::query()
.r#where(json!({"name": "Tom"}))
.r#where(json!(["active = ?", true]))
.to_sql_string().unwrap();
assert_eq!(sql, "SELECT `users`.* FROM `users` WHERE `users`.`name` = 'Tom' AND (active = 1)");
// where_not
let sql = User::query()
.r#where_not(json!({"name": "Tom", "status": [1, 2, 3]}))
.r#where(json!(["active = ?", true]))
.to_sql_string().unwrap();
assert_eq!(sql, "SELECT `users`.* FROM `users` WHERE `users`.`name` != 'Tom' AND `users`.`status` NOT IN (1, 2, 3) AND (active = 1)");
// range
let sql = User::query().where_range("age", 18..25).to_sql_string().unwrap();
assert_eq!(sql, "SELECT `users`.* FROM `users` WHERE (`users`.`age` >= 18 AND `users`.`age` < 25)");
// range_between
let sql = User::query().where_range_between("age", 18..25).to_sql_string().unwrap();
assert_eq!(sql, "SELECT `users`.* FROM `users` WHERE `users`.`age` BETWEEN 18 AND 25");
```
</details>

<details>
<summary>joins</summary>

```rust
let sql = User::query()
.joins(json!("left join orders on users.id = orders.user_id"))
.r#where(json!({"name": "Tom"}))
.to_sql_string().unwrap();
assert_eq!(sql, "SELECT `users`.* FROM `users` left join orders on users.id = orders.user_id WHERE `users`.`name` = 'Tom'");
```
</details>

<details>
<summary>lock</summary>

```rust
let sql = User::lock().r#where(json!({"x": 1})).to_sql_string().unwrap();
assert_eq!(sql, "SELECT `users`.* FROM `users` WHERE `users`.`x` = 1 FOR UPDATE");
```
</details>

<details>
<summary>group & having</summary>

```rust
let sql = User::query().group(json!(["name", "email"])).group(json!("age")).to_sql_string().unwrap();
assert_eq!(sql, "SELECT `users`.* FROM `users` GROUP BY `users`.`name`, `users`.`email`, age");

let sql = User::query().group(json!("age"))
    .having_not(json!({"x": 1}))
    .having(json!(["y > ?", 2]))
    .having_range("z", 18..)
    .to_sql_string().unwrap();
assert_eq!(sql, "SELECT `users`.* FROM `users` GROUP BY age HAVING `users`.`x` != 1 AND y > 2 AND `users`.`z` >= 18");
```
</details>

<details>
<summary>order</summary>

```rust
let sql = User::query().order(json!({
            "name": "desc"
        })).order(json!("age ASC")).to_sql_string().unwrap();
assert_eq!(sql, "SELECT `users`.* FROM `users` ORDER BY `users`.`name` DESC, age ASC");
```
</details>

<details>
<summary>limit & offset</summary>

```rust
let sql = User::query().limit(10).to_sql_string().unwrap();
assert_eq!(sql, "SELECT `users`.* FROM `users` LIMIT 10");
let sql = User::query().offset(10).to_sql_string().unwrap();
assert_eq!(sql, "SELECT `users`.* FROM `users` OFFSET 10");
let sql = User::query().paginate(5, 10).to_sql_string().unwrap();
assert_eq!(sql, "SELECT `users`.* FROM `users` LIMIT 10 OFFSET 40");
```
</details>

### Insert

```rust
let sql = User::create(json!({
            "name": "Tom",
            "age": 18,
        })).to_sql_string().unwrap();
assert_eq!(sql, "INSERT INTO `users` (`age`, `name`) VALUES (18, 'Tom')");
```

### Update

<details>
<summary>increment & decrement</summary>

```rust
// increment
let sql = User::table().increment("x", 2).r#where(json!({"id": 1})).to_sql_string().unwrap();
assert_eq!(sql, "UPDATE `users` SET `users`.`x` = COALESCE(`users`.`x`, 0) + 2 WHERE `users`.`id` = 1");
// decrement
let sql = User::table().decrement("x", 2).r#where(json!({"id": 1})).to_sql_string().unwrap();
assert_eq!(sql, "UPDATE `users` SET `users`.`x` = COALESCE(`users`.`x`, 0) - 2 WHERE `users`.`id` = 1");
```
</details>

```rust
let sql = User::update_all(json!({
                "name": "Tom"
            })).r#where(json!({
                "x": 1
            })).to_sql_string().unwrap();
assert_eq!(sql, "UPDATE `users` SET `users`.`name` = 'Tom' WHERE `users`.`x` = 1");
```

### Delete

```rust
let sql = User::delete_all(json!({
            "name": "Tom",
            "age": 18,
        })).order(json!("id desc")).offset(1).limit(5).to_sql_string().unwrap();
assert_eq!(sql, "DELETE FROM `users` WHERE `users`.`age` = 18 AND `users`.`name` = 'Tom' ORDER BY id desc LIMIT 5 OFFSET 1");
```

---

### Transaction

```rust
let mut u1 = User::query().fetch_one().await?;
// if u1 should move to closure, please use with_transaction replaced, (prevent clone u1)
let mut u1 = u1.clone().with_lock(|tx| Box::pin(async move {
    u1.set_desc2("with_lock1".to_string());
    u1.save_with_executor(&mut *tx).await?;
    Ok(Some(u1))
})).await.unwrap();
println!("{:?}", u1);

let tx = User::transaction_start().await?;
let u1 = User::transaction_auto_commit(|tx| Box::pin(async move {
    u1.lock_self_with_executor(&mut *tx).await?;
    u1.set_desc2("with_lock1".to_string());
    u1.save_with_executor(&mut *tx).await?;
    Ok(Some(u1))
}), tx).await?;
println!("{:?}", u1);

let mut u1 = User::query().fetch_one().await?;
let u1 = User::with_transaction(|tx| Box::pin(async move {
    u1.lock_self_with_executor(&mut *tx).await?;
    let mut u2 = User::query().fetch_last_with_executor(&mut *tx).await?;
    u1.set_desc2("tx1".to_string());
    u2.set_desc2("tx2".to_string());
    u1.save_with_executor(&mut *tx).await?;
    u2.save_with_executor(&mut *tx).await?;
    Ok(Some(u1))
})).await?;
println!("{:?}", u1);
```

If you wanna support Optimistic Lock please provide [locking_column](https://github.com/cargo-crates/arel/blob/b2185f34f6897f04fb774ccfa58594b2b71fa1f7/arel/tests/visitors/sqlite_sqlx/mod.rs#L4) attribute 

--- 

### Association 
Supports: `belongs_to`, `has_one`, `has_many`, `has_and_belongs_to_many`

look at [test code](https://github.com/cargo-crates/arel/blob/main/arel/tests/visitors/sqlite_sqlx/sqlite_sqlx_association.rs)