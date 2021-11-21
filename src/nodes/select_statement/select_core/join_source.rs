use crate::nodes::statements::StatementsType;

#[derive(Clone, Debug)]
pub struct JoinSource {
    left: Option<StatementsType>,
    right: Option<StatementsType>,
}

impl JoinSource {
    pub fn new() -> Self {
        Self {
            left: None,
            right: None,
        }
    }
}