#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

use std::cmp::Ordering;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy)]
pub struct BestMapNonEmpty<K, V> {
    key: K,
    value: V,
}

impl<K: PartialOrd, V> BestMapNonEmpty<K, V> {
    pub fn insert_gt(&mut self, key: K, value: V) {
        if key > self.key {
            self.insert(key, value);
        }
    }

    pub fn insert_ge(&mut self, key: K, value: V) {
        if key >= self.key {
            self.insert(key, value);
        }
    }

    pub fn insert_lt(&mut self, key: K, value: V) {
        if key < self.key {
            self.insert(key, value);
        }
    }

    pub fn insert_le(&mut self, key: K, value: V) {
        if key <= self.key {
            self.insert(key, value);
        }
    }
}

impl<K, V> BestMapNonEmpty<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self {
            key: key,
            value: value,
        }
    }

    fn insert(&mut self, key: K, value: V) {
        self.key = key;
        self.value = value;
    }

    pub fn get(&self) -> (&K, &V) {
        (&self.key, &self.value)
    }
    pub fn key(&self) -> &K {
        &self.key
    }
    pub fn value(&self) -> &V {
        &self.value
    }

    pub fn into_key_and_value(self) -> (K, V) {
        (self.key, self.value)
    }
    pub fn into_key(self) -> K {
        self.key
    }
    pub fn into_value(self) -> V {
        self.value
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy)]
pub struct BestMap<K, V> {
    non_empty: Option<BestMapNonEmpty<K, V>>,
}
impl<K, V> Default for BestMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: PartialOrd, V> BestMap<K, V> {
    pub fn insert_gt(&mut self, key: K, value: V) {
        if let Some(non_empty) = self.non_empty.as_mut() {
            non_empty.insert_gt(key, value);
            return;
        }
        self.non_empty = Some(BestMapNonEmpty::new(key, value));
    }

    pub fn insert_ge(&mut self, key: K, value: V) {
        if let Some(non_empty) = self.non_empty.as_mut() {
            non_empty.insert_ge(key, value);
            return;
        }
        self.non_empty = Some(BestMapNonEmpty::new(key, value));
    }

    pub fn insert_lt(&mut self, key: K, value: V) {
        if let Some(non_empty) = self.non_empty.as_mut() {
            non_empty.insert_lt(key, value);
            return;
        }
        self.non_empty = Some(BestMapNonEmpty::new(key, value));
    }

    pub fn insert_le(&mut self, key: K, value: V) {
        if let Some(non_empty) = self.non_empty.as_mut() {
            non_empty.insert_le(key, value);
            return;
        }
        self.non_empty = Some(BestMapNonEmpty::new(key, value));
    }
}

impl<K, V> BestMap<K, V> {
    pub fn new() -> Self {
        Self { non_empty: None }
    }

    pub fn get(&self) -> Option<(&K, &V)> {
        self.non_empty.as_ref().map(BestMapNonEmpty::get)
    }

    pub fn key(&self) -> Option<&K> {
        self.non_empty.as_ref().map(BestMapNonEmpty::key)
    }

    pub fn value(&self) -> Option<&V> {
        self.non_empty.as_ref().map(BestMapNonEmpty::value)
    }

    pub fn into_key_and_value(self) -> Option<(K, V)> {
        self.non_empty.map(BestMapNonEmpty::into_key_and_value)
    }

    pub fn into_key(self) -> Option<K> {
        self.non_empty.map(BestMapNonEmpty::into_key)
    }

    pub fn into_value(self) -> Option<V> {
        self.non_empty.map(BestMapNonEmpty::into_value)
    }

    pub fn is_empty(&self) -> bool {
        self.non_empty.is_none()
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy)]
pub struct BestSetNonEmpty<T>(BestMapNonEmpty<T, ()>);

