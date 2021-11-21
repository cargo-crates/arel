mod join_source;
pub use join_source::JoinSource;

use serde_json::{Value as Json, json};
use crate::nodes::statements::{StatementsType, Where};
use std::default::Default;

#[derive(Clone, Debug)]
pub struct SelectCore {
    source: JoinSource,
    // set_quantifier: Option<_>,
    // optimizer_hints: Option<_>,
    projections: Vec<StatementsType>,
    pub wheres: Vec<StatementsType>,
    groups: Vec<StatementsType>,
    havings: Vec<StatementsType>,
    windows: Vec<StatementsType>,
    // comment: None,
}

impl Default for SelectCore {
    fn default() -> Self {
        Self {
            source: JoinSource::new(),
            projections: vec![],
            wheres: vec![],
            groups: vec![],
            havings: vec![],
            windows: vec![],
        }
    }
}

impl SelectCore {
    pub fn r#where(&mut self, condition: Json) -> &mut Self {
        self.wheres.push(StatementsType::Where(Where::new(condition)));
        self
    }
}



