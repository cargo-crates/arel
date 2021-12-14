// use crate::collectors::{SqlString};
use crate::traits::ArelAble;
use crate::statements::{StatementAble, helpers};

pub fn to_sql<M: ArelAble, S: StatementAble<M>>(children: &Vec<S>) -> anyhow::Result<String> {
    helpers::inject_join(children, " AND ")
}

#[cfg(test)]
mod tests {
    use crate as arel;
    use arel::prelude::*;
    use crate::statements::{r#where::{self, Where}};
    #[test]
    fn to_sql() {
        #[arel::arel]
        #[allow(dead_code)]
        struct User {
            id: i64,
        }

        let wheres = vec![
            Where::<User>::new(json!({"profile": null}), r#where::Ops::new(r#where::JoinType::And, false, false)),
            Where::<User>::new(json!(["name = ?", "Tom"]), r#where::Ops::new(r#where::JoinType::And, false, false)),
        ];
        assert_eq!(super::to_sql(&wheres).unwrap(), "`users`.`profile` IS NULL AND name = 'Tom'");
    }
}