use serde_json::{Value as Json};
use std::marker::PhantomData;
use crate::traits::ArelAble;
use crate::statements::StatementAble;
use crate::collectors::Sql;

#[derive(Clone, Debug)]
pub struct Limit<M: ArelAble> {
    value: usize,
    _marker: PhantomData<M>,
}

impl<M> StatementAble<M> for Limit<M> where M: ArelAble {
    fn json_value(&self) -> Option<&Json> { None }
    fn to_sql(&self) -> anyhow::Result<Sql> {
        let mut sql = Sql::default();
        sql.push_str(&format!("LIMIT {}", self.value));
        Ok(sql)
    }
}

impl<M> Limit<M> where M: ArelAble {
    pub fn new(value: usize) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }
}


#[cfg(test)]
mod tests {
    use crate as arel;
    use arel::prelude::*;
    use super::*;
    #[test]
    fn to_sql() {
        #[arel::arel]
        #[allow(dead_code)]
        struct User {
            id: i64,
        }

        let limit = Limit::<User>::new(10);
        assert_eq!(limit.to_sql_string().unwrap(), "LIMIT 10");
    }
}