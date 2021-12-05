use crate::statements::{StatementAble, Insert};
use serde_json::{Value as Json};
use std::marker::PhantomData;
use std::default::Default;
use crate::traits::ModelAble;
use crate::nodes::{SqlLiteral};

#[derive(Debug, Clone)]
pub struct InsertStatement<M: ModelAble> {
    insert: Option<Insert<M>>,
    _marker: PhantomData<M>,
}

impl<M> Default for InsertStatement<M> where M: ModelAble {
    fn default() -> Self {
        Self {
            insert: None,
            _marker: PhantomData,
        }
    }
}

impl<M> InsertStatement<M> where M: ModelAble {
    pub fn insert(&mut self, condition: Json) -> &mut Self {
        self.insert = Some(Insert::new(condition));
        self
    }
    pub fn get_insert_sql(&self) -> anyhow::Result<Option<SqlLiteral>> {
        if let Some(insert) = &self.insert {
            let mut sql = "INSERT INTO ".to_string();
            sql.push_str(&insert.to_sql()?);
            Ok(Some(SqlLiteral::new(sql)))
        } else {
            Ok(None)
        }
    }
}

