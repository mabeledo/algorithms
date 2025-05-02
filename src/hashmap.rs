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
        self.deleted || self.key.is_none()
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

    fn probe_next(&self, slot: usize) -> usize {
        (slot + 1) & (self.capacity - 1)
    }

    /// Find an entry in the map.
    /// This will either return:
    ///   * an effectively free slot, either marked as deleted or actually vacant, or
    ///   * the slot where the entry currently lives, or
    ///   * None, if there aren't any more slots available.
    /// # Arguments
    /// * `key` - The key to look up
    /// # Returns
     /// * `Option<&V>` - Returns `Some(&V)` if the key exists, or `None` if it doesn't
    fn find_entry(&self, key: &K) -> Option<usize> {
        let mut slot = self.find_slot(key);
        for _ in 0..self.capacity {
            let entry = &self.entries[slot];
            if entry.is_empty() || entry.key.as_ref().unwrap() == key {
                // Slot is available or contains the key we're looking for.
                return Some(slot);
            }

            slot = self.probe_next(slot);
        }
        None
    }

    /// Doubles the capacity of the hash map and rehashes all existing entries.
    fn resize(&mut self) {
        let new_capacity = self.capacity * 2;
        let mut hashmap: HashMap<K, V> = HashMap::new(new_capacity);

        for entry in self.entries.iter() {
            if !entry.deleted && entry.key.is_some() {
                hashmap.insert(entry.key.clone().unwrap(), entry.value.clone().unwrap());
            }
        }

        *self = hashmap;
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
        let slot = self.find_entry(&key).expect("Out of slots.");
        let entry = &mut self.entries[slot];

        if entry.deleted || entry.key.is_none() {
            // There is nothing here, let's insert.
            self.size += 1;
            entry.key = Some(key);
            entry.value = Some(value);
            entry.deleted = false;
            None
        } else {
            // There is already something here. Replace the value and return the old one.
            entry.value.replace(value)
        }
    }

    // Retrieves a reference to the value associated with the given key.
    ///
    /// # Arguments
    /// * `key` - The key to look up
    ///
    /// # Returns
    /// * `Option<&V>` - Returns `Some(&V)` if the key exists, or `None` if it doesn't
    fn get(&self, key: &K) -> Option<&V> {
        match self.find_entry(key) {
            Some(slot) => {
                let entry = &self.entries[slot];
                if entry.is_empty() {
                    return None;
                }
                entry.value.as_ref()
            },
            None => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
