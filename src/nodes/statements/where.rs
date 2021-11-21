use serde_json::{Value as Json, json};

#[derive(Clone, Debug)]
pub struct Where {
    value: Json,
}

impl Where {
    pub fn new(value: Json) -> Where {
        Where {
            value
        }
    }
}
