mod join_source;
pub use join_source::JoinSource;

use serde_json::{Value as Json};
use crate::statements::{Where, helpers::and};
use std::default::Default;
use crate::traits::ModelAble;
use std::marker::PhantomData;
use crate::nodes::{SqlLiteral};

#[derive(Clone, Debug)]
pub struct SelectCore<M: ModelAble> {
    join_source: Option<JoinSource<M>>,
    // set_quantifier: Option<_>,
    // optimizer_hints: Option<_>,
    // projections: Vec<StatementsType<M>>,
    pub wheres: Vec<Where<M>>,
    // groups: Vec<StatementsType<M>>,
    // havings: Vec<StatementsType<M>>,
    // windows: Vec<StatementsType<M>>,
    // comment: None,
    _marker: PhantomData<M>,
}

impl<M> Default for SelectCore<M> where M: ModelAble {
    fn default() -> Self {
        Self {
            join_source: None,
            // projections: vec![],
            wheres: vec![],
            // groups: vec![],
            // havings: vec![],
            // windows: vec![],
            _marker: PhantomData,
        }
    }
}

impl<M> SelectCore<M> where M: ModelAble {
    pub fn joins(&mut self, condition: Json) -> &mut Self {
        self.join_source = Some(JoinSource::<M>::new(condition));
        self
    }
    pub fn get_joins_sql(&self) -> Option<SqlLiteral> {
        if let Some(join_source) = &self.join_source {
            Some(SqlLiteral::new(join_source.to_sql()))
        } else {
            None
        }
    }
    pub fn r#where(&mut self, condition: Json) -> &mut Self {
        self.wheres.push(Where::<M>::new(condition, false));
        self
    }
    pub fn get_where_sql(&self) -> Option<SqlLiteral> {
        if self.r#wheres.len() == 0 {
            None
        } else {
            Some(SqlLiteral::new(format!("{}", and::to_sql(&self.r#wheres))))
        }
    }
}



