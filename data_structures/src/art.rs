extern crate congee;

use congee::Art;
use crate::binary_search_tree::{LockFreeBinarySearchTree, TreeParams};
use crate::interfaces::{KeyType, ValueType, Tree, GetType};


pub struct ArtTreeWrapper<K: KeyType, V: ValueType>
    where K: Clone + From<usize>,
          V: Clone + From<usize>,
          usize: From<V> + From<K> {
    tree: Art<K, V>,
}

impl<K: KeyType, V: ValueType> Tree<K, V> for ArtTreeWrapper<K, V>
    where K: Clone + From<usize>,
          V: Clone + From<usize>,
          usize: From<V> + From<K> {
    const GET_TYPE: crate::interfaces::GetType = GetType::GetVal;

    fn new() -> Self {
        Self {
            tree: Art::default()
        }
    }

    fn put(&self, key: K, value: V) {
        self.tree.insert(key, value, &self.tree.pin()).expect("Failed");
    }

    fn get_val(&self, key: K) -> Option<V> {
        self.tree.get(&key, &self.tree.pin())
    }
}

pub type BasicArt = ArtTreeWrapper<usize, usize>;