use serde_json::{Value as Json};
use std::marker::PhantomData;
use crate::traits::ArelAble;
use crate::statements::{Join, StatementAble};

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
    pub fn to_sql(&self) -> anyhow::Result<String> {
        self.join.to_sql()
    }
}