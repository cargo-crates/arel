use serde_json::{Value as Json};
use crate::table::select_statement::{SelectStatement, SelectCore};
use std::default::Default;
use crate::traits::ModelAble;
use std::marker::PhantomData;

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
    fn ctx_mut(&mut self) -> &mut SelectCore<M> {
        self.ast.cores.last_mut().unwrap()
    }
    // fn ctx(&self) -> &SelectCore<M> {
    //     self.ast.cores.last().unwrap()
    // }
    pub fn joins(&mut self, condition: Json) -> &mut Self {
        self.ctx_mut().joins(condition);
        self
    }
    pub fn r#where(&mut self, condition: Json) -> &mut Self {
        self.ctx_mut().r#where(condition);
        self
    }
    // pub fn get_where_sql(&self) -> Option<SqlLiteral> {
    //     self.ctx().get_where_sql()
    // }
}