use std::ops::{Bound, RangeBounds};
use serde_json::{Value as Json, json};
use std::marker::PhantomData;
use crate::statements::StatementAble;
use crate::nodes::SqlLiteral;
use crate::traits::ModelAble;
use crate::methods;

#[derive(Clone, Debug)]
pub enum JoinType {
    And,
    Or,
}

#[derive(Clone, Debug)]
pub struct Ops {
    // only one where statement columns join with AND|OR
    join_type: JoinType,
    is_not: bool,
    is_between: bool,
}

impl Ops {
    pub fn new(join_type: JoinType, is_not: bool, is_between: bool) -> Self {
        Self { join_type, is_not, is_between }
    }
}


#[derive(Clone, Debug)]
pub struct Where<M: ModelAble> {
    value: Json,
    ops: Ops,
    _marker: PhantomData<M>,
}

impl<M> StatementAble<M> for Where<M> where M: ModelAble {
    fn json_value(&self) -> Option<&Json> {
        Some(&self.value)
    }
    fn to_sql_literals(&self) -> Vec<SqlLiteral> {
        let mut vec = vec![];
        if let Some(json_value) = self.json_value() {
            match json_value {
                Json::Object(json_object) => {
                    for column_name in json_object.keys() {
                        let table_column_name = methods::table_column_name::<M>(column_name);
                        let json_value = json_object.get(column_name).unwrap();
                        vec.push(SqlLiteral::new(format!("{} {}", table_column_name, self.json_value_sql(json_value, true))));
                    }
                },
                _ => {
                    if self.ops.is_between {
                        match json_value {
                            Json::Array(json_array) if json_array.len() == 3 => {
                                let table_column_name = methods::table_column_name::<M>(&self.json_value_sql(json_array.get(0).unwrap(), false));
                                let between_sql = format!("BETWEEN {} AND {}", self.json_value_sql(json_array.get(1).unwrap(), false), self.json_value_sql(json_array.get(2).unwrap(), false));
                                if self.ops.is_not {
                                    vec.push(SqlLiteral::new(format!("{} NOT {}", table_column_name, between_sql)));
                                }
                                else {
                                    vec.push(SqlLiteral::new(format!("{} {}", table_column_name, between_sql)));
                                }
                            },
                            Json::String(_) => {
                                vec.append(&mut StatementAble::to_sql_literals_default(self))
                            },
                            _ => panic!("Error: Not Support")
                        }
                    } else if self.ops.is_not {
                        panic!("Error: Not Support")
                    } else {
                        vec.append(&mut StatementAble::to_sql_literals_default(self).into_iter().map(|mut i| {
                            i.raw_sql = format!("({})", i.raw_sql);
                            i
                        }).collect())
                    }
                },
            }
        }
        // Ok(vec.join(" AND "))
        vec
    }
    fn to_sql(&self) -> String {
        match self.ops.join_type {
            JoinType::And => self.to_sql_with_concat(" AND "),
            JoinType::Or => format!("({})", self.to_sql_with_concat(" OR "))
        }
    }
}