impl<T: PartialOrd> BestSetNonEmpty<T> {
    pub fn insert_gt(&mut self, value: T) {
        self.0.insert_gt(value, ());
    }
    pub fn insert_ge(&mut self, value: T) {
        self.0.insert_ge(value, ());
    }
    pub fn insert_lt(&mut self, value: T) {
        self.0.insert_lt(value, ());
    }
    pub fn insert_le(&mut self, value: T) {
        self.0.insert_le(value, ());
    }
}

impl<T> BestSetNonEmpty<T> {
    pub fn new(value: T) -> Self {
        BestSetNonEmpty(BestMapNonEmpty::new(value, ()))
    }

    pub fn get(&self) -> &T {
        self.0.key()
    }
    pub fn into_value(self) -> T {
        self.0.into_key()
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy)]
pub struct BestSet<T>(BestMap<T, ()>);
impl<T> Default for BestSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: PartialOrd> BestSet<T> {
    pub fn insert_gt(&mut self, value: T) {
        self.0.insert_gt(value, ());
    }
    pub fn insert_ge(&mut self, value: T) {
        self.0.insert_ge(value, ());
    }
    pub fn insert_lt(&mut self, value: T) {
        self.0.insert_lt(value, ());
    }
    pub fn insert_le(&mut self, value: T) {
        self.0.insert_le(value, ());
    }
}

impl<T> BestSet<T> {
    pub fn new() -> Self {
        BestSet(BestMap::new())
    }
    pub fn get(&self) -> Option<&T> {
        self.0.key()
    }
    pub fn into_value(self) -> Option<T> {
        self.0.into_key()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct BestMultiSet<T>(Vec<T>);
impl<T> Default for BestMultiSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub type BestMultiSetIter<'a, T> = ::std::slice::Iter<'a, T>;
pub type BestMultiSetDrain<'a, T> = ::std::vec::Drain<'a, T>;

impl<T> BestMultiSet<T> {
    pub fn new() -> Self {
        BestMultiSet(Vec::new())
    }
    pub fn iter(&self) -> BestMultiSetIter<T> {
        self.0.iter()
    }
    pub fn drain(&mut self) -> BestMultiSetDrain<T> {
        self.0.drain(..)
    }
    pub fn first(&self) -> Option<&T> {
        self.0.first()
    }
    pub fn clear(&mut self) {
        self.0.clear();
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    fn replace_with_single(&mut self, value: T) {
        self.0.clear();
        self.0.push(value);
    }
    pub fn insert_lt_by<F>(&mut self, value: T, mut compare: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        match self.0.first().map(|v| compare(&value, v)) {
            None | Some(Ordering::Equal) => self.0.push(value),
            Some(Ordering::Less) => self.replace_with_single(value),
            Some(Ordering::Greater) => (),
        }
    }
    pub fn insert_gt_by<F>(&mut self, value: T, mut compare: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        match self.0.first().map(|v| compare(&value, v)) {
            None | Some(Ordering::Equal) => self.0.push(value),
            Some(Ordering::Greater) => self.replace_with_single(value),
            Some(Ordering::Less) => (),
        }
    }
}

impl<T: PartialOrd> BestMultiSet<T> {
    pub fn insert_lt(&mut self, value: T) {
        match self.0.first().map(|v| value.partial_cmp(v)) {
            None | Some(Some(Ordering::Equal)) => self.0.push(value),
            Some(None) | Some(Some(Ordering::Greater)) => (),
            Some(Some(Ordering::Less)) => self.replace_with_single(value),
        }
    }
    pub fn insert_gt(&mut self, value: T) {
        match self.0.first().map(|v| value.partial_cmp(v)) {
            None | Some(Some(Ordering::Equal)) => self.0.push(value),
            Some(None) | Some(Some(Ordering::Less)) => (),
            Some(Some(Ordering::Greater)) => self.replace_with_single(value),
        }
    }
}

impl<'a, T> IntoIterator for &'a BestMultiSet<T> {
    type Item = &'a T;
    type IntoIter = BestMultiSetIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
