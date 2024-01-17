use std::collections::HashMap;

use crate::{
    item::Item,
    types::{Store, MAX_ALLOWED_ITEMS},
};

#[derive(Debug)]
pub struct StoreManager {
    store: HashMap<String, Item>,
    /**
     * This will fake that the store will reached full capacity
     */
    max_allowed_items: usize,
}

impl StoreManager {
    fn new(max_allowed_items: usize) -> StoreManager {
        StoreManager {
            store: HashMap::new(),
            max_allowed_items,
        }
    }

    pub fn insert_or_update(&mut self, key: String, value: Item) {
        if self.store.len() < self.max_allowed_items {
            self.store.insert(key, value);
            return;
        }

        if let Some(_) = self.store.get(&key) {
            self.store.insert(key, value);
            return;
        }

        // 1.1.1 The value is associated with a key so we can update it?
        // 1.1.1.1 If yes, then UPDATE and finish.
        // 1.1.1.2 If not, then LRU algorithm to find the key-value pair to remove
    }

    pub fn get(&self, key: String) -> Option<&Item> {
        self.store.get(&key)
    }
}

impl Default for StoreManager {
    fn default() -> Self {
        StoreManager {
            store: HashMap::new(),
            max_allowed_items: MAX_ALLOWED_ITEMS,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::item::tests::ItemBuilder;

    use super::*;

    #[test]
    fn should_insert_item_due_to_enough_space() {
        let mut st_manager = StoreManager::new(2);
        let key = "key".to_owned();
        let item = ItemBuilder::new().build();

        st_manager.insert_or_update(key, item);

        assert_eq!(st_manager.store.len(), 1);
    }
}
