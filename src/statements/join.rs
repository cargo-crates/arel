use serde_json::{Value as Json};
use std::marker::PhantomData;
use crate::traits::ModelAble;
use crate::statements::StatementAble;

#[derive(Clone, Debug)]
pub struct Join<M: ModelAble> {
    value: Json,
    _marker: PhantomData<M>,
}

impl<M> StatementAble<M> for Join<M> where M: ModelAble {
    fn json_value(&self) -> Option<&Json> {
        Some(&self.value)
    }
    fn to_sql(&self) -> String {
        self.to_sql_with_concat(" ")
    }
    fn json_value_sql(&self, json_value: &Json) -> String {
        match json_value {
            Json::String(json_string) => {
                format!("{}", json_string)
            },
            _ => StatementAble::json_value_sql_default(self, json_value)
        }
    }
}

impl<M> Join<M> where M: ModelAble {
    pub fn new(value: Json) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json};
    #[test]
    fn to_sql() {
        #[derive(Clone, Debug)]
        struct User {}
        impl ModelAble for User {}

        let join = Join::<User>::new(json!("LEFT JOINS orders ON users.id = orders.user_id"));
        assert_eq!(join.to_sql(), "LEFT JOINS orders ON users.id = orders.user_id");

        let join = Join::<User>::new(json!(["LEFT JOINS orders ON users.id = orders.user_id"]));
        assert_eq!(join.to_sql(), "LEFT JOINS orders ON users.id = orders.user_id");
        let join = Join::<User>::new(json!(["LEFT JOINS ? ON users.id = ? AND users.age = ?", "orders", "orders.user_id", 18]));
        assert_eq!(join.to_sql(), "LEFT JOINS orders ON users.id = orders.user_id AND users.age = 18");
    }
}
