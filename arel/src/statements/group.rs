use serde_json::{Value as Json, json};
use std::marker::PhantomData;
use std::default::Default;
use crate::traits::ArelAble;
use crate::statements::StatementAble;
use crate::collectors::Sql;
use crate::methods;

#[derive(Clone, Debug)]
pub struct Group<M: ArelAble> {
    pub value: Json,
    _marker: PhantomData<M>,
}

impl<M> StatementAble<M> for Group<M> where M: ArelAble {
    fn json_value(&self) -> Option<&Json> {
        Some(&self.value)
    }
    fn to_sub_sqls(&self) -> anyhow::Result<Vec<Sql>> {
        let mut vec = vec![];
        if let Some(json_value) = self.json_value() {
            match json_value {
                Json::Array(json_array) => {
                    for column_name in json_array.iter() {
                        if let Json::String(column_name) = column_name {
                            let table_column_name = methods::table_column_name::<M>(column_name);
                            vec.push(Sql::new(format!("{}", table_column_name)));
                        } else {
                            return Err(anyhow::anyhow!("Error: {:?} Not Support", self.json_value()))
                        }
                    }
                },
                Json::String(_) =>  vec.append(&mut StatementAble::default_to_sub_sqls(self)?),
                _ => return Err(anyhow::anyhow!("Error: {:?} Not Support", self.json_value()))
            }
        }
        // Ok(vec.join(" AND "))
        Ok(vec)
    }
    fn to_sql(&self) -> anyhow::Result<Sql> {
        self.to_sql_with_concat(", ")
    }
}

impl<M> Default for Group<M> where M: ArelAble {
    fn default() -> Self {
        Self {
            value: json!(["*"]),
            _marker: PhantomData
        }
    }
}

impl<M> Group<M> where M: ArelAble {
    pub fn new(value: Json) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
#[cfg(feature = "mysql")]
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

        let group = Group::<User>::new(json!("name, age"));
        assert_eq!(group.to_sql_string().unwrap(), "name, age");

        let group = Group::<User>::new(json!(["name", "age"]));
        assert_eq!(group.to_sql_string().unwrap(), "`users`.`name`, `users`.`age`");
    }
}
