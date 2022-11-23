//! # Arbitrary Ext
//!
//! Since `1.2.0` Arbitrary supports custom arbitrary implementation for fields on derive.
//!
//! But it still remains tricky, if a type that does not implement `Arbitrary` is wrapped into a generic type.
//!
//! This crate provides s set of function combinators to support the containers and collections form the standard library.
//!
//! See an example below.
//!
//!
//! ## Usage
//!
//! ```rust
//! use arbitrary::{Unstructured, Arbitrary};
//! use arbitrary_ext::{arbitrary_option, arbitrary_vec, arbitrary_hash_map};
//! use std::collections::HashMap;
//!
//! // Imagine this is a foreign type, that by some reason does not implement Arbitrary trait.
//! #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
//! struct Number(u32);
//!
//! // Our custom function to generate arbitrary Number out of Unstructured.
//! fn arbitrary_number(u: &mut Unstructured) -> arbitrary::Result<Number> {
//!     let value = u.int_in_range(0..=1000)?;
//!     Ok(Number(value))
//! }
//!
//! #[derive(Debug, Arbitrary)]
//! struct Example {
//!     #[arbitrary(with = arbitrary_number)]
//!     number: Number,
//!
//!     #[arbitrary(with = arbitrary_option(arbitrary_number))]
//!     option: Option<Number>,
//!
//!     #[arbitrary(with = arbitrary_vec(arbitrary_number))]
//!     vec: Vec<Number>,
//!
//!     #[arbitrary(
//!         with = arbitrary_hash_map(
//!             arbitrary_number,
//!             arbitrary_vec(arbitrary_option(arbitrary_number))
//!         )
//!     )]
//!     hash_map: HashMap<Number, Vec<Option<Number>>>,
//! }
//! ```


use std::{
    collections::{BTreeSet, HashSet, HashMap, BTreeMap, VecDeque, LinkedList, BinaryHeap},
    hash::Hash,
};
use arbitrary::Unstructured;

pub fn arbitrary_option<T>(
    arbitrary_inner: impl Fn(&mut Unstructured<'_>) -> arbitrary::Result<T> + Copy,
) -> impl Fn(&mut Unstructured<'_>) -> arbitrary::Result<Option<T>> + Copy {
    move |u: &mut Unstructured<'_>| {
        // Generate 1 None out of 5 runs on average.
        match u.ratio(1, 5)? {
            true => Ok(None),
            false => {
                let val = arbitrary_inner(u)?;
                Ok(Some(val))
            }
        }
    }
}

pub fn arbitrary_btree_set<T: Ord>(
    arbitrary_inner: impl Fn(&mut Unstructured<'_>) -> arbitrary::Result<T> + Copy,
) -> impl Fn(&mut Unstructured<'_>) -> arbitrary::Result<BTreeSet<T>> + Copy {
    move |u: &mut Unstructured<'_>| {
        let len = arbitrary_len(u)?;
        let mut set = BTreeSet::new();
        for _ in 0..len {
            let val = arbitrary_inner(u)?;
            set.insert(val);
        }
        Ok(set)
    }
}

pub fn arbitrary_hash_set<T: Hash + Eq>(
    arbitrary_inner: impl Fn(&mut Unstructured<'_>) -> arbitrary::Result<T> + Copy,
) -> impl Fn(&mut Unstructured<'_>) -> arbitrary::Result<HashSet<T>> + Copy {
    move |u: &mut Unstructured<'_>| {
        let len = arbitrary_len(u)?;
        let mut set = HashSet::with_capacity(len);
        for _ in 0..len {
            let val = arbitrary_inner(u)?;
            set.insert(val);
        }
        Ok(set)
    }
}

pub fn arbitrary_vec<T>(
    arbitrary_inner: impl Fn(&mut Unstructured<'_>) -> arbitrary::Result<T> + Copy,
) -> impl Fn(&mut Unstructured<'_>) -> arbitrary::Result<Vec<T>> + Copy {
    move |u: &mut Unstructured<'_>| {
        let len = arbitrary_len(u)?;
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            let val = arbitrary_inner(u)?;
            vec.push(val);
        }
        Ok(vec)
    }
}

