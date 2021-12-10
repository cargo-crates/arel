use serde_json::{Value as Json};
use std::marker::PhantomData;
use crate::traits::ArelAble;
use crate::statements::StatementAble;

#[derive(Clone, Debug)]
pub struct Lock<M: ArelAble> {
    value: Json,
    _marker: PhantomData<M>,
}

impl<M> StatementAble<M> for Lock<M> where M: ArelAble {
    fn json_value(&self) -> Option<&Json> {
        Some(&self.value)
    }
    fn to_sql(&self) -> anyhow::Result<String> {
        self.to_sql_with_concat(" ")
    }
    fn json_value_sql(&self, json_value: &Json) -> anyhow::Result<String> {
        match json_value {
            Json::String(json_string) => {
                Ok(format!("{}", json_string))
            },
            _ => StatementAble::json_value_sql_default(self, json_value)
        }
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
    use super::*;
    use serde_json::{json};
    #[test]
    fn to_sql() {
        #[arel::arel]
        #[allow(dead_code)]
        struct User {}

        let lock = Lock::<User>::new(json!("FOR UPDATE"));
        assert_eq!(lock.to_sql().unwrap(), "FOR UPDATE");

        let lock = Lock::<User>::new(json!(["FOR UPDATE"]));
        assert_eq!(lock.to_sql().unwrap(), "FOR UPDATE");
        let lock = Lock::<User>::new(json!(["FOR ?", "UPDATE"]));
        assert_eq!(lock.to_sql().unwrap(), "FOR UPDATE");
    }
}
