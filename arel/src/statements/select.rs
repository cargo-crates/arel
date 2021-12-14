use serde_json::{Value as Json, json};
use std::marker::PhantomData;
use std::default::Default;
use crate::traits::ArelAble;
use crate::statements::StatementAble;
use crate::nodes::SqlLiteral;
use crate::methods;

#[derive(Clone, Debug)]
pub enum Op {
    Count,
    Sum(String),
    Avg(String),
    Min(String),
    Max(String),
}

#[derive(Clone, Debug)]
pub struct Select<M: ArelAble> {
    pub value: Json,
    pub distinct: bool,
    pub op: Option<Op>,
    _marker: PhantomData<M>,
}

impl<M> StatementAble<M> for Select<M> where M: ArelAble {
    fn json_value(&self) -> Option<&Json> {
        Some(&self.value)
    }
    fn to_sql_literals(&self) -> anyhow::Result<Vec<SqlLiteral>> {
        let mut vec = vec![];
        if let Some(json_value) = self.json_value() {
            match json_value {
                Json::Array(json_array) => {
                    for column_name in json_array.iter() {
                        if let Json::String(column_name) = column_name {
                            let table_column_name = methods::table_column_name::<M>(column_name);
                            vec.push(SqlLiteral::new(format!("{}", table_column_name)));
                        } else {
                            return Err(anyhow::anyhow!("Error: {:?} Not Support", self.json_value()))
                        }
                    }
                },
                Json::String(_) =>  vec.append(&mut StatementAble::to_sql_literals_default(self)?),
                _ => return Err(anyhow::anyhow!("Error: {:?} Not Support", self.json_value()))
            }
        }
        // Ok(vec.join(" AND "))
        Ok(vec)
    }
    fn to_sql(&self) -> anyhow::Result<String> {
        let mut sql = self.to_sql_with_concat(", ")?;
        if self.distinct {
            sql = format!("DISTINCT {}", &sql);
        }
        if let Some(op) = &self.op {
            match op {
                Op::Count => {
                    sql = format!("COUNT({})", &sql);
                },
                Op::Sum(column_name) => {
                    let select = Select::<M>::new(json!([column_name]), self.distinct);
                    sql = format!("SUM({})", &select.to_sql()?);
                },
                Op::Avg(column_name) => {
                    let select = Select::<M>::new(json!([column_name]), self.distinct);
                    sql = format!("AVG({})", &select.to_sql()?);
                },
                Op::Min(column_name) => {
                    let select = Select::<M>::new(json!([column_name]), self.distinct);
                    sql = format!("MIN({})", &select.to_sql()?);
                },
                Op::Max(column_name) => {
                    let select = Select::<M>::new(json!([column_name]), self.distinct);
                    sql = format!("MAX({})", &select.to_sql()?);
                }
            }
        }
        Ok(sql)
    }
}

impl<M> Default for Select<M> where M: ArelAble {
    fn default() -> Self {
        Self {
            value: json!(["*"]),
            distinct: false,
            op: None,
            _marker: PhantomData
        }
    }
}

impl<M> Select<M> where M: ArelAble {
    pub fn new(value: Json, distinct: bool) -> Self {
        Self {
            value,
            distinct,
            op: None,
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
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

        let select = Select::<User>::new(json!("name, age"), false);
        assert_eq!(select.to_sql().unwrap(), "name, age");

        let select = Select::<User>::new(json!(["name", "age"]), false);
        assert_eq!(select.to_sql().unwrap(), "`users`.`name`, `users`.`age`");
    }
}
