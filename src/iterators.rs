use std::hash::Hash;

use ordermap::{OrderMap, OrderSet, map::IntoValues, set::IntoIter};

use crate::ListImd;

pub enum SetOrIter<K> {
    Set(OrderSet<K>),
    Iter(IntoIter<K>),
}

impl<K: Eq + Hash> SetOrIter<K> {
    pub fn remove(&mut self, key: &K) -> Option<K> {
        match self {
            SetOrIter::Set(order_map) => order_map.take(key),
            SetOrIter::Iter(_) => None,
        }
    }

    pub fn next(&mut self) -> Option<K> {
        match self {
            SetOrIter::Set(order_map) => {
                let mut iter = std::mem::take(order_map).into_iter();
                let next = iter.next();
                *self = SetOrIter::Iter(iter);
                next
            }
            SetOrIter::Iter(iter) => iter.next(),
        }
    }
}

pub enum MapOrIter<K, A> {
    Map(OrderMap<K, A>),
    Iter(IntoValues<K, A>),
}

impl<K: Eq + Hash, A> MapOrIter<K, A> {
    pub fn remove(&mut self, key: &K) -> Option<A> {
        match self {
            MapOrIter::Map(order_map) => order_map.remove(key),
            MapOrIter::Iter(_) => None,
        }
    }
    pub fn next(&mut self) -> Option<A> {
        match self {
            MapOrIter::Map(order_map) => {
                let mut iter = std::mem::take(order_map).into_values();
                let next = iter.next();
                *self = MapOrIter::Iter(iter);
                next
            }
            MapOrIter::Iter(iter) => iter.next(),
        }
    }
}
pub struct IterImd<I, K> {
    pub prev: SetOrIter<K>,
    pub next: I,
}

impl<I: Iterator<Item = K>, K: Eq + Hash> Iterator for IterImd<I, K> {
    type Item = ListImd<K, K>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next.next() {
            Some(item) => {
                if let Some(k) = self.prev.remove(&item) {
                    Some(ListImd::Modify(k, item))
                } else {
                    Some(ListImd::Insert(item))
                }
            }
            None => self.prev.next().map(ListImd::Delete),
        }
    }
}

pub struct IterImdMapped<I, A, F, K> {
    pub prev: MapOrIter<K, A>,
    pub next: I,
    pub f: F,
}

impl<I: Iterator, A, F: FnMut(&I::Item) -> K, K: Eq + Hash> Iterator for IterImdMapped<I, A, F, K> {
    type Item = ListImd<A, I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next.next() {
            Some(item) => {
                if let Some(prev) = self.prev.remove(&(self.f)(&item)) {
                    Some(ListImd::Modify(prev, item))
                } else {
                    Some(ListImd::Insert(item))
                }
            }
            None => self.prev.next().map(ListImd::Delete),
        }
    }
}
