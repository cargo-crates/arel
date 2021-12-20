use arel::prelude::*;
use chrono::{TimeZone};

#[arel::arel]
struct User {
    id: i64,
    desc: String,
    done: Option<bool>,
    expired_at: chrono::DateTime<chrono::Utc>,
}

async fn init_db() -> anyhow::Result<()> {
    let db_state = arel::visitors::get_or_init_db_state(|| sqlx::any::AnyPoolOptions::new().max_connections(5).connect("sqlite::memory:")).await?;
    sqlx::query("CREATE TABLE IF NOT EXISTS users
            (
                id          INTEGER PRIMARY KEY NOT NULL,
                desc TEXT                NOT NULL,
                done        BOOLEAN             NOT NULL DEFAULT 0,
                expired_at   DATETIME NOT NULL
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

    let expired_at = chrono::Utc.ymd(2021, 12, 31).and_hms(23, 59, 59);
    let count = User::query().where_range("expired_at", ..=expired_at).fetch_count().await?;
    println!("{}", count);

    // let mut user = User::query().r#where(json!(["id = ?", 1])).fetch_all().await?;
    // println!("--- find {:#?}", user);
    //
    // user.set_desc("hello world".to_string()).save().await?;
    // println!("--- update {:#?}", user);

    // let mut user = User::new();
    // user.set_desc("hello world2".to_string()).save().await?;
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

