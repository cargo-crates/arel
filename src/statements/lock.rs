use serde_json::{Value as Json};
use std::marker::PhantomData;
use crate::traits::ModelAble;
use crate::statements::StatementAble;

#[derive(Clone, Debug)]
pub struct Lock<M: ModelAble> {
    value: Json,
    _marker: PhantomData<M>,
}

impl<M> StatementAble<M> for Lock<M> where M: ModelAble {
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

impl<M> Lock<M> where M: ModelAble {
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

        let lock = Lock::<User>::new(json!("FOR UPDATE"));
        assert_eq!(lock.to_sql(), "FOR UPDATE");

        let lock = Lock::<User>::new(json!(["FOR UPDATE"]));
        assert_eq!(lock.to_sql(), "FOR UPDATE");
        let lock = Lock::<User>::new(json!(["FOR ?", "UPDATE"]));
        assert_eq!(lock.to_sql(), "FOR UPDATE");
    }
}
