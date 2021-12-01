use serde_json::{Value as Json, json};
use std::marker::PhantomData;
use std::default::Default;
use crate::traits::ModelAble;
use crate::statements::StatementAble;
use crate::nodes::SqlLiteral;
use crate::methods;

#[derive(Clone, Debug)]
pub struct Group<M: ModelAble> {
    pub value: Json,
    _marker: PhantomData<M>,
}

impl<M> StatementAble<M> for Group<M> where M: ModelAble {
    fn json_value(&self) -> Option<&Json> {
        Some(&self.value)
    }
    fn to_sql_literals(&self) -> Vec<SqlLiteral> {
        let mut vec = vec![];
        if let Some(json_value) = self.json_value() {
            match json_value {
                Json::Array(json_array) => {
                    for column_name in json_array.iter() {
                        if let Json::String(column_name) = column_name {
                            let table_column_name = methods::table_column_name::<M>(column_name);
                            vec.push(SqlLiteral::new(format!("{}", table_column_name)));
                        } else {
                            panic!("Error: Not Support");
                        }
                    }
                },
                Json::String(_) =>  vec.append(&mut StatementAble::to_sql_literals_default(self)),
                _ => panic!("Error: Not Support")
            }
        }
        // Ok(vec.join(" AND "))
        vec
    }
    fn to_sql(&self) -> String {
        self.to_sql_with_concat(", ")
    }
}

impl<M> Default for Group<M> where M: ModelAble {
    fn default() -> Self {
        Self {
            value: json!(["*"]),
            _marker: PhantomData
        }
    }
}

impl<M> Group<M> where M: ModelAble {
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

        let group = Group::<User>::new(json!("name, age"));
        assert_eq!(group.to_sql(), "name, age");

        let group = Group::<User>::new(json!(["name", "age"]));
        assert_eq!(group.to_sql(), "`users`.`name`, `users`.`age`");
    }
}
