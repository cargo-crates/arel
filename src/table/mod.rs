pub mod select_manager;
pub mod select_statement;
pub use select_manager::SelectManager;
pub use select_statement::SelectStatement;

use serde_json::{Value as Json, json};
use crate::methods::table_name;
use crate::traits::ModelAble;
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct Table<M: ModelAble> {
    pub select_manager: SelectManager<M>,
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
        table_name::<M>()
    }
    pub fn new() -> Self {
        Self {
            select_manager: SelectManager::<M>::default(),
            _marker: PhantomData
        }
    }
    pub fn r#where(&mut self, condition: Json) -> &mut Self {
        self.select_manager.r#where(condition);
        self
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