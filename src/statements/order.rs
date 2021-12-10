use serde_json::{Value as Json};
use std::marker::PhantomData;
use crate::traits::ArelAble;
use crate::statements::StatementAble;
use crate::nodes::SqlLiteral;
use crate::methods;

#[derive(Clone, Debug)]
pub struct Order<M: ArelAble> {
    value: Json,
    _marker: PhantomData<M>,
}

impl<M> StatementAble<M> for Order<M> where M: ArelAble {
    fn json_value(&self) -> Option<&Json> {
        Some(&self.value)
    }
    fn to_sql_literals(&self) -> anyhow::Result<Vec<SqlLiteral>> {
        let mut vec = vec![];
        if let Some(json_value) = self.json_value() {
            match json_value {
                Json::Object(json_object) => {
                    for column_name in json_object.keys() {
                        let table_column_name = methods::table_column_name::<M>(column_name);
                        let json_value = json_object.get(column_name).unwrap();
                        vec.push(SqlLiteral::new(format!("{} {}", table_column_name, self.json_value_sql(json_value)?.to_uppercase())));
                    }
                },
                Json::Array(json_array) => {
                    for column_name in json_array.iter() {
                        if let Json::String(json_string) = column_name {
                            vec.push(SqlLiteral::new(format!("{}", json_string)));
                        } else {
                            return Err(anyhow::anyhow!("Error: {:?} Not Support", self.json_value()))
                        }
                    }
                },
                _ =>  vec.append(&mut StatementAble::to_sql_literals_default(self)?),
            }
        }
        // Ok(vec.join(" AND "))
        Ok(vec)
    }
    fn to_sql(&self) -> anyhow::Result<String> {
        Ok(format!("{}", self.to_sql_with_concat(", ")?))
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

impl<M> Order<M> where M: ArelAble {
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

        let order = Order::<User>::new(json!("name desc"));
        assert_eq!(order.to_sql().unwrap(), "name desc");

        let order = Order::<User>::new(json!(["name desc", "age asc"]));
        assert_eq!(order.to_sql().unwrap(), "name desc, age asc");
        let order = Order::<User>::new(json!({
            "name": "desc",
            "age": "asc"
        }));
        assert_eq!(order.to_sql().unwrap(), "`users`.`age` ASC, `users`.`name` DESC");
    }
}
