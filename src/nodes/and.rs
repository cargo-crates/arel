use crate::nodes::statements::{StatementsType, Where};

#[derive(Clone, Debug)]
pub struct And<'a> {
    children: &'a Vec<StatementsType>
}

impl<'a> And<'a> {
    pub fn new(children: &'a Vec<StatementsType>) -> Self {
        Self {
            children
        }
    }
    pub fn to_sql(&self) -> String {
        "".to_string()
    }
}