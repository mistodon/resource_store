use std::collections::hash_map::{HashMap, DefaultHasher};
use std::hash::{Hash, Hasher};


pub struct Store<T>
{
    map: HashMap<u64, T>
}

impl<T> Store<T>
{
    pub fn new() -> Self
    {
        Store { map: HashMap::new() }
    }

    fn hash_key(name: &'static str) -> u64
    {
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        hasher.finish()
    }

    pub fn insert(&mut self, name: &'static str, item: T) -> StoreKey
    {
        let index = Self::hash_key(name);
        self.map.insert(index, item);
        StoreKey { index: index }
    }

    pub fn get(&self, key: StoreKey) -> Option<&T>
    {
        self.map.get(&key.index)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct StoreKey
{
    index: u64
}