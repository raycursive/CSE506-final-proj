use std::fmt::{Debug, Display};

pub trait KeyType = Debug + Clone + Display + Ord + Send + Sync;

pub trait ValueType = Debug + Display + Clone + PartialEq + Send + Sync;

pub enum GetType {
    GetRef,
    GetVal,
}

#[allow(unused_variables)]
pub trait Tree<K: KeyType, V: ValueType>: Send + Sync {
    const GET_TYPE: GetType = GetType::GetRef;

    fn put(&self, key: K, value: V);
    fn get(&self, key: K) -> Option<&V> {
        panic!("not implemented")
    }
    fn get_val(&self, key: K) -> Option<V> {
        panic!("not implemented")
    }
    fn remove(&self, key: K) {
        panic!("not implemented")
    }
    fn new() -> Self;
}