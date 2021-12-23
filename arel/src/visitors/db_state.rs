use std::future::Future;
use std::pin::Pin;
use once_cell::sync::OnceCell;

// use sqlx::sqlite::{SqlitePool};
use sqlx::any::{AnyPool};

#[derive(Debug)]
pub struct DbState {
    #[allow(dead_code)]
    name: &'static str,
    pool: AnyPool
}

impl DbState {
    pub async fn transaction_auto_commit<T: crate::ArelAble, F: Send>(callback: F, mut tx: sqlx::Transaction<'static, sqlx::Any>) -> anyhow::Result<Option<T>>
        where for<'c> F: FnOnce(&'c mut sqlx::Transaction<sqlx::Any>) -> Pin<Box<dyn Future<Output = anyhow::Result<Option<T>>> + Send + 'c >>
    {
        match callback(&mut tx).await {
            Ok(model) => {
                match tx.commit().await {
                    Ok(_) => Ok(model),
                    Err(e) => Err(anyhow::anyhow!(e.to_string()))
                }
            },
            Err(e) => {
                tx.rollback().await?;
                Err(e)
            }
        }
    }
    pub fn pool(&self) -> &AnyPool {
        &self.pool
    }
    // eg:
    // db_state.exec(async move |db_state: &DbState| {
    //     Ok(())
    // }).await?;
    pub async fn with_transaction<T: crate::ArelAble, F: Send>(&self, callback: F) -> anyhow::Result<Option<T>>
        where for<'c> F: FnOnce(&'c mut sqlx::Transaction<sqlx::Any>) -> Pin<Box<dyn Future<Output = anyhow::Result<Option<T>>> + Send + 'c >>
    {
        Self::transaction_auto_commit(callback, self.pool().begin().await?).await
    }
}

pub static DB_STATE_CELL: OnceCell<DbState>  = OnceCell::new();
// pub async fn get_or_init_db_state<Fut>(f: impl FnOnce() -> Fut) -> anyhow::Result<&'static DbState>
//     where Fut: Future<Output = Result<AnyPool, sqlx::Error>>;
pub async fn get_or_init_db_state<F>(callback: F) -> anyhow::Result<&'static DbState>
    where F: FnOnce() -> Pin<Box<dyn Future<Output = Result<AnyPool, sqlx::Error>>>>
{
    if DB_STATE_CELL.get().is_none() {
        let pool = callback().await?;
        if let Err(_) = DB_STATE_CELL.set(DbState {
            name: "default",
            pool,
        }) {
            return Err(anyhow::anyhow!("Set DB_STATE_CELL Failed"));
        }
    }
    if let Some(db_state) = DB_STATE_CELL.get() {
        Ok(db_state)
    } else {
        Err(anyhow::anyhow!("Get DB_STATE_CELL Failed"))
    }
}
pub fn get_db_state() -> anyhow::Result<&'static DbState> {
    if let Some(db_state) = DB_STATE_CELL.get() {
        Ok(db_state)
    } else {
        Err(anyhow::anyhow!("No DB State"))
    }
}
