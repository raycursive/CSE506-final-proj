use crate::interfaces::{GetType, KeyType, Tree, ValueType};
use cds::avltree::RwLockAVLTree;
use cds::map::ConcurrentMap;

#[allow(invalid_reference_casting)]
impl<K: KeyType + Default, V: ValueType + Default> Tree<K, V> for RwLockAVLTree<K, V> {
    const GET_TYPE: crate::interfaces::GetType = GetType::GetVal;

    fn put(&self, key: K, value: V) {
        let mut_self = unsafe { &mut *(self as *const Self as *mut Self) };
        mut_self.insert(&key, value).expect("Insert failed");
    }

    fn get_val(&self, key: K) -> Option<V> {
        ConcurrentMap::get(self, &key)
    }

    fn new() -> Self {
        <RwLockAVLTree<K, V> as ConcurrentMap<K, V>>::new()
    }
}

pub type ConcurrentAVLTree<K, V> = RwLockAVLTree<K, V>;
