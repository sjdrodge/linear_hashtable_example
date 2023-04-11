use std::borrow::Borrow;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;

// Use inverse load factors to allow integer division.
const INVERSE_MAX_LOAD_FACTOR: usize = 2;
const INVERSE_MIN_LOAD_FACTOR: usize = 8;
const FIRST_ALLOCATION_SIZE: usize = 4;

#[derive(Debug)]
enum Entry<K, V> {
    Empty,
    Tombstone,
    Pair { key: K, value: V },
}

#[derive(Debug)]
pub struct HashMap<K, V> {
    capacity: usize,
    len: usize,
    tombstone_count: usize,
    entries: Option<Box<[Entry<K, V>]>>,
}

impl<K, V> Default for HashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> HashMap<K, V> {
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn new() -> Self {
        HashMap {
            capacity: 0,
            len: 0,
            tombstone_count: 0,
            entries: None,
        }
    }
}

impl<K, V> HashMap<K, V>
where
    K: Hash,
{
    fn index<Q>(&self, key: &Q) -> Option<usize>
    where
        K: Borrow<Q>,
        Q: Hash + ?Sized,
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize).checked_rem(self.capacity)
    }
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    fn lookup<Q>(&self, key: &Q) -> Option<usize>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.entries.as_deref().map(|entries| {
            let mut i = self
                .index(key)
                .expect("capacity should only be zero when entries is None.");
            let mut first_tombstone = None;
            loop {
                match &entries[i] {
                    Entry::Empty => {
                        break match first_tombstone {
                            None => i,
                            Some(tombstone) => tombstone,
                        }
                    }
                    Entry::Tombstone => {
                        if first_tombstone.is_none() {
                            first_tombstone = Some(i);
                        }
                    }
                    Entry::Pair {
                        key: current_key, ..
                    } => {
                        if key == current_key.borrow() {
                            break i;
                        }
                    }
                }
                i += 1;
                if i >= self.capacity {
                    i = 0;
                }
            }
        })
    }

    fn resize(&mut self, new_capacity: usize) {
        if new_capacity == 0 {
            *self = Self::new();
            return;
        }
        let mut v = Vec::with_capacity(new_capacity);
        for _ in 0..new_capacity {
            v.push(Entry::Empty);
        }
        let old_map = mem::replace(
            self,
            HashMap {
                capacity: new_capacity,
                len: 0,
                tombstone_count: 0,
                entries: Some(v.into_boxed_slice()),
            },
        );
        if let Some(entries) = old_map.entries {
            for entry in Vec::from(entries).into_iter() {
                if let Entry::Pair { key, value } = entry {
                    self.insert(key, value);
                }
            }
        }
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.lookup(key).and_then(|i| {
            self.entries
                .as_deref()
                .and_then(|entries| match &entries[i] {
                    Entry::Empty | Entry::Tombstone => None,
                    Entry::Pair { key: _, value } => Some(value),
                })
        })
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.capacity == 0 {
            self.resize(FIRST_ALLOCATION_SIZE);
        } else if self.capacity / (self.len + self.tombstone_count + 1) < INVERSE_MAX_LOAD_FACTOR {
            self.resize(self.capacity * 2);
        }
        self.lookup(&key).and_then(|i| {
            self.entries
                .as_deref_mut()
                .and_then(|entries| match &mut entries[i] {
                    entry @ Entry::Empty => {
                        self.len += 1;
                        *entry = Entry::Pair { key, value };
                        None
                    }
                    entry @ Entry::Tombstone => {
                        self.len += 1;
                        self.tombstone_count -= 1;
                        *entry = Entry::Pair { key, value };
                        None
                    }
                    Entry::Pair {
                        value: old_value, ..
                    } => Some(mem::replace(old_value, value)),
                })
        })
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let result = self.lookup(key).and_then(|i| {
            self.entries
                .as_deref_mut()
                .and_then(|entries| match &mut entries[i] {
                    Entry::Empty | Entry::Tombstone => None,
                    entry @ Entry::Pair { .. } => {
                        self.len -= 1;
                        self.tombstone_count += 1;
                        let old_entry = mem::replace(entry, Entry::Tombstone);
                        if let Entry::Pair { value, .. } = old_entry {
                            Some(value)
                        } else {
                            panic!("Unreachable. Entry is already known to be a Pair.")
                        }
                    }
                })
        });
        if self.len == 0 {
            self.resize(0);
        } else if self.capacity / self.len > INVERSE_MIN_LOAD_FACTOR {
            self.resize(self.capacity / 2);
        }
        result
    }
}
