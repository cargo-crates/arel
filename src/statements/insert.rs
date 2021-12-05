use serde_json::{Value as Json};
use std::marker::PhantomData;
use crate::traits::ModelAble;
use crate::statements::StatementAble;
use crate::nodes::SqlLiteral;
use crate::methods;

#[derive(Clone, Debug)]
pub struct Insert<M: ModelAble> {
    value: Json,
    _marker: PhantomData<M>,
}

impl<M> StatementAble<M> for Insert<M> where M: ModelAble {
    fn json_value(&self) -> Option<&Json> {
        Some(&self.value)
    }
    fn to_sql_literals(&self) -> anyhow::Result<Vec<SqlLiteral>> {
        let mut vec = vec![];
        if let Some(json_value) = self.json_value() {
            match json_value {
                Json::Object(json_object) => {
                    let mut keys = vec![];
                    let mut values = vec![];
                    for column_name in json_object.keys() {
                        keys.push(format!("`{}`", column_name));
                        let json_value = json_object.get(column_name).unwrap();
                        values.push(self.json_value_sql(json_value)?);
                    }
                    vec.push(SqlLiteral::new(format!("{} ({}) VALUES ({})", methods::quote_table_name(&M::table_name()), keys.join(", "), values.join(", "))));
                },
                _ => return Err(anyhow::anyhow!("Error: {:?} Not Support", self.json_value()))
            }
        }
        // Ok(vec.join(", "))
        Ok(vec)
    }
    fn to_sql(&self) -> anyhow::Result<String> {
        self.to_sql_with_concat("")
    }
}

impl<M> Insert<M> where M: ModelAble {
    pub fn new(value: Json) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//     // use serde_json::{json};
//     #[test]
//     fn to_sql() {
//         #[derive(Clone, Debug)]
//         struct User {}
//         impl ModelAble for User {}
//
//         let insert = Insert::<User>::new(10);
//         assert_eq!(insert.to_sql(), "Insert 10");
//     }
// }