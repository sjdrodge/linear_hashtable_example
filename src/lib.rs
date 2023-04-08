use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;

const MAX_LOAD_FACTOR: f64 = 0.8;
const FIRST_ALLOCATION_SIZE: usize = 2;

fn index<T: Hash>(x: &T, modulus: usize) -> usize {
    let mut hasher = DefaultHasher::new();
    x.hash(&mut hasher);
    (hasher.finish() as usize) % modulus // should use Option to avoid dividing by zero
}

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

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    fn lookup(&self, key: &K) -> Option<usize> {
        self.entries.as_deref().map(|entries| {
            let mut i = index(key, self.capacity);
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
                        if let None = first_tombstone {
                            first_tombstone = Some(i);
                        }
                    }
                    Entry::Pair {
                        key: current_key, ..
                    } => {
                        if key == current_key {
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

    pub fn new() -> HashMap<K, V> {
        HashMap {
            capacity: 0,
            len: 0,
            tombstone_count: 0,
            entries: None,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get(&self, key: &K) -> Option<&V> {
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
            let mut v = Vec::with_capacity(FIRST_ALLOCATION_SIZE);
            for _ in 0..FIRST_ALLOCATION_SIZE {
                v.push(Entry::Empty);
            }
            self.capacity = FIRST_ALLOCATION_SIZE;
            self.entries = Some(v.into_boxed_slice());
        } else if ((self.len + self.tombstone_count + 1) / self.capacity) as f64 > MAX_LOAD_FACTOR {
            let new_capacity = self.capacity * 2;
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
            old_map.entries.map(|entries| {
                for entry in Vec::from(entries).into_iter() {
                    if let Entry::Pair { key, value } = entry {
                        self.insert(key, value);
                    }
                }
            });
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

    pub fn remove(&mut self, key: K) -> Option<V> {
        self.lookup(&key).and_then(|i| {
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
        })
    }
}
