use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use crate::{item::Item, types::MAX_ALLOWED_ITEMS};

struct Node {
    prev: Option<Weak<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
    value: String,
}

impl Node {
    fn new(value: String) -> Node {
        Node {
            value,
            prev: None,
            next: None,
        }
    }
}

struct List {
    first_node: Option<Rc<RefCell<Node>>>,
    last_node: Option<Rc<RefCell<Node>>>,
}

impl List {
    fn insert_at_the_end(&mut self, data: String) -> &Self {
        if self.last_node.is_none() {
            let f = Rc::new(RefCell::new(Node::new(data)));
            self.first_node = Some(f.clone());
            self.last_node = Some(f);
        } else {
            let mut new_node = Node::new(data);
            new_node.prev = Some(Rc::downgrade(&self.last_node.as_ref().unwrap()));
            let rc = Rc::new(RefCell::new(new_node));
            self.last_node.as_mut().unwrap().borrow_mut().next = Some(rc);
        }

        self
    }

    fn insert_at_the_beginning(&mut self, data: String) -> &Self {
        if self.first_node.is_none() {
            let f = Rc::new(RefCell::new(Node::new(data)));
            self.first_node = Some(f.clone());
            self.last_node = Some(f);
        } else {
            let mut new_node = Node::new(data);
            new_node.next = Some(self.first_node.as_mut().unwrap().clone());
            let rc = Rc::new(RefCell::new(new_node));
            self.first_node.as_mut().unwrap().borrow_mut().prev = Some(Rc::downgrade(&rc));
            self.first_node = Some(rc);
        }

        self
    }
}

impl Default for List {
    fn default() -> Self {
        List {
            first_node: None,
            last_node: None,
        }
    }
}

#[cfg(test)]
mod list_tests {

    use super::*;

    #[test]
    fn should_create_a_list() {
        let mut list = List::default();
        list.insert_at_the_end("hello".to_string());
        list.insert_at_the_end("world".to_string());
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