pub fn arbitrary_vec_deque<T>(
    arbitrary_inner: impl Fn(&mut Unstructured<'_>) -> arbitrary::Result<T> + Copy,
) -> impl Fn(&mut Unstructured<'_>) -> arbitrary::Result<VecDeque<T>> + Copy {
    move |u: &mut Unstructured<'_>| {
        let len = arbitrary_len(u)?;
        let mut vec = VecDeque::with_capacity(len);
        for _ in 0..len {
            let val = arbitrary_inner(u)?;
            vec.push_back(val);
        }
        Ok(vec)
    }
}

pub fn arbitrary_linked_list<T>(
    arbitrary_inner: impl Fn(&mut Unstructured<'_>) -> arbitrary::Result<T> + Copy,
) -> impl Fn(&mut Unstructured<'_>) -> arbitrary::Result<LinkedList<T>> + Copy {
    move |u: &mut Unstructured<'_>| {
        let len = arbitrary_len(u)?;
        let mut list = LinkedList::new();
        for _ in 0..len {
            let val = arbitrary_inner(u)?;
            list.push_back(val);
        }
        Ok(list)
    }
}


pub fn arbitrary_hash_map<K: Eq + Hash, V>(
    arbitrary_key: impl Fn(&mut Unstructured<'_>) -> arbitrary::Result<K> + Copy,
    arbitrary_value: impl Fn(&mut Unstructured<'_>) -> arbitrary::Result<V> + Copy,
) -> impl Fn(&mut Unstructured<'_>) -> arbitrary::Result<HashMap<K, V>> + Copy {
    move |u: &mut Unstructured<'_>| {
        let len = arbitrary_len(u)?;
        let mut hashmap = HashMap::with_capacity(len);
        for _ in 0..len {
            let key = arbitrary_key(u)?;
            let value = arbitrary_value(u)?;
            hashmap.insert(key, value);
        }
        Ok(hashmap)
    }
}

pub fn arbitrary_btree_map<K: Ord, V>(
    arbitrary_key: impl Fn(&mut Unstructured<'_>) -> arbitrary::Result<K> + Copy,
    arbitrary_value: impl Fn(&mut Unstructured<'_>) -> arbitrary::Result<V> + Copy,
) -> impl Fn(&mut Unstructured<'_>) -> arbitrary::Result<BTreeMap<K, V>> + Copy {
    move |u: &mut Unstructured<'_>| {
        let len = arbitrary_len(u)?;
        let mut btreemap = BTreeMap::new();
        for _ in 0..len {
            let key = arbitrary_key(u)?;
            let value = arbitrary_value(u)?;
            btreemap.insert(key, value);
        }
        Ok(btreemap)
    }
}

pub fn arbitrary_binary_heap<T: Ord>(
    arbitrary_inner: impl Fn(&mut Unstructured<'_>) -> arbitrary::Result<T> + Copy,
) -> impl Fn(&mut Unstructured<'_>) -> arbitrary::Result<BinaryHeap<T>> + Copy {
    move |u: &mut Unstructured<'_>| {
        let len = arbitrary_len(u)?;
        let mut heap = BinaryHeap::new();
        for _ in 0..len {
            let val = arbitrary_inner(u)?;
            heap.push(val);
        }
        Ok(heap)
    }
}


// P(len = 0..=5) = 90%
// P(len = 6..=20) = 5%
// P(len = 21..=50) = 4%
// P(len = 51..=100) = 0.9%
// P(len = 101..=1000) = 0.1%
fn arbitrary_len(u: &mut Unstructured) -> arbitrary::Result<usize> {
    let n = u.int_in_range(0..=1000)?;
    match n {
        0..=900 => u.int_in_range(0..=5),
        901..=950 => u.int_in_range(6..=20),
        951..=990 => u.int_in_range(21..=50),
        991..=999 => u.int_in_range(51..=100),
        1000 => u.int_in_range(101..=1000),
        _ => unreachable!(),
    }
}
