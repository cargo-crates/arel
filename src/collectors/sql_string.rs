#[derive(Clone, Debug)]
pub struct SqlString {
    bind_index: usize,
    value: String,
}

impl SqlString {
    pub fn new() -> SqlString {
        SqlString {
            bind_index: 1,
            value: "".to_string(),
        }
    }
}