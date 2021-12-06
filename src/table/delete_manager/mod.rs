pub mod delete_statement;
pub use delete_statement::DeleteStatement;

use serde_json::Value as Json;
use std::default::Default;
use std::marker::PhantomData;
use crate::traits::ArelAble;
use crate::statements::{r#where};

#[derive(Debug, Clone)]
pub struct DeleteManager<M: ArelAble> {
    pub ast: DeleteStatement<M>,
    _marker: PhantomData<M>,
}

impl<M> Default for DeleteManager<M> where M: ArelAble {
    fn default() -> Self {
        Self {
            ast: DeleteStatement::default(),
            _marker: PhantomData
        }
    }
}

impl<M> DeleteManager<M> where M: ArelAble {
    pub fn ctx_mut(&mut self) -> &mut DeleteStatement<M> {
        &mut self.ast
    }
    // fn ctx(&self) -> &SelectCore<M> {
    //     self.ast.cores.last().unwrap()
    // }
    pub fn r#where(&mut self, condition: Json, ops: r#where::Ops) -> &mut Self {
        self.ctx_mut().r#where(condition, ops);
        self
    }
    // pub fn get_where_sql(&self) -> Option<SqlLiteral> {
    //     self.ctx().get_where_sql()
    // }
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
