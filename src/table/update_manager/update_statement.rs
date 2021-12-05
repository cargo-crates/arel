use crate::statements::{StatementAble, r#where::{self, Where}, Order, Limit, Offset, Update, helpers::{self, and}};
use serde_json::{Value as Json, json};
use std::marker::PhantomData;
use std::default::Default;
use crate::traits::ModelAble;
use crate::nodes::{SqlLiteral};
use crate::methods;

#[derive(Debug, Clone)]
pub struct UpdateStatement<M: ModelAble> {
    // @relation = nil
    update: Option<Update<M>>,
    pub wheres: Vec<Where<M>>,
    orders: Vec<Order<M>>,
    limit: Option<Limit<M>>,
    offset: Option<Offset<M>>,
    _marker: PhantomData<M>,
}

impl<M> Default for UpdateStatement<M> where M: ModelAble {
    fn default() -> Self {
        Self {
            update: None,
            wheres: vec![],
            orders: vec![],
            limit: None,
            offset: None,
            _marker: PhantomData,
        }
    }
}

impl<M> UpdateStatement<M> where M: ModelAble {
    pub fn update(&mut self, condition: Json) -> &mut Self {
        self.update = Some(Update::new(condition));
        self
    }
    pub fn increment(&mut self, column_name: &str, by: isize) -> &mut Self {
        let table_column_name = methods::table_column_name::<M>(column_name);
        let mut raw_sql = format!("{} = COALESCE({}, 0)", table_column_name, table_column_name);
        if by >= 0 {
            raw_sql.push_str(&format!(" + {}", by));
        } else {
            raw_sql.push_str(&format!(" - {}", by.abs()));
        }
        self.update = Some(Update::<M>::new(json!(raw_sql)));
        self
    }
    pub fn decrement(&mut self, column_name: &str, by: isize) -> &mut Self {
        self.increment(column_name, -by)
    }
    pub fn get_update_sql(&self) -> anyhow::Result<Option<SqlLiteral>> {
        if let Some(update) = &self.update {
            let mut sql = "UPDATE ".to_string();
            sql.push_str(&methods::quote_table_name(&M::table_name()));
            sql.push_str(" SET ");
            sql.push_str(&update.to_sql()?);
            Ok(Some(SqlLiteral::new(sql)))
        } else {
            Ok(None)
        }
    }
    pub fn r#where(&mut self, condition: Json, ops: r#where::Ops) -> &mut Self {
        self.wheres.push(Where::<M>::new(condition, ops));
        self
    }
    pub fn get_where_sql(&self) -> anyhow::Result<Option<SqlLiteral>> {
        if self.r#wheres.len() == 0 {
            Ok(None)
        } else {
            Ok(Some(SqlLiteral::new(format!("{}", and::to_sql(&self.r#wheres)?))))
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

