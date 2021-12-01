mod join_source;
pub use join_source::JoinSource;

use serde_json::{Value as Json};
use crate::statements::{StatementAble, Select, Where, Group, Having, helpers::{self, and}};
use std::default::Default;
use crate::traits::ModelAble;
use std::marker::PhantomData;
use crate::nodes::{SqlLiteral};

#[derive(Clone, Debug)]
pub struct SelectCore<M: ModelAble> {
    join_source: Option<JoinSource<M>>,
    select: Select<M>,
    // set_quantifier: Option<_>,
    // optimizer_hints: Option<_>,
    // projections: Vec<StatementsType<M>>,
    pub wheres: Vec<Where<M>>,
    pub groups: Vec<Group<M>>,
    pub havings: Vec<Having<M>>,
    // windows: Vec<StatementsType<M>>,
    // comment: None,
    _marker: PhantomData<M>,
}

impl<M> Default for SelectCore<M> where M: ModelAble {
    fn default() -> Self {
        Self {
            join_source: None,
            select: Select::<M>::default(),
            // projections: vec![],
            wheres: vec![],
            groups: vec![],
            havings: vec![],
            // windows: vec![],
            _marker: PhantomData,
        }
    }
}

impl<M> SelectCore<M> where M: ModelAble {
    pub fn select(&mut self, condition: Json) -> &mut Self {
        self.select.value = condition;
        self
    }
    pub fn distinct(&mut self) -> &mut Self {
        self.select.distinct = true;
        self
    }
    pub fn get_select_sql(&self) -> SqlLiteral {
        let mut sql = "SELECT ".to_string();
        if self.select.distinct {
            sql.push_str("DISTINCT ");
        }
        sql.push_str(&self.select.to_sql());
        SqlLiteral::new(sql)
    }
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
    pub fn r#where(&mut self, condition: Json, is_not: bool) -> &mut Self {
        self.wheres.push(Where::<M>::new(condition, is_not));
        self
    }
    pub fn get_where_sql(&self) -> Option<SqlLiteral> {
        if self.r#wheres.len() == 0 {
            None
        } else {
            Some(SqlLiteral::new(and::to_sql(&self.r#wheres)))
        }
    }
    pub fn group(&mut self, condition: Json) -> &mut Self {
        self.groups.push(Group::<M>::new(condition));
        self
    }
    pub fn get_group_sql(&self) -> Option<SqlLiteral> {
        if self.groups.len() == 0 {
            None
        } else {
            Some(SqlLiteral::new(helpers::inject_join(&self.groups, ", ")))
        }
    }
    pub fn having(&mut self, condition: Json, is_not: bool) -> &mut Self {
        self.havings.push(Having::<M>::new(condition, is_not));
        self
    }
    pub fn get_having_sql(&self) -> Option<SqlLiteral> {
        if self.havings.len() == 0 {
            None
        } else {
            Some(SqlLiteral::new(and::to_sql(&self.havings)))
        }
    }
}



