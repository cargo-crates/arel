use std::default::Default;
use std::marker::PhantomData;
use serde_json::{Value as Json};
use crate::traits::ModelAble;
use crate::statements::{StatementAble, r#where::{self, Where}, Order, Limit, Offset, helpers::{self, and}};
use crate::nodes::{SqlLiteral};

#[derive(Clone, Debug)]
pub struct DeleteStatement<M: ModelAble> {
    pub wheres: Vec<Where<M>>,
    orders: Vec<Order<M>>,
    limit: Option<Limit<M>>,
    offset: Option<Offset<M>>,
    _marker: PhantomData<M>,
}

impl<M> Default for DeleteStatement<M> where M: ModelAble {
    fn default() -> Self {
        Self {
            wheres: vec![],
            orders: vec![],
            limit: None,
            offset: None,
            _marker: PhantomData,
        }
    }
}

impl<M> DeleteStatement<M> where M: ModelAble {
    pub fn r#where(&mut self, condition: Json, ops: r#where::Ops) -> &mut Self {
        self.wheres.push(Where::<M>::new(condition, ops));
        self
    }
    pub fn get_where_sql(&self) -> Option<SqlLiteral> {
        if self.r#wheres.len() == 0 {
            None
        } else {
            Some(SqlLiteral::new(and::to_sql(&self.r#wheres)))
        }
    }
    pub fn order(&mut self, condition: Json) -> &mut Self {
        self.orders.push(Order::new(condition));
        self
    }
    pub fn get_order_sql(&self) -> Option<SqlLiteral> {
        if self.orders.len() == 0 {
            None
        } else {
            Some(SqlLiteral::new(helpers::inject_join(&self.orders, ", ")))
        }
    }
    pub fn limit(&mut self, condition: usize) -> &mut Self {
        self.limit = Some(Limit::new(condition));
        self
    }
    pub fn get_limit_sql(&self) -> Option<SqlLiteral> {
        if let Some(limit) = &self.limit {
            Some(SqlLiteral::new(limit.to_sql()))
        } else {
            None
        }
    }
    pub fn offset(&mut self, condition: usize) -> &mut Self {
        self.offset = Some(Offset::new(condition));
        self
    }
    pub fn get_offset_sql(&self) -> Option<SqlLiteral> {
        if let Some(offset) = &self.offset {
            Some(SqlLiteral::new(offset.to_sql()))
        } else {
            None
        }
    }
}