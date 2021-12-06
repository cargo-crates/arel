pub mod select_core;
pub use select_core::SelectCore;

use std::default::Default;
use std::marker::PhantomData;
use serde_json::Value as Json;
use crate::traits::ArelAble;
use crate::statements::{StatementAble, Order, Limit, Offset, Lock, helpers};
use crate::nodes::{SqlLiteral};

#[derive(Clone, Debug)]
pub struct SelectStatement<M: ArelAble> {
    pub cores: Vec<SelectCore<M>>,
    orders: Vec<Order<M>>,
    limit: Option<Limit<M>>,
    offset: Option<Offset<M>>,
    lock: Option<Lock<M>>,
    // with: Option<StatementsType<M>>,
    _marker: PhantomData<M>,
}

// impl<M> ManagerStatement<M> for SelectStatement<M> where M: ArelAble {}

impl<M> Default for SelectStatement<M> where M: ArelAble {
    fn default() -> Self {
        Self {
            cores: vec![SelectCore::<M>::default()],
            orders: vec![],
            limit: None,
            offset: None,
            lock: None,
            // with: None,
            _marker: PhantomData,
        }
    }
}

impl<M> SelectStatement<M> where M: ArelAble {
    pub fn lock(&mut self, condition: Json) -> &mut Self {
        self.lock = Some(Lock::<M>::new(condition));
        self
    }
    pub fn get_lock_sql(&self) -> anyhow::Result<Option<SqlLiteral>> {
        if let Some(lock) = &self.lock {
            Ok(Some(SqlLiteral::new(lock.to_sql()?)))
        } else {
            Ok(None)
        }
    }
    pub fn order(&mut self, condition: Json) -> &mut Self {
        self.orders.push(Order::new(condition));
        self
    }
    pub fn get_order_sql(&self) -> anyhow::Result<Option<SqlLiteral>> {
        if self.orders.len() == 0 {
            Ok(None)
        } else {
            Ok(Some(SqlLiteral::new(helpers::inject_join(&self.orders, ", ")?)))
        }
    }
    pub fn limit(&mut self, condition: usize) -> &mut Self {
        self.limit = Some(Limit::new(condition));
        self
    }
    pub fn get_limit_sql(&self) -> anyhow::Result<Option<SqlLiteral>> {
        if let Some(limit) = &self.limit {
            Ok(Some(SqlLiteral::new(limit.to_sql()?)))
        } else {
            Ok(None)
        }
    }
    pub fn offset(&mut self, condition: usize) -> &mut Self {
        self.offset = Some(Offset::new(condition));
        self
    }
    pub fn get_offset_sql(&self) -> anyhow::Result<Option<SqlLiteral>> {
        if let Some(offset) = &self.offset {
            Ok(Some(SqlLiteral::new(offset.to_sql()?)))
        } else {
            Ok(None)
        }
    }
}
