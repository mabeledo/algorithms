use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug)]
struct Entry<K, V> {
    key: Option<K>,
    value: Option<V>,
    deleted: bool,
}

impl<K, V> Entry<K, V> {
    fn new() -> Self {
        Self {
            key: None,
            value: None,
            deleted: false,
        }
    }

    fn is_empty(&self) -> bool {
        self.key.is_none() && !self.deleted
    }
}

#[derive(Debug)]
pub struct HashMap<K, V> {
    entries: Vec<Entry<K, V>>,
    capacity: usize,
    size: usize,
}

impl<K, V> HashMap<K, V>
where
    K: Eq + Clone + Hash,
    V: Clone,
{
    fn new(initial_capacity: usize) -> Self {
        let capacity = initial_capacity.next_power_of_two();
        Self {
            entries: (0..capacity).map(|_| Entry::new()).collect(),
            capacity,
            size: 0,
        }
    }

    /// Calculates the slot index for a given key in the hash map.
    ///
    /// # Arguments
    /// * `key` - Reference to the key to find a slot for
    ///
    /// # Returns
    /// * `usize` - The index of the slot in the internal storage where the key should be placed
    ///
    /// # Note
    /// This implementation requires that the capacity is a power of 2 to ensure
    /// proper distribution of values using the bitwise AND operation.
    fn find_slot(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish() as usize;

        let mask = self.capacity - 1;
        hash & mask
    }

    /// Inserts a key-value pair into the hash map.
    ///
    /// If there was no key present in the slot the value is being inserted at, `None` is returned.
    /// Otherwise, the previous value is returned.
    ///
    /// # Arguments
    /// * `key` - The key to insert
    /// * `value` - The value to associate with the key
    ///
    /// # Returns
    /// * `Option<V>` - The previous value associated with the key, if any
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        // Check if there is enough space. If not, resize.
        if (self.size + 1) > ((self.capacity * 3) / 4) {
            self.resize();
        }
        
        // Find the slot.
        let slot = self.find_slot(&key);
        let entry = &mut self.entries[slot];
        
        if entry.key.is_none() {
            // There is nothing here, let's insert.
            self.size += 1;
            entry.key = Some(key);
            entry.value = Some(value);
            None
        } else {
            let previous_value = entry.value.replace(value);
            previous_value
        }
    }

    /// Doubles the capacity of the hash map and rehashes all existing entries.
    fn resize(&mut self) {
        let new_capacity = self.capacity * 2;
        let mut hashmap: HashMap<K, V> = HashMap::new(new_capacity);
        
        for entry in self.entries.iter() {
            if !entry.is_empty() && entry.deleted == false {
                hashmap.insert(entry.key.clone().unwrap(), entry.value.clone().unwrap());
            }
        }
        
        *self = hashmap;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
