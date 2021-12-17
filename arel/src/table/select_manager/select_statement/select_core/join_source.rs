use serde_json::{Value as Json};
use std::marker::PhantomData;
use crate::traits::ArelAble;
use crate::statements::{Join, StatementAble};
use crate::collectors::Sql;

#[derive(Clone, Debug)]
pub struct JoinSource<M: ArelAble> {
    join: Join<M>,
    // left: Option<StatementsType<M>>,
    // right: Option<StatementsType<M>>,
    _marker: PhantomData<M>,
}

impl<M> JoinSource<M> where M: ArelAble {
    pub fn new(condition: Json) -> Self {
        Self {
            join: Join::<M>::new(condition),
            // left: None,
            // right: None,
            _marker: PhantomData,
        }
    }
    pub fn to_sql(&self) -> anyhow::Result<Sql> {
        self.join.to_sql()
    }
    pub fn to_sql_string(&self) -> anyhow::Result<String> {
        self.to_sql()?.to_sql_string()
    }
}