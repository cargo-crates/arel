use serde_json::{Value as Json};
use std::marker::PhantomData;
use crate::traits::ArelAble;
use crate::statements::StatementAble;
use crate::collectors::Sql;

#[derive(Clone, Debug)]
pub struct Lock<M: ArelAble> {
    value: Json,
    _marker: PhantomData<M>,
}

impl<M> StatementAble<M> for Lock<M> where M: ArelAble {
    fn json_value(&self) -> Option<&Json> {
        Some(&self.value)
    }
    fn value_sql_string_from_json(json_value: &Json) -> anyhow::Result<String> {
        match json_value {
            Json::String(json_string) => {
                Ok(format!("{}", json_string))
            },
            _ => Self::default_value_sql_string_from_json(json_value)
        }
    }
    fn to_sql(&self) -> anyhow::Result<Sql> {
        self.to_sql_with_concat(" ")
    }
}

impl<M> Lock<M> where M: ArelAble {
    pub fn new(value: Json) -> Self {
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

        let lock = Lock::<User>::new(json!("FOR UPDATE"));
        assert_eq!(lock.to_sql_string().unwrap(), "FOR UPDATE");
    }
}
