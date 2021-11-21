use serde_json::{Value as Json, json};
use crate::nodes::{ SqlLiteral, And, select_statement::{SelectStatement, SelectCore} };
use std::default::Default;

#[derive(Debug, Clone)]
pub struct SelectManager {
    pub ast: SelectStatement,
}

impl Default for SelectManager {
    fn default() -> Self {
        Self {
            ast: SelectStatement::default(),
        }
    }
}

impl SelectManager {
    fn get_ctx_mut(&mut self) -> &mut SelectCore {
        self.ast.cores.last_mut().unwrap()
    }
    fn get_ctx(&self) -> &SelectCore {
        self.ast.cores.last().unwrap()
    }
    pub fn r#where(&mut self, condition: Json) -> &mut Self {
        self.get_ctx_mut().r#where(condition);
        self
    }
    pub fn get_where_sql(&self) -> Option<SqlLiteral> {
        let ctx = self.get_ctx();
        if self.get_ctx().r#wheres.len() == 0 {
            None
        } else {
            Some(SqlLiteral::new(&format!("WHERE {}", And::new(&ctx.r#wheres).to_sql())))
        }
    }
}