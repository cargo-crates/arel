use std::ops::{Bound, RangeBounds};
use serde_json::{Value as Json};
use std::marker::PhantomData;
use crate::statements::StatementAble;
use crate::nodes::SqlLiteral;
use crate::traits::ArelAble;
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
pub struct Where<M: ArelAble> {
    value: Json,
    ops: Ops,
    _marker: PhantomData<M>,
}

impl<M> StatementAble<M> for Where<M> where M: ArelAble {
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
                        vec.push(SqlLiteral::new(format!("{} {}", table_column_name, self.json_value_sql(json_value, true)?)));
                    }
                },
                _ => {
                    if self.ops.is_between {
                        match json_value {
                            // json([column_name, start, end])
                            Json::Array(json_array) if json_array.len() == 3 => {
                                let table_column_name = methods::table_column_name::<M>(&self.json_value_sql(json_array.get(0).unwrap(), false)?);
                                let between_sql = format!("BETWEEN {} AND {}", self.json_value_sql(json_array.get(1).unwrap(), false)?, self.json_value_sql(json_array.get(2).unwrap(), false)?);
                                if self.ops.is_not {
                                    vec.push(SqlLiteral::new(format!("{} NOT {}", table_column_name, between_sql)));
                                }
                                else {
                                    vec.push(SqlLiteral::new(format!("{} {}", table_column_name, between_sql)));
                                }
                            },
                            Json::String(_) => {
                                vec.append(&mut StatementAble::to_sql_literals_default(self)?)
                            },
                            _ => return Err(anyhow::anyhow!("Error: {:?} Not Support", self.json_value()))
                        }
                    } else if self.ops.is_not {
                        return Err(anyhow::anyhow!("Error: {:?} Not Support", self.json_value()))
                    } else {
                        vec.append(&mut StatementAble::to_sql_literals_default(self)?)
                    }
                },
            }
        }
        // Ok(vec.join(" AND "))
        Ok(vec)
    }
    fn to_sql(&self) -> anyhow::Result<String> {
        match self.ops.join_type {
            JoinType::And => Ok(self.to_sql_with_concat(" AND ")?),
            JoinType::Or => Ok(format!("({})", self.to_sql_with_concat(" OR ")?))
        }
    }
}

impl<M> Where<M> where M: ArelAble {
    pub fn new(value: Json, ops: Ops) -> Self {
        Self {
            value,
            ops,
            _marker: PhantomData,
        }
    }
    fn json_value_sql(&self, json_value: &Json, with_modifier: bool) -> anyhow::Result<String> {
        match json_value {
            Json::Array(json_array) => {
                let mut values = vec![];
                for json_value in json_array.iter() {
                    values.push(self.json_value_sql(json_value, false)?);
                }
                if self.ops.is_between {
                    if json_array.len() == 2 && with_modifier {
                        let between_sql = format!("BETWEEN {} AND {}", values.get(0).unwrap(), values.get(1).unwrap());
                        if self.ops.is_not { Ok(format!("NOT {}", &between_sql)) } else { Ok(format!("{}", &between_sql)) }
                    } else {
                        return Err(anyhow::anyhow!("Error: {:?} Not Support, Between statement Array must 2 length", self.json_value()))
                    }
                } else {
                    let value = format!("({})", values.join(", "));
                    if with_modifier {
                        if self.ops.is_not { Ok(format!("NOT IN {}", value)) } else { Ok(format!("IN {}", value)) }
                    } else {
                        Ok(value)
                    }
                }
            },
            Json::String(json_string) => {
                if with_modifier {
                    if self.ops.is_not { Ok(format!("!= '{}'", json_string)) } else { Ok(format!("= '{}'", json_string)) }
                } else {
                    Ok(format!("'{}'", json_string))
                }
            },
            Json::Number(json_number) => {
                if with_modifier {
                    if self.ops.is_not { Ok(format!("!= {}", json_number)) } else { Ok(format!("= {}", json_number)) }
                } else {
                    Ok(format!("{}", json_number))
                }
            },
            Json::Bool(json_bool) => {
                let value = if *json_bool {1} else {0};
                if with_modifier {
                    if self.ops.is_not { Ok(format!("!= {}", value)) } else { Ok(format!("= {}", value)) }
                } else {
                    Ok(format!("{}", value))
                }
            },
            Json::Null => {
                if with_modifier {
                    if self.ops.is_not { Ok(format!("IS NOT NULL")) } else { Ok(format!("IS NULL")) }
                } else {
                    return Err(anyhow::anyhow!("Error: {:?} Not Support", self.json_value()))
                }
            },
            _ => return Err(anyhow::anyhow!("Error: {:?} Not Support", self.json_value()))
        }
    }
}

