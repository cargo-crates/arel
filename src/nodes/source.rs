use crate::traits::ModelAble;
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct Source<T: ModelAble> {
    _marker: PhantomData<T>
}

impl<T> Source<T> where T: ModelAble {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}