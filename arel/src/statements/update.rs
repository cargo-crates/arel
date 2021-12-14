use serde_json::{Value as Json};
use std::marker::PhantomData;
use crate::traits::ArelAble;
use crate::statements::StatementAble;
use crate::nodes::SqlLiteral;

#[derive(Clone, Debug)]
pub struct Update<M: ArelAble> {
    value: Json,
    _marker: PhantomData<M>,
}

impl<M> StatementAble<M> for Update<M> where M: ArelAble {
    fn json_value(&self) -> Option<&Json> {
        Some(&self.value)
    }
    fn to_sql_literals(&self) -> anyhow::Result<Vec<SqlLiteral>> {
        let mut vec = vec![];
        if let Some(json_value) = self.json_value() {
            match json_value {
                Json::Object(json_object) => {
                    for column_name in json_object.keys() {
                        #[cfg(feature = "sqlite")]
                        let table_column_name = format!("\"{}\"", column_name);
                        // #[cfg(all(
                        //     not(feature = "sqlite"),
                        //     any(feature = "mysql", feature = "postgres", feature = "mssql")
                        // ))]
                        #[cfg(not(feature = "sqlite"))]
                        let table_column_name = crate::methods::table_column_name::<M>(column_name);
                        let json_value = json_object.get(column_name).unwrap();
                        vec.push(SqlLiteral::new(format!("{} = {}", table_column_name, self.json_value_sql(json_value)?)));
                    }
                },
                _ => vec.append(&mut StatementAble::to_sql_literals_default(self)?)
            }
        }
        // Ok(vec.join(", "))
        Ok(vec)
    }
    fn to_sql(&self) -> anyhow::Result<String> {
        self.to_sql_with_concat(", ")
    }
}

impl<M> Update<M> where M: ArelAble {
    pub fn new(value: Json) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }
}

#[allow(dead_code)]
#[cfg(test)]
#[cfg(feature = "mysql")]
mod tests {
    use crate as arel;
    use arel::prelude::*;
    use super::*;
    #[test]
    fn to_sql() {
        #[arel::arel]
        struct User {
            id: i64,
        }

        let update = Update::<User>::new(serde_json::json!({
            "name": "Tome",
            "age": 18,
            "active": true,
            "profile": null
        }));
        assert_eq!(update.to_sql().unwrap(), "`users`.`active` = 1, `users`.`age` = 18, `users`.`name` = 'Tome', `users`.`profile` = NULL");

        let update = Update::<User>::new(serde_json::json!("users.active = 1"));
        assert_eq!(update.to_sql().unwrap(), "users.active = 1");

        let update = Update::<User>::new(serde_json::json!(["users.active = ?", 1]));
        assert_eq!(update.to_sql().unwrap(), "users.active = 1");
    }
}
