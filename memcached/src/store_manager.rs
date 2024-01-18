use std::collections::HashMap;

use crate::{item::Item, types::MAX_ALLOWED_ITEMS};

struct Node<'a> {
    prev: Option<&'a Node<'a>>,
    next: Option<&'a Node<'a>>,
    value: String,
}

impl<'a> Node<'a> {
    fn new(value: String, prev: Option<&'a Node<'a>>, next: Option<&'a Node<'a>>) -> Node<'a> {
        Node { value, prev, next }
    }

    fn addPrev(&mut self, prev: Option<&'a Node<'a>>) {
        self.prev = prev;
    }

    fn addNext(&mut self, next: Option<&'a Node<'a>>) {
        self.next = next;
    }
}

#[cfg(test)]
mod list_tests {
    use super::*;

    #[test]
    fn should_create_a_list() {
        let mut node = Node::new("value".to_string(), None, None);
        let mut node2 = Node::new("value2".to_string(), None, None);
        {
            node.addNext(Some(&node2));
        }
        node2.addPrev(Some(&node));
    }
}

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

    #[test]
    fn should_insert_second_item_due_to_enough_space() {
        let mut st_manager = StoreManager::new(2);
        let key = "key".to_owned();
        let item = ItemBuilder::new().build();

        st_manager.insert_or_update(key, item.clone());

        let key2 = "key2".to_owned();
        st_manager.insert_or_update(key2, item);
        assert_eq!(st_manager.store.len(), 2);
    }

    #[test]
    fn should_replace_item_when_there_is_no_space_left() {
        let mut st_manager = StoreManager::new(1);
        let key = "key".to_owned();
        let item = ItemBuilder::new().build();

        st_manager.insert_or_update(key, item);

        let key2 = "key2".to_owned();
        let item2 = ItemBuilder::new().build();
        st_manager.insert_or_update(key2.clone(), item2.clone());
        assert_eq!(st_manager.store.len(), 1);

        let stored_item = st_manager.get(key2).unwrap();
        assert_eq!(stored_item.value, item2.value);
    }

    #[test]
    fn should_replace_least_used_item_when_there_is_no_space_left() {
        let mut st_manager = StoreManager::new(3);
        let key = "key".to_owned();
        let item = ItemBuilder::new().build();
        let key2 = "key2".to_owned();
        let item2 = ItemBuilder::new().build();
        let key3 = "key3".to_owned();
        let item3 = ItemBuilder::new().build();

        st_manager.insert_or_update(key.clone(), item.clone());
        st_manager.insert_or_update(key2.clone(), item2.clone());
        st_manager.insert_or_update(key3.clone(), item3.clone());

        assert_eq!(st_manager.store.len(), st_manager.max_allowed_items);

        st_manager.insert_or_update(key.clone(), item.clone());

        assert_eq!(st_manager.store.len(), st_manager.max_allowed_items);
        assert!(st_manager.get(key2.clone()).is_some());
        assert!(st_manager.get(key3.clone()).is_some());
        assert!(st_manager.get(key.clone()).is_some());

        let key4 = "key4".to_owned();
        let item4 = ItemBuilder::new().build();

        st_manager.insert_or_update(key4.clone(), item4.clone());
        assert_eq!(st_manager.store.len(), st_manager.max_allowed_items);
        assert!(st_manager.get(key2).is_none());
        assert!(st_manager.get(key3.clone()).is_some());
        assert!(st_manager.get(key.clone()).is_some());
        assert!(st_manager.get(key4.clone()).is_some());

        let key5 = "key5".to_owned();
        let item5 = ItemBuilder::new().build();

        st_manager.insert_or_update(key5.clone(), item5.clone());
        assert_eq!(st_manager.store.len(), st_manager.max_allowed_items);
        assert!(st_manager.get(key3).is_none());
        assert!(st_manager.get(key.clone()).is_some());
        assert!(st_manager.get(key4.clone()).is_some());
        assert!(st_manager.get(key5.clone()).is_some());

        st_manager.insert_or_update(key.clone(), item.clone());

        let key6 = "key6".to_owned();

        st_manager.insert_or_update(key6.clone(), ItemBuilder::new().build());
        assert_eq!(st_manager.store.len(), st_manager.max_allowed_items);
        assert!(st_manager.get(key4).is_none());
        assert!(st_manager.get(key5.clone()).is_some());
        assert!(st_manager.get(key.clone()).is_some());
        assert!(st_manager.get(key6.clone()).is_some());

        st_manager.insert_or_update(key6.clone(), ItemBuilder::new().build());
        st_manager.insert_or_update(key5.clone(), ItemBuilder::new().build());
        st_manager.insert_or_update(key.clone(), ItemBuilder::new().build());

        let key7 = "key7".to_owned();

        st_manager.insert_or_update(key7.clone(), ItemBuilder::new().build());
        assert_eq!(st_manager.store.len(), st_manager.max_allowed_items);
        assert!(st_manager.get(key6).is_none());
        assert!(st_manager.get(key5.clone()).is_some());
        assert!(st_manager.get(key.clone()).is_some());
        assert!(st_manager.get(key7.clone()).is_some());
    }
}