pub fn help_range_to_sql<T: ToString>(table_column_name: &str, range: impl RangeBounds<T>) -> anyhow::Result<String> {
    let raw_sql;

    match range.start_bound() {
        Bound::Unbounded => {
            match range.end_bound() {
                Bound::Unbounded => return Err(anyhow::anyhow!("Error: Not Support")),
                Bound::Included(end) => raw_sql = format!("{} <= {}", table_column_name, end.to_string()),
                Bound::Excluded(end) => raw_sql = format!("{} < {}", table_column_name, end.to_string()),
            }
        },
        Bound::Included(start) => {
            match range.end_bound() {
                Bound::Unbounded => raw_sql = format!("{} >= {}", table_column_name, start.to_string()),
                Bound::Included(end) => raw_sql = format!("{} BETWEEN {} AND {}", table_column_name, start.to_string(), end.to_string()),
                Bound::Excluded(end) => raw_sql = format!("{} >= {} AND {} < {}", table_column_name, start.to_string(), table_column_name, end.to_string()),
            }
        },
        Bound::Excluded(start) => {
            match range.end_bound() {
                Bound::Unbounded => raw_sql = format!("{} > {}", table_column_name, start.to_string()),
                Bound::Included(end) => raw_sql = format!("{} > {} AND {} <= {}", table_column_name, start.to_string(), table_column_name, end.to_string()),
                Bound::Excluded(end) => raw_sql = format!("{} > {} AND {} < {}", table_column_name, start.to_string(), table_column_name, end.to_string()),
            }
        },
    }
    Ok(raw_sql)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json};
    #[test]
    fn to_sql() {
        #[derive(Clone, Debug)]
        struct User {}
        impl ArelAble for User {}
        let r#where = Where::<User>::new(json!({
            "name": "Tom",
            "age": 18,
             "gender": ["male", "female"],
             "role": [1, 2],
             "active": true,
             "profile": null
         }), Ops::new(JoinType::And, false, false));
        assert_eq!(r#where.to_sql().unwrap(), "`users`.`active` = 1 AND `users`.`age` = 18 AND `users`.`gender` IN ('male', 'female') AND `users`.`name` = 'Tom' AND `users`.`profile` IS NULL AND `users`.`role` IN (1, 2)");
        let r#where = Where::<User>::new(json!({
            "name": "Tom",
            "age": 18,
             "gender": ["male", "female"],
             "active": true,
             "profile": null
         }), Ops::new(JoinType::And, true, false));
        assert_eq!(r#where.to_sql().unwrap(), "`users`.`active` != 1 AND `users`.`age` != 18 AND `users`.`gender` NOT IN ('male', 'female') AND `users`.`name` != 'Tom' AND `users`.`profile` IS NOT NULL");

        let r#where = Where::<User>::new(json!("age > 18"), Ops::new(JoinType::And, false, false));
        assert_eq!(r#where.to_sql().unwrap(), "age > 18");

        let r#where = Where::<User>::new(json!(["age > 18"]), Ops::new(JoinType::And, false, false));
        assert_eq!(r#where.to_sql().unwrap(), "age > 18");
        let r#where = Where::<User>::new(json!(["name = ? AND age > ? AND gender in ? AND enable = ?", "Tom", 18, ["male", "female"], true]), Ops::new(JoinType::And, false, false));
        assert_eq!(r#where.to_sql().unwrap(), "name = 'Tom' AND age > 18 AND gender in ('male', 'female') AND enable = 1");

        //between
        let r#where = Where::<User>::new(json!({"age": [18, 30]}), Ops::new(JoinType::And, false, true));
        assert_eq!(r#where.to_sql().unwrap(), "`users`.`age` BETWEEN 18 AND 30");
        let r#where = Where::<User>::new(json!({"age": [18, 30]}), Ops::new(JoinType::And, true, true));
        assert_eq!(r#where.to_sql().unwrap(), "`users`.`age` NOT BETWEEN 18 AND 30");
        let r#where = Where::<User>::new(json!(["age", 18, 30]), Ops::new(JoinType::And, false, true));
        assert_eq!(r#where.to_sql().unwrap(), "`users`.`'age'` BETWEEN 18 AND 30");
        let r#where = Where::<User>::new(json!(["age", 18, 30]), Ops::new(JoinType::And, true, true));
        assert_eq!(r#where.to_sql().unwrap(), "`users`.`'age'` NOT BETWEEN 18 AND 30");
        // or between
        let r#where = Where::<User>::new(json!({"age": [18, 30], "name": "Tom"}), Ops::new(JoinType::Or, false, false));
        assert_eq!(r#where.to_sql().unwrap(), "(`users`.`age` IN (18, 30) OR `users`.`name` = 'Tom')");
        let r#where = Where::<User>::new(json!({"age": [18, 30], "name": "Tom"}), Ops::new(JoinType::Or, true, false));
        assert_eq!(r#where.to_sql().unwrap(), "(`users`.`age` NOT IN (18, 30) OR `users`.`name` != 'Tom')");
        let r#where = Where::<User>::new(json!({"age": [18, 30], "name": "Tom"}), Ops::new(JoinType::Or, true, true));
        assert_eq!(r#where.to_sql().unwrap(), "(`users`.`age` NOT BETWEEN 18 AND 30 OR `users`.`name` != 'Tom')");
        let r#where = Where::<User>::new(json!(["age", 18, 30]), Ops::new(JoinType::Or, true, true));
        assert_eq!(r#where.to_sql().unwrap(), "(`users`.`'age'` NOT BETWEEN 18 AND 30)");
    }
}

