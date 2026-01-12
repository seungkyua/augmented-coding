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

    fn len(&self) -> usize {
        self.entries.len()
    }

    fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.entries.insert(key, value)
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.entries.get(key)
    }
}

#[derive(Debug)]
pub struct BPlusTree<K, V> {
    root: LeafNode<K, V>,
}

impl<K: Ord, V> BPlusTree<K, V> {
    pub fn new(branching_factor: usize) -> Self {
        Self {
            root: LeafNode::new(branching_factor),
        }
    }

    pub fn len(&self) -> usize {
        self.root.len()
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_empty()
    }

    pub fn branching_factor(&self) -> usize {
        self.root.branching_factor
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.root.insert(key, value)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.root.get(key)
    }

    pub fn remove(&mut self, _key: &K) -> Option<V> {
        unimplemented!()
    }
}
