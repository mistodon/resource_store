use std::marker::PhantomData;
use std::ops::Index;


#[derive(Debug)]
pub struct StoreKey<T>
{
    index: usize,
    _phantom: PhantomData<T>
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


pub trait ResourceStore<T>
{
    fn add(&mut self, item: T) -> StoreKey<T>;
    fn get(&self, key: StoreKey<T>) -> &T;
}

pub struct Store<T>
{
    items: Vec<T>
}

impl<T> Store<T>
{
    pub fn new() -> Self
    {
        Store
        {
            items: Vec::new()
        }
    }
}


impl<T> ResourceStore<T> for Store<T>
{
    fn add(&mut self, item: T) -> StoreKey<T>
    {
        let index = self.items.len();
        self.items.push(item);
        StoreKey { index: index, _phantom: PhantomData }
    }

    fn get(&self, key: StoreKey<T>) -> &T
    {
        self.items.index(key.index)
    }
}


#[cfg(test)]
mod tests
{
    use super::*;


    #[test]
    fn can_insert_items_into_store()
    {
        let mut store = Store::new();
        let _: StoreKey<usize> = store.add(5);
    }

    #[test]
    fn can_fetch_items_from_store()
    {
        let mut store = Store::new();
        let key = store.add(10);
        assert_eq!(*store.get(key), 10);
    }

    fn sendy_sync<T>(_: T)
        where T: Send + Sync
    {

    }

    #[test]
    fn store_keys_can_be_send_between_threads()
    {
        let mut store = Store::new();
        let key = store.add(15);
        sendy_sync(key);
    }
}