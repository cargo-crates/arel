use crate::statements::{StatementAble, Insert};
use serde_json::{Value as Json};
use std::marker::PhantomData;
use std::default::Default;
use crate::traits::ArelAble;
use crate::collectors::Sql;

#[derive(Debug, Clone)]
pub struct InsertStatement<M: ArelAble> {
    insert: Option<Insert<M>>,
    _marker: PhantomData<M>,
}

impl<M> Default for InsertStatement<M> where M: ArelAble {
    fn default() -> Self {
        Self {
            insert: None,
            _marker: PhantomData,
        }
    }
}

impl<M> InsertStatement<M> where M: ArelAble {
    pub fn insert(&mut self, condition: Json) -> &mut Self {
        self.insert = Some(Insert::new(condition));
        self
    }
    pub fn get_insert_sql(&self) -> anyhow::Result<Option<Sql>> {
        if let Some(insert) = &self.insert {
            let mut collector = insert.to_sql()?;
            collector.value = format!("INSERT INTO {}", &collector.value);
            Ok(Some(collector))
        } else {
            Ok(None)
        }
    }
}

