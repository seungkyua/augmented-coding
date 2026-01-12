use std::collections::BTreeMap;

#[derive(Debug)]
struct LeafNode<K, V> {
    branching_factor: usize,
    entries: BTreeMap<K, V>,
}

impl<K: Ord, V> LeafNode<K, V> {
    fn new(branching_factor: usize) -> Self {
        Self {
            branching_factor,
            entries: BTreeMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct BPlusTree<K, V> {
    root: LeafNode<K, V>,
    branching_factor: usize,
}

impl<K: Ord, V> BPlusTree<K, V> {
    pub fn new(branching_factor: usize) -> Self {
        Self {
            branching_factor,
            root: LeafNode::new(branching_factor),
        }
    }

    pub fn len(&self) -> usize {
        self.root.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.root.entries.is_empty()
    }

    pub fn branching_factor(&self) -> usize {
        self.root.branching_factor
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.root.entries.insert(key, value)
    }

    pub fn get(&self, _key: &K) -> Option<&V> {
        unimplemented!()
    }

    pub fn remove(&mut self, _key: &K) -> Option<V> {
        unimplemented!()
    }
}
