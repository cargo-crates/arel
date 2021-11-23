use serde_json::{Value as Json, json};
use crate::nodes::statements::StatementAble;

use crate::nodes::SqlLiteral;
use crate::traits::ModelAble;
use std::marker::PhantomData;

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
    fn to_sql(&self) -> Vec<SqlLiteral> {
        let mut vec = vec![];
        if let Json::Object(json_object) = self.value() {
            for column_name in json_object.keys() {
                let full_column_name = Self::full_column_name(column_name);
                if let Some(json_value) = json_object.get(column_name) {
                    match json_value {
                        Json::Array(json_array) => {
                            let mut values = vec![];
                            for json_value in json_array.iter() {
                                match json_value {
                                    Json::String(json_string) => {
                                        values.push(format!("'{}'", json_string));
                                    },
                                    Json::Number(json_number) => {
                                        values.push(format!("{}", json_number));
                                    },
                                    _ => ()
                                }
                            }
                            if values.len() > 0 {
                                if self.is_where_not {
                                    vec.push(SqlLiteral::new(format!("{} NOT IN ({})", full_column_name, values.join(", "))));
                                } else {
                                    vec.push(SqlLiteral::new(format!("{} IN ({})", full_column_name, values.join(", "))));
                                }
                            }
                        },
                        Json::String(json_string) => {
                            if self.is_where_not {
                                vec.push(SqlLiteral::new(format!("{} != '{}'", full_column_name, json_string)));
                            } else {
                                vec.push(SqlLiteral::new(format!("{} = '{}'", full_column_name, json_string)));
                            }
                        },
                        Json::Number(json_number) => {
                            if self.is_where_not {
                                vec.push(SqlLiteral::new(format!("{} != {}", full_column_name, json_number)));
                            } else {
                                vec.push(SqlLiteral::new(format!("{} = {}", full_column_name, json_number)));
                            }
                        },
                        Json::Bool(json_bool) => {
                            let value = if *json_bool {1} else {0};
                            if self.is_where_not {
                                vec.push(SqlLiteral::new(format!("{} != {}", full_column_name, value)));
                            } else {
                                vec.push(SqlLiteral::new(format!("{} = {}", full_column_name, value)));
                            }
                        },
                        Json::Null => {
                            if self.is_where_not {
                                vec.push(SqlLiteral::new(format!("{} IS NOT NULL", full_column_name)));
                            } else {
                                vec.push(SqlLiteral::new(format!("{} IS NULL", full_column_name)));
                            }
                        },
                        _ => ()
                    }
                }
            }
        }
        // Ok(vec.join(" AND "))
        vec
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
}
