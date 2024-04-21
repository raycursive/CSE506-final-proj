use std::fmt::{Debug, Display};

pub trait KeyType = Debug + Clone + Display + Ord;
pub trait ValueType = Debug + Display + Clone + PartialEq;

#[allow(unused_variables)]
pub trait Tree<K: KeyType, V: ValueType> {
    fn put(&mut self, key: K, value: V);
    fn get(&self, key: K) -> Option<&V> {
        panic!("not implemented")
    }
    fn get_val(&self, key: K) -> Option<V> {
        panic!("not implemented")
    }
    fn remove(&mut self, key: K) {
        panic!("not implemented")
    }
    fn new() -> Self;
}
