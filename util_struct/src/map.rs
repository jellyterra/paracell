// Copyright 2025 Jelly Terra <jellyterra@symboltics.com>
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, Debug)]
pub struct OrderedHashMap<K, V>
where
    K: Hash + Eq + Clone,
{
    pub vals: Vec<V>,
    pub keys: Vec<K>,
    pub map: HashMap<K, usize>,
}

impl<K, V> OrderedHashMap<K, V>
where
    K: Hash + Eq + Clone,
{
    pub fn new() -> OrderedHashMap<K, V> {
        OrderedHashMap {
            vals: Vec::new(),
            keys: Vec::new(),
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, k: K, v: V) -> usize {
        self.keys.push(k.clone());
        self.vals.push(v);
        let i = self.vals.len() - 1;
        self.map.insert(k, i);
        i
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        match self.map.get(k) {
            None => None,
            Some(v) => Some(&self.vals[v.clone()]),
        }
    }
}
