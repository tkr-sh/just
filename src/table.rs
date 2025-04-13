use {super::*, std::collections::btree_map};

#[derive(Debug, PartialEq, Serialize)]
#[serde(transparent)]
pub struct Table<'key, V: Keyed<'key>> {
    map: BTreeMap<&'key str, V>,
}

impl<'key, V: Keyed<'key>> Table<'key, V> {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, value: V) {
        self.map.insert(value.key(), value);
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn get(&self, key: &str) -> Option<&V> {
        self.map.get(key)
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn values(&self) -> btree_map::Values<&'key str, V> {
        self.map.values()
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }

    pub fn keys(&self) -> btree_map::Keys<&'key str, V> {
        self.map.keys()
    }

    pub fn iter(&self) -> btree_map::Iter<&'key str, V> {
        self.map.iter()
    }

    pub fn pop(&mut self) -> Option<V> {
        let key = self.map.keys().next().copied()?;
        self.map.remove(key)
    }

    pub fn remove(&mut self, key: &str) -> Option<V> {
        self.map.remove(key)
    }
}

impl<'key, V: Keyed<'key>> Default for Table<'key, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'key, V: Keyed<'key>> FromIterator<V> for Table<'key, V> {
    fn from_iter<I: IntoIterator<Item = V>>(iter: I) -> Self {
        Self {
            map: iter.into_iter().map(|value| (value.key(), value)).collect(),
        }
    }
}

impl<'key, V: Keyed<'key>> Index<&'key str> for Table<'key, V> {
    type Output = V;

    #[inline]
    fn index(&self, key: &str) -> &V {
        self.map.get(key).expect("no entry found for key")
    }
}

impl<'key, V: Keyed<'key>> IntoIterator for Table<'key, V> {
    type IntoIter = btree_map::IntoIter<&'key str, V>;
    type Item = (&'key str, V);

    fn into_iter(self) -> btree_map::IntoIter<&'key str, V> {
        self.map.into_iter()
    }
}

impl<'table, V: Keyed<'table> + 'table> IntoIterator for &'table Table<'table, V> {
    type IntoIter = btree_map::Iter<'table, &'table str, V>;
    type Item = (&'table &'table str, &'table V);

    fn into_iter(self) -> btree_map::Iter<'table, &'table str, V> {
        self.map.iter()
    }
}
