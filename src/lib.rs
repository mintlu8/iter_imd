#![doc = include_str!("../README.md")]

use std::hash::Hash;
mod iterators;
use iterators::{IterImd, IterImdMapped, MapOrIter, SetOrIter};

/// Insertion, modification and deletion in an ordered container.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListImd<A, B> {
    Insert(B),
    Modify(A, B),
    Delete(A),
}

/// Extension for iterating two iterators of unique keys as insertions, modifications and removals.
pub trait IterImdExt: IntoIterator {
    /// Express difference between two iterators as insertions, modifications and removals.
    fn iter_list_imd(
        self,
        next: impl IntoIterator<Item = Self::Item>,
    ) -> impl Iterator<Item = ListImd<Self::Item, Self::Item>>
    where
        Self::Item: Eq + Hash;

    /// Express difference between two iterators as insertions, modifications and removals.
    fn iter_list_imd_mapped<Key: Eq + Hash, Item>(
        self,
        next: impl IntoIterator<Item = Item>,
        map_self: impl FnMut(&Self::Item) -> Key,
        map_other: impl FnMut(&Item) -> Key,
    ) -> impl Iterator<Item = ListImd<Self::Item, Item>>;

    /// Express difference between two iterators as insertions, modifications, moves and removals.
    fn iter_list_imd_indexed(
        self,
        next: impl IntoIterator<Item = Self::Item>,
    ) -> impl Iterator<Item = ListImd<(usize, Self::Item), (usize, Self::Item)>>
    where
        Self::Item: Eq + Hash + Clone;

    /// Express difference between two iterators as insertions, modifications, moves and removals.
    fn iter_list_imd_indexed_mapped<Key: Eq + Hash, Item>(
        self,
        next: impl IntoIterator<Item = Item>,
        map_self: impl FnMut(&Self::Item) -> Key,
        map_other: impl FnMut(&Item) -> Key,
    ) -> impl Iterator<Item = ListImd<(usize, Self::Item), (usize, Item)>>;
}

impl<T> IterImdExt for T
where
    T: IntoIterator,
{
    fn iter_list_imd(
        self,
        next: impl IntoIterator<Item = Self::Item>,
    ) -> impl Iterator<Item = ListImd<Self::Item, Self::Item>>
    where
        Self::Item: Eq + Hash,
    {
        IterImd {
            prev: SetOrIter::Set(self.into_iter().collect()),
            next: next.into_iter(),
        }
    }

    fn iter_list_imd_mapped<Key: Eq + Hash, Item>(
        self,
        next: impl IntoIterator<Item = Item>,
        mut map_self: impl FnMut(&Self::Item) -> Key,
        map_other: impl FnMut(&Item) -> Key,
    ) -> impl Iterator<Item = ListImd<Self::Item, Item>> {
        IterImdMapped {
            prev: MapOrIter::Map(self.into_iter().map(|x| (map_self(&x), x)).collect()),
            next: next.into_iter(),
            f: map_other,
        }
    }

    fn iter_list_imd_indexed(
        self,
        next: impl IntoIterator<Item = Self::Item>,
    ) -> impl Iterator<Item = ListImd<(usize, Self::Item), (usize, Self::Item)>>
    where
        Self::Item: Eq + Hash + Clone,
    {
        IterImdMapped {
            prev: MapOrIter::Map(
                self.into_iter()
                    .enumerate()
                    .map(|(i, v)| (v.clone(), (i, v)))
                    .collect(),
            ),
            next: next.into_iter().enumerate(),
            f: |(_, v): &(usize, Self::Item)| v.clone(),
        }
    }

    fn iter_list_imd_indexed_mapped<Key: Eq + Hash, Item>(
        self,
        next: impl IntoIterator<Item = Item>,
        mut map_self: impl FnMut(&Self::Item) -> Key,
        mut map_other: impl FnMut(&Item) -> Key,
    ) -> impl Iterator<Item = ListImd<(usize, Self::Item), (usize, Item)>> {
        IterImdMapped {
            prev: MapOrIter::Map(
                self.into_iter()
                    .enumerate()
                    .map(|(i, v)| (map_self(&v), (i, v)))
                    .collect(),
            ),
            next: next.into_iter().enumerate(),
            f: move |(_, v): &(usize, Item)| map_other(v),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::IterImdExt;

    #[test]
    pub fn test() {
        let first = [1, 2, 3, 4, 5];
        let second = [1, 2, 4, 5, 6];

        let mut joined = first.iter_list_imd(second);

        assert_eq!(joined.next(), Some(crate::ListImd::Modify(1, 1)));
        assert_eq!(joined.next(), Some(crate::ListImd::Modify(2, 2)));
        assert_eq!(joined.next(), Some(crate::ListImd::Modify(4, 4)));
        assert_eq!(joined.next(), Some(crate::ListImd::Modify(5, 5)));
        assert_eq!(joined.next(), Some(crate::ListImd::Insert(6)));
        assert_eq!(joined.next(), Some(crate::ListImd::Delete(3)));

        let first = [1, 2, 4];
        let second = [1, 3, 4];

        let mut joined = first.iter_list_imd(second);

        assert_eq!(joined.next(), Some(crate::ListImd::Modify(1, 1)));
        assert_eq!(joined.next(), Some(crate::ListImd::Insert(3)));
        assert_eq!(joined.next(), Some(crate::ListImd::Modify(4, 4)));
        assert_eq!(joined.next(), Some(crate::ListImd::Delete(2)));

        let first = ['a', 'b', 'c'];
        let second = ['a', 'c', 'd'];

        let mut joined = first.iter_list_imd_indexed(second);

        assert_eq!(
            joined.next(),
            Some(crate::ListImd::Modify((0, 'a'), (0, 'a')))
        );
        assert_eq!(
            joined.next(),
            Some(crate::ListImd::Modify((2, 'c'), (1, 'c')))
        );
        assert_eq!(joined.next(), Some(crate::ListImd::Insert((2, 'd'))));
        assert_eq!(joined.next(), Some(crate::ListImd::Delete((1, 'b'))));
    }
}
