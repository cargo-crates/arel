use arel::prelude::*;
use chrono::{TimeZone};

#[arel(
    primary_key="id",
    has_and_belongs_to_many("roles", struct = "Role", join_table = "users_roles", foreign_key = "user_id", association_foreign_key = "role_id"),
    has_many("role_admins", struct = "RoleAdmin", through = "roles"),
)]
struct User {
    #[arel(table_column_name="id")]
    uid: Option<i64>,
    admin_id: Option<i64>,
    #[arel(table_column_name="desc")]
    desc2: String,
    done: Option<bool>,
    #[arel(table_column_name="type")]
    r#type: Option<i32>,
    expired_at: chrono::DateTime<chrono::Utc>,
}

#[arel(
has_many("role_admins", struct = "RoleAdmin")
)]
struct Role {
    id: Option<i32>,
}

#[arel]
struct RoleAdmin {
    id: Option<i32>,
    role_id: Option<i32>,
}

async fn init_db() -> anyhow::Result<()> {
    let db_state = arel::visitors::get_or_init_db_state(|| Box::pin(async {
        sqlx::any::AnyPoolOptions::new().max_connections(5).connect("sqlite::memory:").await
    })).await?;
    sqlx::query("CREATE TABLE IF NOT EXISTS users
            (
                id          INTEGER PRIMARY KEY NOT NULL,
                desc TEXT                NOT NULL,
                done        BOOLEAN             NOT NULL DEFAULT 0,
                expired_at   DATETIME NOT NULL,
                admin_id    INTEGER DEFAULT 0
            );"
    ).execute(db_state.pool()).await?;
    for i in 0..10 {
        User::create(json!({
                "desc": format!("test-{}", i),
                "expired_at": "2021-12-31 23:59:59"
            })).execute().await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    if init_db().await.is_ok() {
        println!("ok");
    }


    let u1 = User::query().fetch_one().await?;

    println!("-{}", u1.role_admins()?.to_sql_string()?);
    println!("={}", User::role_admins_join_string());

    println!("-{}", u1.roles()?.to_sql_string()?);
    println!("={}", User::roles_join_string());
    // println!("={}", User::order_shop_products_join_string());
    // println!("-{}", User::order_shops_join_string());
    // println!("-{}", u1.wallet()?.to_sql_string()?);
    // println!("-{}", u1.admin()?.to_sql_string()?);

    // let mut u1 = User::query().fetch_one().await?;
    // let mut u1 = u1.clone().with_lock(|tx| Box::pin(async move {
    //     u1.set_desc2("with_lock1".to_string());
    //     u1.save_with_executor(&mut *tx).await?;
    //     Ok(Some(u1))
    // })).await?.unwrap();
    // println!("{:?}", u1);
    //
    // let tx = User::transaction_start().await?;
    // let u1 = User::transaction_auto_commit(|tx| Box::pin(async move {
    //     u1.lock_self_with_executor(tx).await?;
    //     u1.set_desc2("with_lock1".to_string());
    //     u1.save_with_executor(&mut *tx).await?;
    //     Ok(Some(u1))
    // }), tx).await?.unwrap();
    // println!("{:?}", u1);
    //
    // let mut u1 = User::query().fetch_one().await?;
    // let u1 = User::with_transaction(|tx| Box::pin(async move {
    //     u1.lock_self_with_executor(tx).await?;
    //     let mut u2 = User::query().fetch_last_with_executor(&mut *tx).await?;
    //     u1.set_desc2("tx1".to_string());
    //     u2.set_desc2("tx2".to_string());
    //     u1.save_with_executor(&mut *tx).await?;
    //     u2.save_with_executor(&mut *tx).await?;
    //     Ok(Some(u1))
    // })).await?.unwrap();
    // println!("{:?}", u1);

    //
    // let u1 = User::query().fetch_one().await?;
    // println!("u1: {:?}", u1);
    // assert_eq!(u1.desc2(), Some(&"tx1".to_string()));
    // let u2 = User::query().fetch_last().await?;
    // println!("u2: {:?}", u2);
    // assert_eq!(u2.desc2(), Some(&"tx2".to_string()));



    let expired_at = chrono::Utc.ymd(2021, 12, 31).and_hms(23, 59, 59);
    println!("{}", expired_at);

    // let mut user = User::new();
    // user.set_desc2("create desc".to_string())
    //     .set_expired_at(expired_at.clone())
    //     .save().await?;

    // let count = User::query().where_range("expired_at", ..=expired_at).fetch_count().await?;
    // println!("{}", count);

    // let mut user = User::query().r#where(json!(["id = ?", 1])).fetch_all().await?;
    // println!("--- find {:#?}", user);
    //
    // user.set_desc2("hello world".to_string()).save().await?;
    // println!("--- update {:#?}", user);

    // let mut user = User::new();
    // user.set_desc2("hello world2".to_string()).save().await?;
    // println!("--- create {:#?}", user);


    // let users = User::query().r#where(json!({"id": 1})).select(json!(["id"])).fetch_all().await?;
    // // println!("--- find {:#?}, {:?}", users, User::table_column_names());
    //
    // let result = User::update_all(json!({"desc": "update_1"})).execute().await?;
    // println!("===={:?}", result);


    // let mut user = User::query().fetch_one().await?;
    // let result = user.delete().await?;
    // println!("{:?}", result);

    Ok(())
}

