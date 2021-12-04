use crate::statements::{StatementAble, r#where::{self, Where}, Update, helpers::and};
use serde_json::{Value as Json};
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
    // @orders   = []
    // @limit    = nil
    // @offset   = nil
    _marker: PhantomData<M>,
}

impl<M> Default for UpdateStatement<M> where M: ModelAble {
    fn default() -> Self {
        Self {
            update: None,
            wheres: vec![],
            _marker: PhantomData,
        }
    }
}

impl<M> UpdateStatement<M> where M: ModelAble {
    pub fn update(&mut self, condition: Json) -> &mut Self {
        self.update = Some(Update::new(condition));
        self
    }
    pub fn get_update_sql(&self) -> Option<SqlLiteral> {
        if let Some(update) = &self.update {
            let mut sql = "UPDATE ".to_string();
            sql.push_str(&methods::quote_table_name(&M::table_name()));
            sql.push_str(" SET ");
            sql.push_str(&update.to_sql());
            Some(SqlLiteral::new(sql))
        } else {
            None
        }
    }
    pub fn r#where(&mut self, condition: Json, ops: r#where::Ops) -> &mut Self {
        self.wheres.push(Where::<M>::new(condition, ops));
        self
    }
    pub fn get_where_sql(&self) -> Option<SqlLiteral> {
        if self.r#wheres.len() == 0 {
            None
        } else {
            Some(SqlLiteral::new(format!("{}", and::to_sql(&self.r#wheres))))
        }
    }
}

