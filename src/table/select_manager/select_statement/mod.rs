pub mod select_core;
pub use select_core::SelectCore;

use std::default::Default;
use crate::traits::ModelAble;
use std::marker::PhantomData;
// use crate::table::ManagerStatement;

#[derive(Clone, Debug)]
pub struct SelectStatement<M: ModelAble> {
    pub cores: Vec<SelectCore<M>>,
    // orders: Vec<StatementsType<M>>,
    // limit: Option<StatementsType<M>>,
    // lock: Option<StatementsType<M>>,
    // offset: Option<StatementsType<M>>,
    // with: Option<StatementsType<M>>,
    _marker: PhantomData<M>,
}

// impl<M> ManagerStatement<M> for SelectStatement<M> where M: ModelAble {}

impl<M> Default for SelectStatement<M> where M: ModelAble {
    fn default() -> Self {
        Self {
            cores: vec![SelectCore::<M>::default()],
            // orders: vec![],
            // limit: None,
            // lock: None,
            // offset: None,
            // with: None,
            _marker: PhantomData,
        }
    }
}

impl<M> SelectStatement<M> where M: ModelAble {

}
