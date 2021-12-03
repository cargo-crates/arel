pub mod select_manager;
pub mod insert_manager;
pub mod update_manager;
pub mod delete_manager;
pub use select_manager::SelectManager;
pub use select_manager::select_statement::SelectStatement;
pub use insert_manager::InsertManager;
pub use update_manager::UpdateManager;
pub use delete_manager::DeleteManager;

use serde_json::{Value as Json, json};
use crate::methods::type_to_pluralize_string;
use crate::traits::ModelAble;
use std::marker::PhantomData;
use crate::collectors::SqlString;
use crate::visitors;
use crate::statements::{r#where, having};
// pub trait ManagerStatement<M: ModelAble> {}

#[derive(Clone, Debug)]
pub struct Table<M: ModelAble> {
    pub select_manager: Option<SelectManager<M>>,
    pub insert_manager: Option<InsertManager<M>>,
    pub update_manager: Option<UpdateManager<M>>,
    pub delete_manager: Option<DeleteManager<M>>,
    _marker: PhantomData<M>,
}

impl<M> Table<M> where M: ModelAble {
    /// Get Model's table name.
    ///
    /// # Examples
    ///
    /// ```
    /// use arel::traits::ModelAble;
    /// use arel::table::Table;
    ///
    /// #[derive(Clone, Debug)]
    /// struct User {}
    /// impl ModelAble for User {}
    /// assert_eq!(User::table_name(), "users");
    /// ```
    pub fn table_name() -> String {
        type_to_pluralize_string::<M>()
    }
    pub fn new() -> Self {
        Self {
            select_manager: None,
            insert_manager: None,
            update_manager: None,
            delete_manager: None,
            _marker: PhantomData
        }
    }
    fn _where_statement(&mut self, condition: Json, ops: r#where::Ops) -> &mut Self {
        if let Some(select_manager) = &mut self.select_manager {
            select_manager.r#where(condition, ops);
        } else if let Some(update_manager) = &mut self.update_manager {
            update_manager.r#where(condition, ops);
        } else if let Some(delete_manager) = &mut self.delete_manager {
            delete_manager.r#where(condition, ops);
        } else {
            panic!("Not support");
        }
        self
    }
    pub fn r#where(&mut self, condition: Json) -> &mut Self {
        self._where_statement(condition, r#where::Ops::new(r#where::JoinType::And, false, false))
    }
    pub fn where_not(&mut self, condition: Json) -> &mut Self {
        self._where_statement(condition, r#where::Ops::new(r#where::JoinType::And, true, false))
    }
    pub fn where_between(&mut self, condition: Json) -> &mut Self {
        self._where_statement(condition, r#where::Ops::new(r#where::JoinType::And, false, true))
    }
    pub fn where_not_between(&mut self, condition: Json) -> &mut Self {
        self._where_statement(condition, r#where::Ops::new(r#where::JoinType::And, true, true))
    }
    pub fn where_or(&mut self, condition: Json) -> &mut Self {
        self._where_statement(condition, r#where::Ops::new(r#where::JoinType::Or, false, false))
    }
    pub fn where_or_not(&mut self, condition: Json) -> &mut Self {
        self._where_statement(condition, r#where::Ops::new(r#where::JoinType::Or, true, false))
    }
    pub fn where_or_between(&mut self, condition: Json) -> &mut Self {
        self._where_statement(condition, r#where::Ops::new(r#where::JoinType::Or, false, true))
    }
    pub fn where_or_not_between(&mut self, condition: Json) -> &mut Self {
        self._where_statement(condition, r#where::Ops::new(r#where::JoinType::Or, true, true))
    }
    fn _where_range_statement<T: ToString>(&mut self, column_name: &str, range: impl std::ops::RangeBounds<T>, ops: r#where::Ops) -> &mut Self {
        if let Some(select_manager) = &mut self.select_manager {
            select_manager.where_range(column_name, range, ops);
        } else if let Some(update_manager) = &mut self.update_manager {
            update_manager.where_range(column_name, range, ops);
        } else if let Some(delete_manager) = &mut self.delete_manager {
            delete_manager.where_range(column_name, range, ops);
        } else {
            panic!("Not support");
        }
        self
    }
    pub fn where_range<T: ToString>(&mut self, column_name: &str, range: impl std::ops::RangeBounds<T>) -> &mut Self {
        self._where_range_statement(column_name, range, r#where::Ops::new(r#where::JoinType::And, false, false));
        self
    }
    pub fn where_range_between<T: ToString>(&mut self, column_name: &str, range: impl std::ops::RangeBounds<T>) -> &mut Self {
        self._where_range_statement(column_name, range, r#where::Ops::new(r#where::JoinType::And, false, true));
        self
    }
    pub fn with_select_manager(&mut self) -> &mut Self {
        if self.select_manager.is_none() {
            self.select_manager = Some(SelectManager::<M>::default());
        }
        self
    }
    pub fn select(&mut self, condition: Json) -> &mut Self {
        if let Some(select_manager) = &mut self.select_manager {
            select_manager.select(condition);
        } else {
            panic!("Not support");
        }
        self
    }
    pub fn count(&mut self) -> &mut Self {
        if let Some(select_manager) = &mut self.select_manager {
            select_manager.count();
        } else {
            panic!("Not support");
        }
        self
    }
    pub fn sum(&mut self, column_name: &str) -> &mut Self {
        if let Some(select_manager) = &mut self.select_manager {
            select_manager.sum(column_name);
        } else {
            panic!("Not support");
        }
        self
    }
    pub fn avg(&mut self, column_name: &str) -> &mut Self {
        if let Some(select_manager) = &mut self.select_manager {
            select_manager.avg(column_name);
        } else {
            panic!("Not support");
        }
        self
    }
    pub fn min(&mut self, column_name: &str) -> &mut Self {
        if let Some(select_manager) = &mut self.select_manager {
            select_manager.min(column_name);
        } else {
            panic!("Not support");
        }
        self
    }
    pub fn max(&mut self, column_name: &str) -> &mut Self {
        if let Some(select_manager) = &mut self.select_manager {
            select_manager.max(column_name);
        } else {
            panic!("Not support");
        }
        self
    }
    pub fn distinct(&mut self) -> &mut Self {
        if let Some(select_manager) = &mut self.select_manager {
            select_manager.distinct();
        } else {
            panic!("Not support");
        }
        self
    }
    pub fn lock(&mut self) -> &mut Self {
        if let Some(select_manager) = &mut self.select_manager {
            select_manager.lock(json!("FOR UPDATE"));
        } else {
            panic!("Not support");
        }
        self
    }
    pub fn joins(&mut self, condition: Json) -> &mut Self {
        if let Some(select_manager) = &mut self.select_manager {
            select_manager.joins(condition);
        } else {
            panic!("Not support");
        }
        self
    }
    pub fn group(&mut self, condition: Json) -> &mut Self {
        if let Some(select_manager) = &mut self.select_manager {
            select_manager.group(condition);
        } else {
            panic!("Not support");
        }
        self
    }
    pub fn _having_statement(&mut self, condition: Json, ops: having::Ops) -> &mut Self {
        if let Some(select_manager) = &mut self.select_manager {
            select_manager.having(condition, ops);
        } else {
            panic!("Not support");
        }
        self
    }
    pub fn having(&mut self, condition: Json) -> &mut Self {
        self._having_statement(condition, having::Ops::new(r#where::JoinType::And, false, false))
    }
    pub fn having_not(&mut self, condition: Json) -> &mut Self {
        self._having_statement(condition, having::Ops::new(r#where::JoinType::And, true, false))
    }
    pub fn having_between(&mut self, condition: Json) -> &mut Self {
        self._having_statement(condition, having::Ops::new(r#where::JoinType::And, false, true))
    }
    pub fn having_not_between(&mut self, condition: Json) -> &mut Self {
        self._having_statement(condition, having::Ops::new(r#where::JoinType::And, true, true))
    }
    pub fn having_or(&mut self, condition: Json) -> &mut Self {
        self._having_statement(condition, having::Ops::new(r#where::JoinType::Or, false, false))
    }
    pub fn having_or_not(&mut self, condition: Json) -> &mut Self {
        self._having_statement(condition, having::Ops::new(r#where::JoinType::Or, true, false))
    }
    pub fn having_or_between(&mut self, condition: Json) -> &mut Self {
        self._having_statement(condition, having::Ops::new(r#where::JoinType::Or, false, false))
    }
    pub fn having_or_not_between(&mut self, condition: Json) -> &mut Self {
        self._having_statement(condition, having::Ops::new(r#where::JoinType::Or, true, true))
    }
    pub fn order(&mut self, condition: Json) -> &mut Self {
        if let Some(select_manager) = &mut self.select_manager {
            select_manager.order(condition);
        } else if let Some(delete_manager) = &mut self.delete_manager {
            delete_manager.order(condition);
        } else {
            panic!("Not support");
        }
        self
    }
    pub fn limit(&mut self, condition: usize) -> &mut Self {
        if let Some(select_manager) = &mut self.select_manager {
            select_manager.limit(condition);
        } else if let Some(delete_manager) = &mut self.delete_manager {
            delete_manager.limit(condition);
        } else {
            panic!("Not support");
        }
        self
    }
    pub fn offset(&mut self, condition: usize) -> &mut Self {
        if let Some(select_manager) = &mut self.select_manager {
            select_manager.offset(condition);
        } else if let Some(delete_manager) = &mut self.delete_manager {
            delete_manager.offset(condition);
        } else {
            panic!("Not support");
        }
        self
    }
    pub fn paginate(&mut self, page: usize, page_size: usize) -> &mut Self {
        let offset = (page - 1) * page_size;
        self.limit(page_size);
        self.offset(offset);
        self
    }
    pub fn with_update_manager(&mut self) -> &mut Self {
        if self.update_manager.is_none() {
            self.update_manager = Some(UpdateManager::<M>::default());
            // if let Some(select_manager) = &mut self.select_manager {
            //     if let Some(update_manager) = &mut self.update_manager {
            //         update_manager.ctx_mut().wheres.append(&mut select_manager.ctx_mut().wheres);
            //         self.select_manager = None;
            //     }
            // }
        }
        self
    }
    pub fn update_all(&mut self, condition: Json) -> &mut Self {
        self.with_update_manager();
        if let Some(update_manager) = &mut self.update_manager {
            update_manager.update(condition);
        } else {
            panic!("Not support");
        }
        self
    }
    pub fn with_insert_manager(&mut self) -> &mut Self {
        if self.insert_manager.is_none() {
            self.insert_manager = Some(InsertManager::<M>::default());
        }
        self
    }
    pub fn create(&mut self, condition: Json) -> &mut Self {
        self.with_insert_manager();
        if let Some(insert_manager) = &mut self.insert_manager {
            insert_manager.insert(condition);
        } else {
            panic!("Not support");
        }
        self
    }
    pub fn with_delete_manager(&mut self) -> &mut Self {
        if self.delete_manager.is_none() {
            self.delete_manager = Some(DeleteManager::<M>::default());
        }
        self
    }
    pub fn delete_all(&mut self, condition: Json) -> &mut Self {
        self.with_delete_manager();
        self.r#where(condition);
        self
    }
    pub fn to_sql(&mut self) -> String {
        let mut collector = SqlString::default();
        if let Some(insert_manager) = &self.insert_manager {
            visitors::accept_insert_manager(insert_manager, &mut collector);
        } else if let Some(update_manager) = &self.update_manager {
            let mut for_update_select_manager = None;
            if let Some(select_manager) = &mut self.select_manager {
                select_manager.select(json!([M::primary_key()]));
                for_update_select_manager = Some(select_manager);
            }
            visitors::accept_update_manager(update_manager, for_update_select_manager, &mut collector);
        } else if let Some(delete_manager) = &self.delete_manager {
            let mut for_update_select_manager = None;
            if let Some(select_manager) = &mut self.select_manager {
                select_manager.select(json!([M::primary_key()]));
                for_update_select_manager = Some(select_manager);
            }
            visitors::accept_delete_manager(delete_manager, for_update_select_manager, &mut collector);
        } else if let Some(select_manager) = &self.select_manager {
            visitors::accept_select_manager(select_manager, &mut collector);
        }  else {
            panic!("Not support");
        }
        collector.value
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn it_works() {
//         #[derive(Clone, Debug)]
//         struct User {}
//         impl ModelAble for User {}
//         impl User {
//             fn new() -> Self {
//                 Self {}
//             }
//         }
//         assert_eq!(User::table_name(), "users");
//
//         let table = Table::new(User::new());
//         assert_eq!(table.table_name(), "users");
//     }
// }