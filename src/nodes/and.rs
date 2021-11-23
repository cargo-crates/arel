use crate::nodes::NodeAble;
use crate::nodes::statements::{StatementsType, Where};
use crate::collectors::{SqlString};
use crate::traits::ModelAble;
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct And<'a, M: ModelAble> {
    children: &'a Vec<StatementsType<M>>,
    _marker: PhantomData<M>,
}

impl<'a, M> NodeAble<M> for And<'a, M> where M: ModelAble {}

impl<'a, M> And<'a, M> where M: ModelAble {
    pub fn new(children: &'a Vec<StatementsType<M>>) -> Self {
        Self {
            children,
            _marker: PhantomData,
        }
    }

    pub fn to_sql(&self) -> String {
        let mut collector = SqlString::new();
        Self::inject_join(self.children,&mut collector, " AND ");
        collector.value
    }
}