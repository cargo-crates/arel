pub mod update_statement;
pub use update_statement::UpdateStatement;

use std::default::Default;
use std::marker::PhantomData;
use serde_json::{Value as Json};
use crate::traits::ModelAble;

#[derive(Debug, Clone)]
pub struct UpdateManager<M: ModelAble> {
    pub ast: UpdateStatement<M>,
    _marker: PhantomData<M>,
}

impl<M> Default for UpdateManager<M> where M: ModelAble {
    fn default() -> Self {
        Self {
            ast: UpdateStatement::default(),
            _marker: PhantomData
        }
    }
}

impl<M> UpdateManager<M> where M: ModelAble {
    pub fn ctx_mut(&mut self) -> &mut UpdateStatement<M> {
        &mut self.ast
    }
    pub fn update(&mut self, condition: Json) -> &mut Self {
        self.ctx_mut().update(condition);
        self
    }
    pub fn r#where(&mut self, condition: Json, is_not: bool) -> &mut Self {
        self.ctx_mut().r#where(condition, is_not);
        self
    }
}