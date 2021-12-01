use crate::collectors::{SqlString};
use crate::traits::ModelAble;
use crate::statements::{StatementAble, helpers};

pub fn to_sql<M: ModelAble, S: StatementAble<M>>(children: &Vec<S>) -> String {
    helpers::inject_join(children, " AND ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json};
    use crate::statements::Where;
    #[test]
    fn to_sql() {
        #[derive(Clone, Debug)]
        struct User {}
        impl ModelAble for User {}
        let wheres = vec![
            Where::<User>::new(json!({"profile": null}), false),
            Where::<User>::new(json!(["name = ?", "Tom"]), false),
        ];
        assert_eq!(super::to_sql(&wheres), "`users`.`profile` IS NULL AND (name = 'Tom')");
    }
}