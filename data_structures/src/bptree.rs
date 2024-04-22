use bplustree::BPlusTree;


use crate::binary_search_tree::TreeParams;
use crate::interfaces::{GetType, KeyType, Tree, ValueType};

impl<K: KeyType, V: ValueType> Tree<K, V> for BPlusTree<K, V> {
    const GET_TYPE: crate::interfaces::GetType = GetType::GetVal;

    fn put(&self, key: K, value: V) {
        self.insert(key, value);
    }

    fn get_val(&self, key: K) -> Option<V> {
        self.lookup(&key, |v| v.clone())
    }

    fn new() -> Self {
        BPlusTree::new()
    }
}

pub type BpTree<K, V> = BPlusTree<K, V>;