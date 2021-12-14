// use std::future::Future;
// pub type ClosureType = &'static dyn FnOnce(&mut sqlx::Transaction<sqlx::Any>) -> dyn Future<Output = anyhow::Result<()>>;

pub struct DbTransaction<'a> {
    tx: sqlx::Transaction<'a, sqlx::Any>,
    // closure: ClosureType,
}

impl<'a> DbTransaction<'a> {
    pub fn new(tx: sqlx::Transaction<'a, sqlx::Any>) -> Self
    {
        Self {
            tx,
            // closure
        }
    }
    // fn tx(&mut self) -> &mut sqlx::Transaction<'a, sqlx::Any> {
    //     &mut self.tx
    // }
    pub async fn execute(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

use core::fmt::Debug;
impl<'a> Debug for DbTransaction<'a>  {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Transaction: {:?}", &self.tx)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use serde_json::{json};
//     #[test]
//     fn to_sql() {
//         #[derive(Clone, Debug)]
//         struct User {}
//         impl ModelAble for User {}
//     }
// }