impl<M> Where<M> where M: ModelAble {
    pub fn new(value: Json, ops: Ops) -> Self {
        Self {
            value,
            ops,
            _marker: PhantomData,
        }
    }
    pub fn new_column_range<T: ToString>(column_name: &str, range: impl RangeBounds<T>, ops: Ops) -> Self {
        let table_column_name = methods::table_column_name::<M>(column_name);
        let mut raw_sql;

        if ops.is_between {
            match range.start_bound() {
                Bound::Unbounded => panic!("Error: Not Support"),
                Bound::Included(start) => {
                    match range.end_bound() {
                        Bound::Unbounded => panic!("Error: Not Support"),
                        Bound::Included(end) => raw_sql = format!("{} BETWEEN {} AND {}", table_column_name, start.to_string(), end.to_string()),
                        Bound::Excluded(end) => raw_sql = format!("{} BETWEEN {} AND {}", table_column_name, start.to_string(), end.to_string()),
                    }
                },
                Bound::Excluded(start) => {
                    match range.end_bound() {
                        Bound::Unbounded => panic!("Error: Not Support"),
                        Bound::Included(end) => raw_sql = format!("{} BETWEEN {} AND {}", table_column_name, start.to_string(), end.to_string()),
                        Bound::Excluded(end) => raw_sql = format!("{} BETWEEN {} AND {}", table_column_name, start.to_string(), end.to_string()),
                    }
                },
            }
        } else {
            match range.start_bound() {
                Bound::Unbounded => {
                    match range.end_bound() {
                        Bound::Unbounded => panic!("Error: Not Support"),
                        Bound::Included(end) => raw_sql = format!("{} <= {}", table_column_name, end.to_string()),
                        Bound::Excluded(end) => raw_sql = format!("{} < {}", table_column_name, end.to_string()),
                    }
                }
                Bound::Included(start) => {
                    raw_sql = format!("{} >= {}", table_column_name, start.to_string());
                    match range.end_bound() {
                        Bound::Unbounded => (),
                        Bound::Included(end) => raw_sql = format!("{} AND {} <= {}", raw_sql, table_column_name, end.to_string()),
                        Bound::Excluded(end) => raw_sql = format!("{} AND {} < {}", raw_sql, table_column_name, end.to_string()),
                    }
                },
                Bound::Excluded(start) => {
                    raw_sql = format!("{} > {}", table_column_name, start.to_string());
                    match range.end_bound() {
                        Bound::Unbounded => (),
                        Bound::Included(end) => raw_sql = format!("{} AND {} <= {}", raw_sql, table_column_name, end.to_string()),
                        Bound::Excluded(end) => raw_sql = format!("{} AND {} < {}", raw_sql, table_column_name, end.to_string()),
                    }
                },
            }
        }

        Self {
            value: json!(raw_sql),
            ops,
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
                if self.ops.is_between {
                    if json_array.len() == 2 && with_modifier {
                        let between_sql = format!("BETWEEN {} AND {}", values.get(0).unwrap(), values.get(1).unwrap());
                        if self.ops.is_not { format!("NOT {}", &between_sql) } else { format!("{}", &between_sql) }
                    } else {
                        panic!("Error: Not Support, Between statement Array must 2 length")
                    }
                } else {
                    let value = format!("({})", values.join(", "));
                    if with_modifier {
                        if self.ops.is_not { format!("NOT IN {}", value) } else { format!("IN {}", value) }
                    } else {
                        value
                    }
                }
            },
            Json::String(json_string) => {
                if with_modifier {
                    if self.ops.is_not { format!("!= '{}'", json_string) } else { format!("= '{}'", json_string) }
                } else {
                    format!("'{}'", json_string)
                }
            },
            Json::Number(json_number) => {
                if with_modifier {
                    if self.ops.is_not { format!("!= {}", json_number) } else { format!("= {}", json_number) }
                } else {
                    format!("{}", json_number)
                }
            },
            Json::Bool(json_bool) => {
                let value = if *json_bool {1} else {0};
                if with_modifier {
                    if self.ops.is_not { format!("!= {}", value) } else { format!("= {}", value) }
                } else {
                    format!("{}", value)
                }
            },
            Json::Null => {
                if with_modifier {
                    if self.ops.is_not { format!("IS NOT NULL") } else { format!("IS NULL") }
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
         }), Ops::new(JoinType::And, false, false));
        assert_eq!(r#where.to_sql(), "`users`.`active` = 1 AND `users`.`age` = 18 AND `users`.`gender` IN ('male', 'female') AND `users`.`name` = 'Tom' AND `users`.`profile` IS NULL AND `users`.`role` IN (1, 2)");
        let r#where = Where::<User>::new(json!({
            "name": "Tom",
            "age": 18,
             "gender": ["male", "female"],
             "active": true,
             "profile": null
         }), Ops::new(JoinType::And, true, false));
        assert_eq!(r#where.to_sql(), "`users`.`active` != 1 AND `users`.`age` != 18 AND `users`.`gender` NOT IN ('male', 'female') AND `users`.`name` != 'Tom' AND `users`.`profile` IS NOT NULL");

        let r#where = Where::<User>::new(json!("age > 18"), Ops::new(JoinType::And, false, false));
        assert_eq!(r#where.to_sql(), "(age > 18)");

        let r#where = Where::<User>::new(json!(["age > 18"]), Ops::new(JoinType::And, false, false));
        assert_eq!(r#where.to_sql(), "(age > 18)");
        let r#where = Where::<User>::new(json!(["name = ? AND age > ? AND gender in ? AND enable = ?", "Tom", 18, ["male", "female"], true]), Ops::new(JoinType::And, false, false));
        assert_eq!(r#where.to_sql(), "(name = 'Tom' AND age > 18 AND gender in ('male', 'female') AND enable = 1)");

        //between
        let r#where = Where::<User>::new(json!({"age": [18, 30]}), Ops::new(JoinType::And, false, true));
        assert_eq!(r#where.to_sql(), "`users`.`age` BETWEEN 18 AND 30");
        let r#where = Where::<User>::new(json!({"age": [18, 30]}), Ops::new(JoinType::And, true, true));
        assert_eq!(r#where.to_sql(), "`users`.`age` NOT BETWEEN 18 AND 30");
        let r#where = Where::<User>::new(json!(["age", 18, 30]), Ops::new(JoinType::And, false, true));
        assert_eq!(r#where.to_sql(), "`users`.`'age'` BETWEEN 18 AND 30");
        let r#where = Where::<User>::new(json!(["age", 18, 30]), Ops::new(JoinType::And, true, true));
        assert_eq!(r#where.to_sql(), "`users`.`'age'` NOT BETWEEN 18 AND 30");
        // or between
        let r#where = Where::<User>::new(json!({"age": [18, 30], "name": "Tom"}), Ops::new(JoinType::Or, false, false));
        assert_eq!(r#where.to_sql(), "(`users`.`age` IN (18, 30) OR `users`.`name` = 'Tom')");
        let r#where = Where::<User>::new(json!({"age": [18, 30], "name": "Tom"}), Ops::new(JoinType::Or, true, false));
        assert_eq!(r#where.to_sql(), "(`users`.`age` NOT IN (18, 30) OR `users`.`name` != 'Tom')");
        let r#where = Where::<User>::new(json!({"age": [18, 30], "name": "Tom"}), Ops::new(JoinType::Or, true, true));
        assert_eq!(r#where.to_sql(), "(`users`.`age` NOT BETWEEN 18 AND 30 OR `users`.`name` != 'Tom')");
        let r#where = Where::<User>::new(json!(["age", 18, 30]), Ops::new(JoinType::Or, true, true));
        assert_eq!(r#where.to_sql(), "(`users`.`'age'` NOT BETWEEN 18 AND 30)");
        // range
        let r#where = Where::<User>::new_column_range("age", ..18, Ops::new(JoinType::And, false, false));
        assert_eq!(r#where.to_sql(), "(`users`.`age` < 18)");
        let r#where = Where::<User>::new_column_range("age", 0..18, Ops::new(JoinType::And, false, true));
        assert_eq!(r#where.to_sql(), "`users`.`age` BETWEEN 0 AND 18");
    }
}

