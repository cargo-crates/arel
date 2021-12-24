use std::ops::{Bound, RangeBounds};
use serde_json::{Value as Json, json};
use std::marker::PhantomData;
use crate::statements::{self, StatementAble};
use crate::collectors::Sql;
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
    fn to_sub_sqls(&self) -> anyhow::Result<Vec<Sql>> {
        let mut vec = vec![];
        if let Some(json_value) = self.json_value() {
            match json_value {
                Json::Object(json_object) => {
                    for column_name in json_object.keys() {
                        let table_column_name = methods::table_column_name::<M>(column_name);
                        let json_value = json_object.get(column_name).unwrap();

                        let mut sql = Sql::new(table_column_name);
                        sql.push_str(" ").push_from_sql(&self.value_sql_from_json(json_value, true)?);
                        vec.push(sql);
                    }
                },
                _ => {
                    if self.ops.is_between {
                        match json_value {
                            // json([column_name, start, end])
                            Json::Array(json_array) if json_array.len() == 3 => {
                                let table_column_name = methods::table_column_name::<M>(&self.value_sql_from_json(json_array.get(0).unwrap(), false)?.value);
                                let start_sql = self.value_sql_from_json(json_array.get(1).unwrap(), false)?;
                                let end_sql = self.value_sql_from_json(json_array.get(2).unwrap(), false)?;
                                // let between_sql = format!("BETWEEN {} AND {}", self.value_sql_from_json(json_array.get(1).unwrap(), false)?, );
                                if self.ops.is_not {
                                    let mut sql = Sql::new(table_column_name);
                                    sql.push_str(" NOT BETWEEN ").push_from_sql(&start_sql).push_str(" AND ").push_from_sql(&end_sql);
                                    vec.push(sql);
                                }
                                else {
                                    let mut sql = Sql::new(table_column_name);
                                    sql.push_str(" BETWEEN ").push_from_sql(&start_sql).push_str(" AND ").push_from_sql(&end_sql);
                                    vec.push(sql);
                                }
                            },
                            Json::String(_) => {
                                vec.append(&mut StatementAble::default_to_sub_sqls(self)?)
                            },
                            _ => return Err(anyhow::anyhow!("Error: {:?} Not Support", self.json_value()))
                        }
                    } else if self.ops.is_not {
                        return Err(anyhow::anyhow!("Error: {:?} Not Support", self.json_value()))
                    } else {
                        vec.append(&mut self.default_to_sub_sqls()?)
                    }
                },
            }
        }
        // Ok(vec.join(" AND "))
        Ok(vec)
    }
    fn to_sql(&self) -> anyhow::Result<Sql> {
        match self.ops.join_type {
            JoinType::And => self.to_sql_with_concat(" AND "),
            JoinType::Or => {
                let mut sql = self.to_sql_with_concat(" OR ")?;
                sql.value = format!("({})", &sql.value);
                Ok(sql)
            }
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
    fn value_sql_from_json(&self, json_value: &Json, with_modifier: bool) -> anyhow::Result<Sql> {
        let mut sql = Sql::default();
        match json_value {
            Json::Array(json_array) => {
                let mut values = vec![];
                for json_value in json_array.iter() {
                    values.push(self.value_sql_from_json(json_value, false)?);
                }
                if self.ops.is_between {
                    if json_array.len() == 2 && with_modifier {
                        let start_sql = values.get(0).unwrap();
                        let end_sql = values.get(1).unwrap();
                        if self.ops.is_not {
                            sql.push_str("NOT BETWEEN ").push_from_sql(start_sql).push_str(" AND ").push_from_sql(end_sql);
                            Ok(sql)
                        } else {
                            sql.push_str("BETWEEN ").push_from_sql(start_sql).push_str(" AND ").push_from_sql(end_sql);
                            Ok(sql)
                        }
                    } else {
                        return Err(anyhow::anyhow!("Error: {:?} Not Support, Between statement Array must 2 length", self.json_value()))
                    }
                } else {
                    if with_modifier {
                        if self.ops.is_not {
                            sql.push_str("NOT IN (").push_from_sqls(&values, ", ").push_str(")");
                            Ok(sql)
                        } else {
                            sql.push_str("IN (").push_from_sqls(&values, ", ").push_str(")");
                            Ok(sql)
                        }
                    } else {
                        sql.push_from_sqls(&values, ", ");
                        Ok(sql)
                    }
                }
            },
            Json::String(json_string) => {
                if with_modifier {
                    if self.ops.is_not {
                        sql.push_str(&format!("!= '{}'", json_string));
                        Ok(sql)
                    } else {
                        sql.push_str(&format!("= '{}'", json_string));
                        Ok(sql)
                    }
                } else {
                    sql.push_str(&format!("'{}'", json_string));
                    Ok(sql)
                }
            },
            Json::Number(json_number) => {
                if with_modifier {
                    if self.ops.is_not {
                        sql.push_str(&format!("!= {}", json_number));
                        Ok(sql)
                    } else {
                        sql.push_str(&format!("= {}", json_number));
                        Ok(sql)
                    }
                } else {
                    sql.push_str(&format!("{}", json_number));
                    Ok(sql)
                }
            },
            Json::Bool(json_bool) => {
                let value = if *json_bool {1} else {0};
                if with_modifier {
                    if self.ops.is_not {
                        sql.push_str(&format!("!= {}", value));
                        Ok(sql)
                    } else {
                        sql.push_str(&format!("= {}", value));
                        Ok(sql)
                    }
                } else {
                    sql.push_str(&format!("{}", value));
                    Ok(sql)
                }
            },
            Json::Null => {
                if with_modifier {
                    if self.ops.is_not {
                        sql.push_str(&format!("IS NOT NULL"));
                        Ok(sql)
                    } else {
                        sql.push_str(&format!("IS NULL"));
                        Ok(sql)
                    }
                } else {
                    return Err(anyhow::anyhow!("Error: {:?} Not Support", self.json_value()))
                }
            },
            _ => return Err(anyhow::anyhow!("Error: {:?} Not Support", self.json_value()))
        }
    }
}

pub fn help_range_to_sql<T: serde::Serialize>(table_column_name: &str, range: impl RangeBounds<T>) -> anyhow::Result<String> {
    let raw_sql;

    let get_bound_value = |value: &T| {
        let json_value = json!(value);
        statements::core_value_sql_string_from_json(&json_value)
    };

    match range.start_bound() {
        Bound::Unbounded => {
            match range.end_bound() {
                Bound::Unbounded => return Err(anyhow::anyhow!("Error: Not Support")),
                Bound::Included(end) => raw_sql = format!("{} <= {}", table_column_name, get_bound_value(end)?),
                Bound::Excluded(end) => raw_sql = format!("{} < {}", table_column_name, get_bound_value(end)?),
            }
        },
        Bound::Included(start) => {
            match range.end_bound() {
                Bound::Unbounded => {
                    raw_sql = format!("{} >= {}", table_column_name, get_bound_value(start)?)
                },
                Bound::Included(end) => raw_sql = format!("{} BETWEEN {} AND {}", table_column_name, get_bound_value(start)?, get_bound_value(end)?),
                Bound::Excluded(end) => raw_sql = format!("{} >= {} AND {} < {}", table_column_name, get_bound_value(start)?, table_column_name, get_bound_value(end)?),
            }
        },
        Bound::Excluded(start) => {
            match range.end_bound() {
                Bound::Unbounded => raw_sql = format!("{} > {}", table_column_name, get_bound_value(start)?),
                Bound::Included(end) => raw_sql = format!("{} > {} AND {} <= {}", table_column_name, get_bound_value(start)?, table_column_name, get_bound_value(end)?),
                Bound::Excluded(end) => raw_sql = format!("{} > {} AND {} < {}", table_column_name, get_bound_value(start)?, table_column_name, get_bound_value(end)?),
            }
        },
    }
    Ok(raw_sql)
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

        let r#where = Where::<User>::new(json!({
            "name": "Tom",
            "age": 18,
             "gender": ["male", "female"],
             "role": [1, 2],
             "active": true,
             "profile": null
         }), Ops::new(JoinType::And, false, false));
        assert_eq!(r#where.to_sql_string().unwrap(), "`users`.`active` = 1 AND `users`.`age` = 18 AND `users`.`gender` IN ('male', 'female') AND `users`.`name` = 'Tom' AND `users`.`profile` IS NULL AND `users`.`role` IN (1, 2)");
        let r#where = Where::<User>::new(json!({
            "name": "Tom",
            "age": 18,
             "gender": ["male", "female"],
             "active": true,
             "profile": null
         }), Ops::new(JoinType::And, true, false));
        assert_eq!(r#where.to_sql_string().unwrap(), "`users`.`active` != 1 AND `users`.`age` != 18 AND `users`.`gender` NOT IN ('male', 'female') AND `users`.`name` != 'Tom' AND `users`.`profile` IS NOT NULL");

        let r#where = Where::<User>::new(json!("age > 18"), Ops::new(JoinType::And, false, false));
        assert_eq!(r#where.to_sql_string().unwrap(), "age > 18");

        let r#where = Where::<User>::new(json!(["age > 18"]), Ops::new(JoinType::And, false, false));
        assert_eq!(r#where.to_sql_string().unwrap(), "age > 18");
        let r#where = Where::<User>::new(json!(["name = ? AND age > ? AND gender in ? AND enable = ?", "Tom", 18, ["male", "female"], true]), Ops::new(JoinType::And, false, false));
        assert_eq!(r#where.to_sql_string().unwrap(), "name = 'Tom' AND age > 18 AND gender in ('male', 'female') AND enable = 1");
        assert_eq!(r#where.to_sql().unwrap().value, "name = ? AND age > ? AND gender in ? AND enable = ?");

        //between
        let r#where = Where::<User>::new(json!({"age": [18, 30]}), Ops::new(JoinType::And, false, true));
        assert_eq!(r#where.to_sql_string().unwrap(), "`users`.`age` BETWEEN 18 AND 30");
        let r#where = Where::<User>::new(json!({"age": [18, 30]}), Ops::new(JoinType::And, true, true));
        assert_eq!(r#where.to_sql_string().unwrap(), "`users`.`age` NOT BETWEEN 18 AND 30");
        let r#where = Where::<User>::new(json!(["age", 19, 31]), Ops::new(JoinType::And, false, true));
        assert_eq!(r#where.to_sql_string().unwrap(), "`users`.`'age'` BETWEEN 19 AND 31");
        let r#where = Where::<User>::new(json!(["age", 18, 30]), Ops::new(JoinType::And, true, true));
        assert_eq!(r#where.to_sql_string().unwrap(), "`users`.`'age'` NOT BETWEEN 18 AND 30");
        // or between
        let r#where = Where::<User>::new(json!({"age": [18, 30], "name": "Tom"}), Ops::new(JoinType::Or, false, false));
        assert_eq!(r#where.to_sql_string().unwrap(), "(`users`.`age` IN (18, 30) OR `users`.`name` = 'Tom')");
        let r#where = Where::<User>::new(json!({"age": [18, 30], "name": "Tom"}), Ops::new(JoinType::Or, true, false));
        assert_eq!(r#where.to_sql_string().unwrap(), "(`users`.`age` NOT IN (18, 30) OR `users`.`name` != 'Tom')");
        let r#where = Where::<User>::new(json!({"age": [18, 30], "name": "Tom"}), Ops::new(JoinType::Or, true, true));
        assert_eq!(r#where.to_sql_string().unwrap(), "(`users`.`age` NOT BETWEEN 18 AND 30 OR `users`.`name` != 'Tom')");
        let r#where = Where::<User>::new(json!(["age", 18, 30]), Ops::new(JoinType::Or, true, true));
        assert_eq!(r#where.to_sql_string().unwrap(), "(`users`.`'age'` NOT BETWEEN 18 AND 30)");
    }
}

