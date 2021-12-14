use std::future::Future;
use once_cell::sync::OnceCell;

// use sqlx::sqlite::{SqlitePool};
use sqlx::any::{AnyPool};
use crate::visitors::db_transaction::{DbTransaction};

#[derive(Debug)]
pub struct DbState {
    #[allow(dead_code)]
    name: &'static str,
    pool: AnyPool
}

impl DbState {
    pub fn pool(&self) -> &AnyPool {
        &self.pool
    }
    // eg:
    // db_state.exec(async move |db_state: &DbState| {
    //     Ok(())
    // }).await?;
    pub async fn transaction(&self) -> anyhow::Result<()> {

        let mut transaction = DbTransaction::new(self.pool().begin().await?);
        transaction.execute().await



        // f(tx).await
        // match {
        //     f(tx).await
        // } {
        //     Ok(()) => {
        //         match tx.commit().await {
        //             Ok(()) => Ok(()),
        //             Err(err) => Err(err.into())
        //         }
        //     },
        //     Err(err) => Err(err),
        // }
        // match f(&mut tx).await {
        //     Ok(()) => {
        //         match tx.commit().await {
        //             Ok(()) => Ok(()),
        //             Err(err) => Err(err.into())
        //         }
        //     },
        //     Err(err) => Err(err),
        // }
    }
}

pub static DB_STATE_CELL: OnceCell<DbState>  = OnceCell::new();
pub async fn get_or_init_db_state<Fut>(f: impl FnOnce() -> Fut) -> anyhow::Result<&'static DbState>
    where Fut: Future<Output = Result<AnyPool, sqlx::Error>>
{
    if DB_STATE_CELL.get().is_none() {
        let pool = f().await?;
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
