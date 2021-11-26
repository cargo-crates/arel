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

impl SqlString {
    pub fn push(&mut self, char: char) -> &mut Self {
        self.value.push(char);
        self
    }
    pub fn push_str(&mut self, sub_str: &str) -> &mut Self {
        self.value.push_str(sub_str);
        self
    }
}