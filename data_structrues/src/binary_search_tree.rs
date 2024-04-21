use std::{
    alloc::dealloc,
    alloc::{alloc_zeroed, Layout},
    cmp,
    fmt::{Debug, Display},
    marker::PhantomData,
    ptr::null_mut,
    sync::atomic::{self, AtomicPtr},
};

use crate::{
    fix_sized_key::{FixSizedKey, FixSizedKeyParams},
    quick_istr::QuickIStr,
    traits::{KeyType, Tree, ValueType},
};

pub trait TreeParams: FixSizedKeyParams {
    type ValueType: Debug + Display + PartialEq + for<'a> From<QuickIStr<'a>>;
    type IKeyType: Debug
        + Clone
        + Display
        + Ord
        + Into<String>
        + From<String>
        + for<'a> From<&'a str>
        + From<&'static str>
        + Into<String>
        + for<'a> From<QuickIStr<'a>>;
}

pub trait UnsafeGet<T> {
    fn get(&self) -> *mut T;
}

impl<T> UnsafeGet<T> for AtomicPtr<T> {
    fn get(&self) -> *mut T {
        unsafe {
            return *self.as_ptr();
        }
    }
}

#[derive(Debug)]
pub struct Node<T: TreeParams> {
    _phantom: PhantomData<T>,
    pub key: T::IKeyType,
    pub p_value: AtomicPtr<T::ValueType>,
    pub p_left: AtomicPtr<Node<T>>,
    pub p_right: AtomicPtr<Node<T>>,
}

impl<T: TreeParams> Node<T> {
    pub fn new_ptr(key: T::IKeyType) -> *mut Self {
        let layout = Layout::from_size_align(std::mem::size_of::<Node<T>>(), 64).unwrap();
        unsafe {
            let ptr = alloc_zeroed(layout) as *mut Node<T>;
            (*ptr).key = key;
            return ptr;
        }
    }
}

pub struct Cursor<T: TreeParams> {
    _phantom: PhantomData<T>,
    root: *const Node<T>,
    pub parent: *mut Node<T>,
    pub node: *mut Node<T>,
    pub key: T::IKeyType,
    pub p_value: *mut T::ValueType,
}

impl<T: TreeParams> Cursor<T> {
    pub fn new(key: T::IKeyType, root: *mut Node<T>) -> Self {
        Cursor {
            parent: null_mut(),
            node: null_mut(),
            key,
            p_value: null_mut(),
            root: root as *const Node<T>,
            _phantom: PhantomData,
        }
    }

    #[inline(always)]
    pub fn find(&mut self) -> bool {
        return self.find_from(self.root as *mut Node<T>);
    }

    pub fn find_from(&mut self, node: *mut Node<T>) -> bool {
        self.node = node;
        while !self.node.is_null() {
            unsafe {
                match (*self.node).key.cmp(&self.key) {
                    cmp::Ordering::Equal => {
                        self.p_value = (*self.node).p_value.get();
                        return true;
                    }
                    cmp::Ordering::Less => {
                        self.parent = self.node;
                        self.node = (*self.node).p_left.get();
                    }
                    cmp::Ordering::Greater => {
                        self.parent = self.node;
                        self.node = (*self.node).p_right.get();
                    }
                }
            }
        }
        return false;
    }
}

/// Lock-free Binary Search Tree, CAS-based, fixed key size
#[derive(Debug)]
pub struct LockFreeBinarySearchTree<T: TreeParams> {
    _phantom: PhantomData<T>,
    pub root: AtomicPtr<Node<T>>,
}

