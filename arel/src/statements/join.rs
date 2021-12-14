use serde_json::{Value as Json};
use std::marker::PhantomData;
use crate::traits::ArelAble;
use crate::statements::StatementAble;

#[derive(Clone, Debug)]
pub struct Join<M: ArelAble> {
    value: Json,
    _marker: PhantomData<M>,
}

impl<M> StatementAble<M> for Join<M> where M: ArelAble {
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

impl<M> Join<M> where M: ArelAble {
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

        let join = Join::<User>::new(json!("LEFT JOINS orders ON users.id = orders.user_id"));
        assert_eq!(join.to_sql().unwrap(), "LEFT JOINS orders ON users.id = orders.user_id");

        let join = Join::<User>::new(json!(["LEFT JOINS orders ON users.id = orders.user_id"]));
        assert_eq!(join.to_sql().unwrap(), "LEFT JOINS orders ON users.id = orders.user_id");
        let join = Join::<User>::new(json!(["LEFT JOINS ? ON users.id = ? AND users.age = ?", "orders", "orders.user_id", 18]));
        assert_eq!(join.to_sql().unwrap(), "LEFT JOINS orders ON users.id = orders.user_id AND users.age = 18");
    }
}
