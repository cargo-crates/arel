
#[cfg(test)]
#[cfg(feature = "sqlite")]
mod sqlite_sqlx {
    use arel::prelude::*;
    use chrono::{TimeZone};

    #[arel(primary_key="id")]
    struct User {
        #[arel(table_column_name="id")]
        uid: Option<i64>,
        #[arel(table_column_name="desc")]
        desc2: String,
        #[arel(table_column_name="type")]
        r#type: Option<i32>,
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
            sqlx::query(&User::create(json!({
                "desc": format!("test-{}", i),
                "expired_at": "2021-01-01 00:00:00"
            })).to_sql_string()?).execute(db_state.pool()).await?;
        }

        Ok(())
    }

    async fn main_test() -> anyhow::Result<()> {
        init_db().await?;

        assert_eq!(User::table_column_names(), vec!["id", "desc", "type", "done", "expired_at"]);
        assert_eq!(User::attr_names(), vec!["uid", "desc2", "r#type", "done", "expired_at"]);
        assert_eq!(User::attr_name_to_table_column_name("uid")?, "id");
        assert_eq!(User::attr_name_to_table_column_name("desc2")?, "desc");
        assert_eq!(User::attr_name_to_table_column_name("r#type")?, "type");
        assert_eq!(User::table_column_name_to_attr_name("id")?, "uid");
        assert_eq!(User::table_column_name_to_attr_name("desc")?, "desc2");
        assert_eq!(User::table_column_name_to_attr_name("type")?, "r#type");
        // assert_eq!(User::uid_table_column_name(), "id");
        // assert_eq!(User::desc_table_column_name(), "desc");
        // assert_eq!(User::done_table_column_name(), "done");
        // assert_eq!(User::expired_at_table_column_name(), "expired_at");

        let count = User::fetch_count().await?;
        assert_eq!(count, 10);

        // query all
        let users = User::query().fetch_all().await?;
        assert_eq!(users.len(), 10);

        // update batch
        User::update_all(json!({"desc": "update_1"})).execute().await?;
        let user = User::query().fetch_one().await?;
        assert_eq!(user.desc2(), Some(&"update_1".to_string()));
        let user = User::fetch_first().await?;
        assert_eq!(user.uid(), Some(&1));
        let user = User::fetch_last().await?;
        eprintln!("------{:?}", user.uid());
        assert_eq!(user.uid(), Some(&10));

        // delete batch
        User::delete_all(json!(["id > ?", 5])).execute().await?;
        let users = User::query().fetch_all().await?;
        assert_eq!(users.len(), 5);

        // query one
        let user = User::query().fetch_one().await?;
        assert_eq!(user.uid(), Some(&1));

        // update
        let mut user = User::query().fetch_one().await?;
        user.set_desc2("custom desc".to_string()).save().await?;
        assert_eq!(user.desc2(), Some(&"custom desc".to_string()));
        let user = User::query().fetch_one().await?;
        assert_eq!(user.desc2(), Some(&"custom desc".to_string()));

        let expired_at = chrono::Utc.ymd(2021, 12, 31).and_hms(23, 59, 59);

        // create
        let mut user = User::new();
        user.set_desc2("create desc".to_string())
            .set_expired_at(expired_at.clone())
            .save().await?;
        let users = User::query().fetch_all().await?;
        assert_eq!(users.len(), 6);

        // // delete
        let mut user = User::query().fetch_one().await?;
        let _result = user.delete().await?;
        let users = User::query().fetch_all().await?;
        assert_eq!(users.len(), 5);

        // query_count
        let count = User::fetch_count().await?;
        assert_eq!(count, 5);
        let count = User::query().where_range("expired_at", ..=expired_at).fetch_count().await?;
        assert_eq!(count, 5);
        let count = User::query().where_range("expired_at", ..expired_at).fetch_count().await?;
        assert_eq!(count, 4);
        let count = User::query().where_range("expired_at", expired_at..).fetch_count().await?;
        assert_eq!(count, 1);

        Ok(())
    }

    #[test]
    fn test_sqlite_sqlx() {
        assert!(
            match tokio_test::block_on(main_test()) {
                Ok(()) => Ok(()),
                Err(e) => {
                    eprintln!("err: {:?}", e);
                    Err(e)
                }
        }.is_ok());
    }
}