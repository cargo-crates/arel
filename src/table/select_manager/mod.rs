pub mod select_statement;
pub use select_statement::SelectStatement;

use serde_json::Value as Json;
use std::default::Default;
use std::marker::PhantomData;
use crate::traits::ModelAble;
use select_statement::SelectCore;

#[derive(Debug, Clone)]
pub struct SelectManager<M: ModelAble> {
    pub ast: SelectStatement<M>,
    _marker: PhantomData<M>,
}

impl<M> Default for SelectManager<M> where M: ModelAble {
    fn default() -> Self {
        Self {
            ast: SelectStatement::default(),
            _marker: PhantomData
        }
    }
}

impl<M> SelectManager<M> where M: ModelAble {
    pub fn ctx_mut(&mut self) -> &mut SelectCore<M> {
        self.ast.cores.last_mut().unwrap()
    }
    // fn ctx(&self) -> &SelectCore<M> {
    //     self.ast.cores.last().unwrap()
    // }
    pub fn select(&mut self, condition: Json) -> &mut Self {
        self.ctx_mut().select(condition);
        self
    }
    pub fn distinct(&mut self) -> &mut Self {
        self.ctx_mut().distinct();
        self
    }
    pub fn lock(&mut self, condition: Json) -> &mut Self {
        self.ast.lock(condition);
        self
    }
    pub fn joins(&mut self, condition: Json) -> &mut Self {
        self.ctx_mut().joins(condition);
        self
    }
    pub fn r#where(&mut self, condition: Json, is_not: bool) -> &mut Self {
        self.ctx_mut().r#where(condition, is_not);
        self
    }
    // pub fn get_where_sql(&self) -> Option<SqlLiteral> {
    //     self.ctx().get_where_sql()
    // }
    pub fn group(&mut self, condition: Json) -> &mut Self {
        self.ctx_mut().group(condition);
        self
    }
    pub fn having(&mut self, condition: Json, is_not: bool) -> &mut Self {
        self.ctx_mut().having(condition, is_not);
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
