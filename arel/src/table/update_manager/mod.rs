pub mod update_statement;
pub use update_statement::UpdateStatement;

use std::default::Default;
use std::marker::PhantomData;
use serde_json::{Value as Json};
use crate::traits::ArelAble;
use crate::statements::{r#where::{self}};

#[derive(Debug, Clone)]
pub struct UpdateManager<M: ArelAble> {
    pub ast: UpdateStatement<M>,
    _marker: PhantomData<M>,
}

impl<M> Default for UpdateManager<M> where M: ArelAble {
    fn default() -> Self {
        Self {
            ast: UpdateStatement::default(),
            _marker: PhantomData
        }
    }
}

impl<M> UpdateManager<M> where M: ArelAble {
    pub fn ctx_mut(&mut self) -> &mut UpdateStatement<M> {
        &mut self.ast
    }
    pub fn update(&mut self, condition: Json) -> &mut Self {
        self.ctx_mut().update(condition);
        self
    }
    pub fn increment(&mut self, column_name: &str, by: isize) -> &mut Self {
        self.ctx_mut().increment(column_name, by);
        self
    }
    pub fn decrement(&mut self, column_name: &str, by: isize) -> &mut Self {
        self.ctx_mut().decrement(column_name, by);
        self
    }
    pub fn r#where(&mut self, condition: Json, ops: r#where::Ops) -> &mut Self {
        self.ctx_mut().r#where(condition, ops);
        self
    }
    pub fn order(&mut self, condition: Json) -> &mut Self {
        self.ast.order(condition);
        self
    }
    pub fn limit(&mut self, condition: usize) -> &mut Self {
        self.ast.limit(condition);
        self
    }
    pub fn offset(&mut self, condition: usize) -> &mut Self {
        self.ast.offset(condition);
        self
    }
}