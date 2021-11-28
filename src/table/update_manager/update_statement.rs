use crate::statements::{StatementAble, Where, Update, helpers::and};
use serde_json::{Value as Json};
use std::marker::PhantomData;
use std::default::Default;
use crate::traits::ModelAble;
use crate::nodes::{SqlLiteral};

#[derive(Debug, Clone)]
pub struct UpdateStatement<M: ModelAble> {
    // @relation = nil
    wheres: Vec<Where<M>>,
    values: Option<Update<M>>,
    // @orders   = []
    // @limit    = nil
    // @offset   = nil
    _marker: PhantomData<M>,
}

impl<M> Default for UpdateStatement<M> where M: ModelAble {
    fn default() -> Self {
        Self {
            wheres: vec![],
            values: None,
            _marker: PhantomData,
        }
    }
}

impl<M> UpdateStatement<M> where M: ModelAble {
    pub fn update(&mut self, condition: Json) -> &mut Self {
        self.values = Some(Update::new(condition));
        self
    }
    pub fn get_update_sql(&self) -> Option<SqlLiteral> {
        if let Some(update) = &self.values {
            Some(SqlLiteral::new(update.to_sql()))
        } else {
            None
        }
    }
}

