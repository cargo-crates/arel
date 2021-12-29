// use crate::collectors::{Sql};
use crate::traits::ArelAble;
use crate::statements::{StatementAble, helpers};
use crate::collectors::Sql;

pub fn to_sql<M: ArelAble, S: StatementAble<M>>(children: &Vec<S>) -> anyhow::Result<Sql> {
    helpers::inject_join(children, " AND ")
}

pub fn to_sql_string<M: ArelAble, S: StatementAble<M>>(children: &Vec<S>) -> anyhow::Result<String> {
    to_sql(children)?.to_sql_string()
}

#[cfg(test)]
#[cfg(feature = "mysql")]
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
            Where::<User>::new(json!({"profile": null}), r#where::Ops::new(r#where::JoinType::And, false, false, false)),
            Where::<User>::new(json!(["name = ?", "Tom"]), r#where::Ops::new(r#where::JoinType::And, false, false, false)),
        ];
        assert_eq!(super::to_sql_string(&wheres).unwrap(), "`users`.`profile` IS NULL AND name = 'Tom'");
    }
}