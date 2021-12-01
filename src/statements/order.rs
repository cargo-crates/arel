use serde_json::{Value as Json};
use std::marker::PhantomData;
use crate::traits::ModelAble;
use crate::statements::StatementAble;
use crate::nodes::SqlLiteral;
use crate::methods;

#[derive(Clone, Debug)]
pub struct Order<M: ModelAble> {
    value: Json,
    _marker: PhantomData<M>,
}

impl<M> StatementAble<M> for Order<M> where M: ModelAble {
    fn value(&self) -> &Json {
        &self.value
    }
    fn to_sql_literals(&self) -> Vec<SqlLiteral> {
        let mut vec = vec![];
        match self.value() {
            Json::Object(json_object) => {
                for column_name in json_object.keys() {
                    let table_column_name = methods::table_column_name::<M>(column_name);
                    let json_value = json_object.get(column_name).unwrap();
                    vec.push(SqlLiteral::new(format!("{} {}", table_column_name, self.json_value_sql(json_value).to_uppercase())));
                }
            },
            Json::Array(json_array) => {
                for column_name in json_array.iter() {
                    if let Json::String(json_string) = column_name {
                        vec.push(SqlLiteral::new(format!("{}", json_string)));
                    } else {
                        panic!("Error: Not Support");
                    }
                }
            },
            _ =>  vec.append(&mut StatementAble::to_sql_literals_default(self)),
        }
        // Ok(vec.join(" AND "))
        vec
    }
    fn to_sql(&self) -> String {
        format!("{}", self.to_sql_with_concat(", "))
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

impl<M> Order<M> where M: ModelAble {
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

        let order = Order::<User>::new(json!("name desc"));
        assert_eq!(order.to_sql(), "name desc");

        let order = Order::<User>::new(json!(["name desc", "age asc"]));
        assert_eq!(order.to_sql(), "name desc, age asc");
        let order = Order::<User>::new(json!({
            "name": "desc",
            "age": "asc"
        }));
        assert_eq!(order.to_sql(), "`users`.`age` ASC, `users`.`name` DESC");
    }
}
