use crossbeam_skiplist::SkipMap;

use crate::interfaces::{KeyType, Tree, ValueType};

pub struct SkipMapWrapper<K: KeyType, V: ValueType>(SkipMap<K, V>);
unsafe impl<K: KeyType, V: ValueType> Send for SkipMapWrapper<K, V> {}
unsafe impl<K: KeyType, V: ValueType> Sync for SkipMapWrapper<K, V> {}

impl<K: KeyType + 'static, V: ValueType + 'static> Tree<K, V> for SkipMapWrapper<K, V> {
    fn new() -> Self {
        SkipMapWrapper(SkipMap::new())
    }

    fn put(&self, key: K, value: V) {
        self.0.insert(key, value);
    }

    fn get_val(&self, key: K) -> Option<V> {
        return self.0.get(&key).map(|v| v.value().clone());
    }

    fn remove(&self, key: K) {
        self.0.remove(&key);
    }
}
