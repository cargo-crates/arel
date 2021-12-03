// use crate::collectors::{SqlString};
use crate::traits::ModelAble;
use crate::statements::{StatementAble, helpers};

pub fn to_sql<M: ModelAble, S: StatementAble<M>>(children: &Vec<S>) -> String {
    helpers::inject_join(children, " AND ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json};
    use crate::statements::{r#where::{self, Where}};
    #[test]
    fn to_sql() {
        #[derive(Clone, Debug)]
        struct User {}
        impl ModelAble for User {}
        let wheres = vec![
            Where::<User>::new(json!({"profile": null}), r#where::Ops::new(r#where::JoinType::And, false, false)),
            Where::<User>::new(json!(["name = ?", "Tom"]), r#where::Ops::new(r#where::JoinType::And, false, false)),
        ];
        assert_eq!(super::to_sql(&wheres), "`users`.`profile` IS NULL AND (name = 'Tom')");
    }
}