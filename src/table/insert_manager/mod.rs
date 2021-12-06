pub mod insert_statement;
pub use insert_statement::InsertStatement;

use std::default::Default;
use std::marker::PhantomData;
use serde_json::{Value as Json};
use crate::traits::ArelAble;

#[derive(Debug, Clone)]
pub struct InsertManager<M: ArelAble> {
    pub ast: InsertStatement<M>,
    _marker: PhantomData<M>,
}

impl<M> Default for InsertManager<M> where M: ArelAble {
    fn default() -> Self {
        Self {
            ast: InsertStatement::default(),
            _marker: PhantomData
        }
    }
}

impl<M> InsertManager<M> where M: ArelAble {
    pub fn ctx_mut(&mut self) -> &mut InsertStatement<M> {
        &mut self.ast
    }
    pub fn insert(&mut self, condition: Json) -> &mut Self {
        self.ctx_mut().insert(condition);
        self
    }
}