impl<T: TreeParams<ValueType = V>, K, V> Tree<K, V> for LockFreeBinarySearchTree<T>
where
    K: Into<T::IKeyType> + KeyType,
    V: ValueType,
{
    fn new() -> Self {
        LockFreeBinarySearchTree {
            root: AtomicPtr::from(null_mut()),
            _phantom: PhantomData,
        }
    }

    fn put(&mut self, key: K, value: V) {
        let new_p_value = Box::into_raw(Box::new(value.into()));
        let mut new_p_node: *mut Node<T> = null_mut();
        let key = key.into();

        loop {
            let rootptr = self.root.get();
            if rootptr.is_null() {
                new_p_node = Node::<T>::new_ptr(key.clone());
                unsafe {
                    (*new_p_node).p_value = AtomicPtr::new(new_p_value);
                }
                if let Ok(_) = self.root.compare_exchange(
                    null_mut(),
                    new_p_node,
                    atomic::Ordering::Release,
                    atomic::Ordering::Relaxed,
                ) {
                    return;
                }
            } else {
                break;
            }
        }

        let mut cursor = Cursor::<T>::new(key, self.root.get());
        loop {
            if cursor.find_from(if cursor.parent.is_null() {
                self.root.get()
            } else {
                cursor.parent
            }) {
                // perform update
                unsafe {
                    loop {
                        if let Ok(_) = (*cursor.node).p_value.compare_exchange(
                            cursor.p_value,
                            new_p_value,
                            atomic::Ordering::Release,
                            atomic::Ordering::Relaxed,
                        ) {
                            break;
                        }
                        cursor.p_value = (*cursor.node).p_value.get();
                    }
                    dealloc(cursor.p_value as *mut u8, Layout::new::<T::ValueType>());
                    if !new_p_node.is_null() {
                        dealloc(
                            new_p_node as *mut u8,
                            Layout::from_size_align(std::mem::size_of::<Node<T>>(), 64).unwrap(),
                        );
                    }
                    return;
                }
            } else {
                // perform insert
                if new_p_node.is_null() {
                    new_p_node = Node::<T>::new_ptr(cursor.key.clone());
                    unsafe {
                        (*new_p_node).p_value = AtomicPtr::new(new_p_value);
                    }
                }
                unsafe {
                    let original = if (*cursor.parent).key < cursor.key {
                        &(*cursor.parent).p_left
                    } else {
                        &(*cursor.parent).p_right
                    };
                    if let Ok(_) = original.compare_exchange(
                        null_mut(),
                        new_p_node,
                        atomic::Ordering::Release,
                        atomic::Ordering::Relaxed,
                    ) {
                        cursor.node = new_p_node;
                        return;
                    }
                }
            }
        }
    }

    fn get(&self, key: K) -> Option<&V> {
        let root = self.root.get();
        if root.is_null() {
            return None;
        }
        let mut cursor = Cursor::<T>::new(key.into(), root);
        if cursor.find() && !cursor.p_value.is_null() {
            return Some(unsafe { &*cursor.p_value });
        }
        return None;
    }

    fn remove(&mut self, _key: K) {
        todo!();
    }
}

impl<T: TreeParams> LockFreeBinarySearchTree<T> {
    fn print(f: &mut std::fmt::Formatter<'_>, prefix: String, node: *mut Node<T>, is_left: bool) {
        if !node.is_null() {
            write!(f, "{}", prefix).unwrap();
            write!(f, "{}", if is_left { "├──" } else { "└──" }).unwrap();
            unsafe {
                write!(f, "{}:{}\n", (*node).key, (*(*node).p_value.get())).unwrap();
            }
            LockFreeBinarySearchTree::print(
                f,
                prefix.clone() + if is_left { "│   " } else { "    " },
                unsafe { (*node).p_left.get() },
                true,
            );
            LockFreeBinarySearchTree::print(
                f,
                prefix.clone() + if is_left { "│   " } else { "    " },
                unsafe { (*node).p_right.get() },
                false,
            );
        }
    }
}

impl<T: TreeParams> Display for LockFreeBinarySearchTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        LockFreeBinarySearchTree::print(f, "".to_string(), self.root.get(), false);
        Ok(())
    }
}

pub struct DefaultParams {}
impl TreeParams for DefaultParams {
    type ValueType = String;
    type IKeyType = FixSizedKey<DefaultParams>;
}

impl FixSizedKeyParams for DefaultParams {
    const KEY_SIZE: usize = 16;
    const ALLOW_INT_CMP: bool = true;
}
pub type LockFreeBST = LockFreeBinarySearchTree<DefaultParams>;
