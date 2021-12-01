use serde_json::{Value as Json, json};
use std::marker::PhantomData;
use std::default::Default;
use crate::traits::ModelAble;
use crate::statements::StatementAble;
use crate::nodes::SqlLiteral;
use crate::methods;

#[derive(Clone, Debug)]
pub struct Select<M: ModelAble> {
    pub value: Json,
    pub distinct: bool,
    _marker: PhantomData<M>,
}

impl<M> StatementAble<M> for Select<M> where M: ModelAble {
    fn value(&self) -> &Json {
        &self.value
    }
    fn to_sql_literals(&self) -> Vec<SqlLiteral> {
        let mut vec = vec![];
        match self.value() {
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
        // Ok(vec.join(" AND "))
        vec
    }
    fn to_sql(&self) -> String {
        let mut sql = "SELECT ".to_string();
        if self.distinct {
            sql.push_str("DISTINCT ");
        }
        sql.push_str(&self.to_sql_with_concat(", "));
        sql.to_string()
    }
}

impl<M> Default for Select<M> where M: ModelAble {
    fn default() -> Self {
        Self {
            value: json!(["*"]),
            distinct: false,
            _marker: PhantomData
        }
    }
}

impl<M> Select<M> where M: ModelAble {
    pub fn new(value: Json, distinct: bool) -> Self {
        Self {
            value,
            distinct,
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

        let select = Select::<User>::new(json!("name, age"), false);
        assert_eq!(select.to_sql(), "SELECT name, age");
        let select = Select::<User>::new(json!("name, age"), true);
        assert_eq!(select.to_sql(), "SELECT DISTINCT name, age");

        let select = Select::<User>::new(json!(["name", "age"]), false);
        assert_eq!(select.to_sql(), "SELECT `users`.`name`, `users`.`age`");
        let select = Select::<User>::new(json!(["name", "age"]), true);
        assert_eq!(select.to_sql(), "SELECT DISTINCT `users`.`name`, `users`.`age`");
    }
}
