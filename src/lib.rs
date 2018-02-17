extern crate serde;
#[macro_use]
extern crate serde_derive;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BestMapNonEmpty<K: PartialOrd, V> {
    key: K,
    value: V,
    len: usize,
}

impl<K: PartialOrd, V> BestMapNonEmpty<K, V> {
    pub fn new(key: K, value: V) -> Self {
        BestMapNonEmpty {
            key: key,
            value: value,
            len: 1,
        }
    }

    pub fn insert_gt(&mut self, key: K, value: V) {
        if key > self.key {
            self.key = key;
            self.value = value;
        }
        self.len += 1;
    }

    pub fn insert_ge(&mut self, key: K, value: V) {
        if key >= self.key {
            self.key = key;
            self.value = value;
        }
        self.len += 1;
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

    pub fn into(self) -> (K, V) {
        (self.key, self.value)
    }
    pub fn into_key(self) -> K {
        self.key
    }
    pub fn into_value(self) -> V {
        self.value
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

    pub fn get(&self) -> Option<(&K, &V)> {
        self.non_empty.as_ref().map(BestMapNonEmpty::get)
    }

    pub fn key(&self) -> Option<&K> {
        self.non_empty.as_ref().map(BestMapNonEmpty::key)
    }

    pub fn value(&self) -> Option<&V> {
        self.non_empty.as_ref().map(BestMapNonEmpty::value)
    }

    pub fn into(self) -> Option<(K, V)> {
        self.non_empty.map(BestMapNonEmpty::into)
    }

    pub fn into_key(self) -> Option<K> {
        self.non_empty.map(BestMapNonEmpty::into_key)
    }

    pub fn into_value(self) -> Option<V> {
        self.non_empty.map(BestMapNonEmpty::into_value)
    }

    pub fn len(&self) -> usize {
        self.non_empty
            .as_ref()
            .map(BestMapNonEmpty::len)
            .unwrap_or(0)
    }
}
