use std::default::Default;

#[derive(Clone, Debug)]
pub struct SqlString {
    pub value: String,
}

impl Default for SqlString {
    fn default() -> Self {
        Self {
            value: "".to_string(),
        }
    }
}