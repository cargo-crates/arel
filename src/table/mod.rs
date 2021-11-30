pub mod select_manager;
pub mod update_manager;
pub use select_manager::SelectManager;
pub use select_manager::select_statement::SelectStatement;
pub use update_manager::UpdateManager;

use serde_json::{Value as Json, json};
use crate::methods::type_to_pluralize_string;
use crate::traits::ModelAble;
use std::marker::PhantomData;
use crate::collectors::SqlString;
use crate::visitors;
// pub trait ManagerStatement<M: ModelAble> {}

#[derive(Clone, Debug)]
pub struct Table<M: ModelAble> {
    pub select_manager: Option<SelectManager<M>>,
    pub update_manager: Option<UpdateManager<M>>,
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
            update_manager: None,
            _marker: PhantomData
        }
    }
    pub fn with_select_manager(&mut self) -> &mut Self {
        self.select_manager = Some(SelectManager::<M>::default());
        self
    }
    pub fn with_update_manager(&mut self) -> &mut Self {
        self.update_manager = Some(UpdateManager::<M>::default());
        if let Some(select_manager) = &mut self.select_manager {
            if let Some(update_manager) = &mut self.update_manager {
                update_manager.ctx_mut().wheres.append(&mut select_manager.ctx_mut().wheres);
                self.select_manager = None;
            }
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
    pub fn r#where(&mut self, condition: Json) -> &mut Self {
        if let Some(select_manager) = &mut self.select_manager {
            select_manager.r#where(condition);
        } else if let Some(update_manager) = &mut self.update_manager {
            update_manager.r#where(condition);
        } else {
            panic!("Not support");
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
    pub fn to_sql(&self) -> String {
        let mut collector = SqlString::default();
        if let Some(select_manager) = &self.select_manager {
            visitors::accept_select_manager(select_manager, &mut collector);
        } else if let Some(update_manager) = &self.update_manager {
            visitors::accept_update_manager(update_manager, &mut collector);
        } else {
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