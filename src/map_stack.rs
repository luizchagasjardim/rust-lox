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

    pub fn get_in_top(&self, key: &K) -> Option<&V> {
        self.0.last()?.get(key)
    }

    pub fn get_in_any(&self, key: &K) -> Option<(usize, &V)> {
        self.0
            .iter()
            .rev()
            .enumerate()
            .find_map(|(depth, hash_map)| Some((depth, hash_map.get(key)?)))
    }

    pub fn any_contains(&self, key: &K) -> Option<usize> {
        let (depth, _) = self.get_in_any(key)?;
        Some(depth)
    }
}
