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

    pub fn insert_lt_cmp<F>(&mut self, key: K, value: V, mut cmp: F)
    where
        F: FnMut(&K) -> Ordering,
    {
        match cmp(&key) {
            Ordering::Less => self.insert(key, value),
            Ordering::Equal | Ordering::Greater => (),
        }
    }

    pub fn insert_le_cmp<F>(&mut self, key: K, value: V, mut cmp: F)
    where
        F: FnMut(&K) -> Ordering,
    {
        match cmp(&key) {
            Ordering::Less | Ordering::Equal => self.insert(key, value),
            Ordering::Greater => (),
        }
    }

    pub fn insert_gt_cmp<F>(&mut self, key: K, value: V, mut cmp: F)
    where
        F: FnMut(&K) -> Ordering,
    {
        match cmp(&key) {
            Ordering::Greater => self.insert(key, value),
            Ordering::Equal | Ordering::Less => (),
        }
    }

    pub fn insert_ge_cmp<F>(&mut self, key: K, value: V, mut cmp: F)
    where
        F: FnMut(&K) -> Ordering,
    {
        match cmp(&key) {
            Ordering::Greater | Ordering::Equal => self.insert(key, value),
            Ordering::Less => (),
        }
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

    pub fn insert_lt_cmp<F>(&mut self, key: K, value: V, cmp: F)
    where
        F: FnMut(&K) -> Ordering,
    {
        if let Some(non_empty) = self.non_empty.as_mut() {
            non_empty.insert_lt_cmp(key, value, cmp);
            return;
        }
        self.non_empty = Some(BestMapNonEmpty::new(key, value));
    }

    pub fn insert_le_cmp<F>(&mut self, key: K, value: V, cmp: F)
    where
        F: FnMut(&K) -> Ordering,
    {
        if let Some(non_empty) = self.non_empty.as_mut() {
            non_empty.insert_le_cmp(key, value, cmp);
            return;
        }
        self.non_empty = Some(BestMapNonEmpty::new(key, value));
    }

    pub fn insert_gt_cmp<F>(&mut self, key: K, value: V, cmp: F)
    where
        F: FnMut(&K) -> Ordering,
    {
        if let Some(non_empty) = self.non_empty.as_mut() {
            non_empty.insert_gt_cmp(key, value, cmp);
            return;
        }
        self.non_empty = Some(BestMapNonEmpty::new(key, value));
    }

    pub fn insert_ge_cmp<F>(&mut self, key: K, value: V, cmp: F)
    where
        F: FnMut(&K) -> Ordering,
    {
        if let Some(non_empty) = self.non_empty.as_mut() {
            non_empty.insert_ge_cmp(key, value, cmp);
            return;
        }
        self.non_empty = Some(BestMapNonEmpty::new(key, value));
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
    pub fn insert_lt_cmp<F>(&mut self, value: T, cmp: F)
    where
        F: FnMut(&T) -> Ordering,
    {
        self.0.insert_lt_cmp(value, (), cmp);
    }
    pub fn insert_le_cmp<F>(&mut self, value: T, cmp: F)
    where
        F: FnMut(&T) -> Ordering,
    {
        self.0.insert_le_cmp(value, (), cmp);
    }
    pub fn insert_gt_cmp<F>(&mut self, value: T, cmp: F)
    where
        F: FnMut(&T) -> Ordering,
    {
        self.0.insert_gt_cmp(value, (), cmp);
    }
    pub fn insert_ge_cmp<F>(&mut self, value: T, cmp: F)
    where
        F: FnMut(&T) -> Ordering,
    {
        self.0.insert_ge_cmp(value, (), cmp);
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
    pub fn insert_lt_cmp<F>(&mut self, value: T, cmp: F)
    where
        F: FnMut(&T) -> Ordering,
    {
        self.0.insert_lt_cmp(value, (), cmp);
    }
    pub fn insert_le_cmp<F>(&mut self, value: T, cmp: F)
    where
        F: FnMut(&T) -> Ordering,
    {
        self.0.insert_le_cmp(value, (), cmp);
    }
    pub fn insert_gt_cmp<F>(&mut self, value: T, cmp: F)
    where
        F: FnMut(&T) -> Ordering,
    {
        self.0.insert_gt_cmp(value, (), cmp);
    }
    pub fn insert_ge_cmp<F>(&mut self, value: T, cmp: F)
    where
        F: FnMut(&T) -> Ordering,
    {
        self.0.insert_ge_cmp(value, (), cmp);
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
