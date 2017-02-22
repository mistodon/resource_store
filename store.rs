use std::collections::hash_map::{HashMap, DefaultHasher};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::sync::Arc;

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

    pub fn insert(&mut self, name: &'static str, item: T) -> StoreKey<T>
    {
        let index = Self::hash_key(name);
        self.map.insert(index, item);
        StoreKey { index: index, _phantom: PhantomData }
    }

    pub fn get(&self, key: StoreKey<T>) -> Option<&T>
    {
        self.map.get(&key.index)
    }
}

#[derive(Debug)]
pub struct StoreKey<T>
{
    index: u64,
    _phantom: PhantomData<Arc<T>>
}

impl<T> Clone for StoreKey<T>
{
    fn clone(&self) -> Self
    {
        StoreKey { index: self.index, _phantom: PhantomData }
    }
}

impl<T> Copy for StoreKey<T> {}
unsafe impl<T> Send for StoreKey<T> {}
unsafe impl<T> Sync for StoreKey<T> {}