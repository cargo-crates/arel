use serde_json::{Value as Json};
use std::marker::PhantomData;
use crate::traits::ArelAble;
use crate::statements::StatementAble;
use crate::collectors::Sql;
use crate::methods;

#[derive(Clone, Debug)]
pub struct Insert<M: ArelAble> {
    value: Json,
    _marker: PhantomData<M>,
}

impl<M> StatementAble<M> for Insert<M> where M: ArelAble {
    fn json_value(&self) -> Option<&Json> {
        Some(&self.value)
    }
    fn to_sub_sqls(&self) -> anyhow::Result<Vec<Sql>> {
        let mut vec = vec![];
        if let Some(json_value) = self.json_value() {
            match json_value {
                Json::Object(json_object) => {
                    let mut keys = vec![];
                    let mut values = vec![];
                    for column_name in json_object.keys() {
                        keys.push(format!("`{}`", column_name));
                        let json_value = json_object.get(column_name).unwrap();
                        values.push(Self::value_sql_string_from_json(json_value)?);
                    }
                    if keys.len() > 0  && values.len() > 0 {
                        vec.push(Sql::new(format!("{} ({}) VALUES ({})", methods::quote_table_name(&M::table_name()), keys.join(", "), values.join(", "))));
                    } else {
                        #[cfg(feature = "sqlite")]
                        vec.push(Sql::new(format!("{} DEFAULT VALUES", methods::quote_table_name(&M::table_name()))));
                        #[cfg(not(feature = "sqlite"))]
                        vec.push(Sql::new(format!("{} VALUES ()", methods::quote_table_name(&M::table_name()))));
                    }
                },
                _ => return Err(anyhow::anyhow!("Error: {:?} Not Support", self.json_value()))
            }
        }
        // Ok(vec.join(", "))
        Ok(vec)
    }
    fn to_sql(&self) -> anyhow::Result<Sql> {
        self.to_sql_with_concat("")
    }
}

impl<M> Insert<M> where M: ArelAble {
    pub fn new(value: Json) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }
}


// #[cfg(test)]
// #[cfg(feature = "mysql")]
// mod tests {
//     use super::*;
//     // use serde_json::{json};
//     #[test]
//     fn to_sql() {
//         #[derive(Clone, Debug)]
//         struct User {}
//         impl ArelAble for User {}
//
//         let insert = Insert::<User>::new(10);
//         assert_eq!(insert.to_sql(), "Insert 10");
//     }
// }