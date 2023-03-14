use std::collections::HashMap;
use std::hash::Hash;

pub struct MapStack<K: Eq + Hash, V>(Vec<HashMap<K, V>>);

impl<K: Eq + Hash, V> MapStack<K, V> {
    pub fn new() -> MapStack<K, V> {
        MapStack(Vec::new())
    }

    pub fn push(&mut self) {
        self.0.push(HashMap::new());
    }

    pub fn pop(&mut self) -> Option<HashMap<K, V>> {
        self.0.pop()
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.0.last_mut()?.insert(key, value)
    }
}
