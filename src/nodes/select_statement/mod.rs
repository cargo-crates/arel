pub mod select_core;
pub use select_core::SelectCore;

use serde_json::{Value as Json, json};
use crate::nodes::statements::StatementsType;
use std::default::Default;

#[derive(Clone, Debug)]
pub struct SelectStatement {
    pub cores: Vec<SelectCore>,
    orders: Vec<StatementsType>,
    limit: Option<StatementsType>,
    lock: Option<StatementsType>,
    offset: Option<StatementsType>,
    with: Option<StatementsType>
}

impl Default for SelectStatement {
    fn default() -> Self {
        Self {
            cores: vec![SelectCore::default()],
            orders: vec![],
            limit: None,
            lock: None,
            offset: None,
            with: None,
        }
    }
}

impl SelectStatement {

}
