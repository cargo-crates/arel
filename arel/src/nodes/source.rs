use crate::traits::ArelAble;
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct Source<T: ArelAble> {
    _marker: PhantomData<T>
}

impl<T> Source<T> where T: ArelAble {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}