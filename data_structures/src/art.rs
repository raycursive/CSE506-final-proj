use congee::Art;

use crate::binary_search_tree::TreeParams;
use crate::interfaces::{GetType, KeyType, Tree, ValueType};


impl<K: KeyType, V: ValueType> Tree<K, V> for Art<K, V> where K: Clone + From<usize>,
                                                              V: Clone + From<usize>,
                                                              usize: From<V> + From<K> {
    const GET_TYPE: crate::interfaces::GetType = GetType::GetVal;

    fn put(&self, key: K, value: V) {
        self.insert(key, value, &self.pin()).expect("Failed");
    }

    fn get_val(&self, key: K) -> Option<V> {
        self.get(&key, &self.pin())
    }

    fn new() -> Self {
        Art::default()
    }
}


pub type DefaultArt = Art<usize, usize>;