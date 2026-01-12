use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct BPlusTree<K, V> {
    map: BTreeMap<K, V>,
}

impl<K: Ord, V> BPlusTree<K, V>  {

    pub fn new() -> Self {
        Self { map: BTreeMap::new() }
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        unimplemented!()
    }

    pub fn insert(&mut self, _key: K, _value: V) -> Option<V> {
        unimplemented!()
    }

    pub fn get(&self, _key: &K) -> Option<&V> {
        unimplemented!()
    }

    pub fn remove(&mut self, _key: &K) -> Option<V> {
        unimplemented!()
    }
}
