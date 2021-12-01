use serde_json::{Value as Json};
use std::marker::PhantomData;
use crate::statements::StatementAble;
use crate::nodes::SqlLiteral;
use crate::traits::ModelAble;
use crate::methods;

#[derive(Clone, Debug)]
pub struct Where<M: ModelAble> {
    value: Json,
    is_where_not: bool,
    _marker: PhantomData<M>,
}

impl<M> StatementAble<M> for Where<M> where M: ModelAble {
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
                    vec.push(SqlLiteral::new(format!("{} {}", table_column_name, self.json_value_sql(json_value, true))));
                }
            },
            _ => vec.append(&mut StatementAble::to_sql_literals_default(self).into_iter().map(|mut i| {
                    i.raw_sql = format!("({})", i.raw_sql);
                    i
                }).collect()),
        }
        // Ok(vec.join(" AND "))
        vec
    }
    fn to_sql(&self) -> String {
        self.to_sql_with_concat(" AND ")
    }
}

impl<M> Where<M> where M: ModelAble {
    pub fn new(value: Json, is_where_not: bool) -> Self {
        Self {
            value,
            is_where_not,
            _marker: PhantomData,
        }
    }
    fn json_value_sql(&self, json_value: &Json, with_modifier: bool) -> String {
        match json_value {
            Json::Array(json_array) => {
                let mut values = vec![];
                for json_value in json_array.iter() {
                    values.push(self.json_value_sql(json_value, false));
                }
                let value = format!("({})", values.join(", "));
                if with_modifier {
                    if self.is_where_not { format!("NOT IN {}", value) } else { format!("IN {}", value) }
                } else {
                    value
                }
            },
            Json::String(json_string) => {
                if with_modifier {
                    if self.is_where_not { format!("!= '{}'", json_string) } else { format!("= '{}'", json_string) }
                } else {
                    format!("'{}'", json_string)
                }
            },
            Json::Number(json_number) => {
                if with_modifier {
                    if self.is_where_not { format!("!= {}", json_number) } else { format!("= {}", json_number) }
                } else {
                    format!("{}", json_number)
                }
            },
            Json::Bool(json_bool) => {
                let value = if *json_bool {1} else {0};
                if with_modifier {
                    if self.is_where_not { format!("!= {}", value) } else { format!("= {}", value) }
                } else {
                    format!("{}", value)
                }
            },
            Json::Null => {
                if with_modifier {
                    if self.is_where_not { format!("IS NOT NULL") } else { format!("IS NULL") }
                } else {
                    panic!("Error: Not Support")
                }
            },
            _ => panic!("Error: Not Support")
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
        let r#where = Where::<User>::new(json!({
            "name": "Tom",
            "age": 18,
             "gender": ["male", "female"],
             "role": [1, 2],
             "active": true,
             "profile": null
         }), false);
        assert_eq!(r#where.to_sql(), "`users`.`active` = 1 AND `users`.`age` = 18 AND `users`.`gender` IN ('male', 'female') AND `users`.`name` = 'Tom' AND `users`.`profile` IS NULL AND `users`.`role` IN (1, 2)");
        let r#where = Where::<User>::new(json!({
            "name": "Tom",
            "age": 18,
             "gender": ["male", "female"],
             "active": true,
             "profile": null
         }), true);
        assert_eq!(r#where.to_sql(), "`users`.`active` != 1 AND `users`.`age` != 18 AND `users`.`gender` NOT IN ('male', 'female') AND `users`.`name` != 'Tom' AND `users`.`profile` IS NOT NULL");

        let r#where = Where::<User>::new(json!("age > 18"), false);
        assert_eq!(r#where.to_sql(), "(age > 18)");

        let r#where = Where::<User>::new(json!(["age > 18"]), false);
        assert_eq!(r#where.to_sql(), "(age > 18)");
        let r#where = Where::<User>::new(json!(["name = ? AND age > ? AND gender in ? AND enable = ?", "Tom", 18, ["male", "female"], true]), false);
        assert_eq!(r#where.to_sql(), "(name = 'Tom' AND age > 18 AND gender in ('male', 'female') AND enable = 1)");
    }
}

