pub mod select_core;
pub use select_core::SelectCore;

use std::default::Default;
use std::marker::PhantomData;
use serde_json::Value as Json;
use crate::traits::ModelAble;
use crate::statements::{StatementAble, Lock};
use crate::nodes::{SqlLiteral};

#[derive(Clone, Debug)]
pub struct SelectStatement<M: ModelAble> {
    pub cores: Vec<SelectCore<M>>,
    // orders: Vec<StatementsType<M>>,
    // limit: Option<StatementsType<M>>,
    lock: Option<Lock<M>>,
    // offset: Option<StatementsType<M>>,
    // with: Option<StatementsType<M>>,
    _marker: PhantomData<M>,
}

// impl<M> ManagerStatement<M> for SelectStatement<M> where M: ModelAble {}

impl<M> Default for SelectStatement<M> where M: ModelAble {
    fn default() -> Self {
        Self {
            cores: vec![SelectCore::<M>::default()],
            // orders: vec![],
            // limit: None,
            lock: None,
            // offset: None,
            // with: None,
            _marker: PhantomData,
        }
    }
}

impl<M> SelectStatement<M> where M: ModelAble {
    pub fn lock(&mut self, condition: Json) -> &mut Self {
        self.lock = Some(Lock::<M>::new(condition));
        self
    }
    pub fn get_lock_sql(&self) -> Option<SqlLiteral> {
        if let Some(lock) = &self.lock {
            Some(SqlLiteral::new(lock.to_sql()))
        } else {
            None
        }
    }
}
