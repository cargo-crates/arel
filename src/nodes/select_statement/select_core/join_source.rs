use crate::nodes::statements::StatementsType;
use crate::traits::ModelAble;
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct JoinSource<M: ModelAble> {
    left: Option<StatementsType<M>>,
    right: Option<StatementsType<M>>,
    _marker: PhantomData<M>,
}

impl<M> JoinSource<M> where M: ModelAble {
    pub fn new() -> Self {
        Self {
            left: None,
            right: None,
            _marker: PhantomData,
        }
    }
}