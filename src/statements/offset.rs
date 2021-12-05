use serde_json::{Value as Json};
use std::marker::PhantomData;
use crate::traits::ModelAble;
use crate::statements::StatementAble;

#[derive(Clone, Debug)]
pub struct Offset<M: ModelAble> {
    value: usize,
    _marker: PhantomData<M>,
}

impl<M> StatementAble<M> for Offset<M> where M: ModelAble {
    fn json_value(&self) -> Option<&Json> { None }
    fn to_sql(&self) -> anyhow::Result<String> {
        Ok(format!("OFFSET {}", self.value))
    }
}

impl<M> Offset<M> where M: ModelAble {
    pub fn new(value: usize) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use serde_json::{json};
    #[test]
    fn to_sql() {
        #[derive(Clone, Debug)]
        struct User {}
        impl ModelAble for User {}

        let offset = Offset::<User>::new(10);
        assert_eq!(offset.to_sql().unwrap(), "OFFSET 10");
    }
}
