#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy)]
pub struct BestMapNonEmpty<K: PartialOrd, V> {
    key: K,
    value: V,
}

impl<K: PartialOrd, V> BestMapNonEmpty<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self {
            key: key,
            value: value,
        }
    }

    pub fn insert_gt(&mut self, key: K, value: V) {
        if key > self.key {
            self.key = key;
            self.value = value;
        }
    }

    pub fn insert_ge(&mut self, key: K, value: V) {
        if key >= self.key {
            self.key = key;
            self.value = value;
        }
    }

    pub fn insert_lt(&mut self, key: K, value: V) {
        if key < self.key {
            self.key = key;
            self.value = value;
        }
    }

    pub fn insert_le(&mut self, key: K, value: V) {
        if key <= self.key {
            self.key = key;
            self.value = value;
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
pub struct BestMap<K: PartialOrd, V> {
    non_empty: Option<BestMapNonEmpty<K, V>>,
}

impl<K: PartialOrd, V> BestMap<K, V> {
    pub fn new() -> Self {
        Self { non_empty: None }
    }

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
pub struct BestSetNonEmpty<T: PartialOrd>(BestMapNonEmpty<T, ()>);

impl<T: PartialOrd> BestSetNonEmpty<T> {
    pub fn new(value: T) -> Self {
        BestSetNonEmpty(BestMapNonEmpty::new(value, ()))
    }
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
    pub fn get(&self) -> &T {
        self.0.key()
    }
    pub fn into_value(self) -> T {
        self.0.into_key()
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy)]
pub struct BestSet<T: PartialOrd>(BestMap<T, ()>);

impl<T: PartialOrd> BestSet<T> {
    pub fn new() -> Self {
        BestSet(BestMap::new())
    }
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
