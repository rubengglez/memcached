use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
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

#[derive(Debug)]
pub struct List {
    first_node: Option<Rc<RefCell<Node>>>,
    last_node: Option<Rc<RefCell<Node>>>,
}

impl List {
    fn insert_at_the_end(&mut self, data: &str) -> &Self {
        if self.last_node.is_none() {
            let f = Rc::new(RefCell::new(Node::new(data.to_string())));
            self.first_node = Some(f.clone());
            self.last_node = Some(f);
        } else {
            let mut new_node = Node::new(data.to_string());
            new_node.prev = Some(Rc::downgrade(&self.last_node.as_ref().unwrap()));
            let rc = Rc::new(RefCell::new(new_node));
            self.last_node = Some(rc);
        }

        self
    }

    pub fn insert_at_the_beginning(&mut self, data: &str) -> &Self {
        if self.first_node.is_none() {
            let f = Rc::new(RefCell::new(Node::new(data.to_string())));
            self.first_node = Some(f.clone());
            self.last_node = Some(f);
        } else {
            let mut new_node = Node::new(data.to_string());
            new_node.next = Some(self.first_node.as_mut().unwrap().clone());
            let rc = Rc::new(RefCell::new(new_node));
            self.first_node.as_mut().unwrap().borrow_mut().prev = Some(Rc::downgrade(&rc));
            self.first_node = Some(rc);
        }

        self
    }

    pub fn insert_at_beginning_and_drop_last_node(&mut self, data: &str) {
        self.insert_at_the_beginning(data);

        if self.last_node.as_ref().unwrap().borrow().value.eq(data) {
            return;
        }

        let prev = self
            .last_node
            .as_mut()
            .unwrap()
            .borrow_mut()
            .prev
            .clone()
            .unwrap()
            .upgrade()
            .unwrap();
        prev.borrow_mut().next = None;
        self.last_node = Some(prev);
    }

    pub fn last_value(&self) -> Option<String> {
        if self.last_node.is_none() {
            return None;
        }

        Some(self.last_node.clone().unwrap().borrow().value.to_owned())
    }

    fn first_value(&self) -> Option<String> {
        if self.first_node.is_none() {
            return None;
        }

        Some(self.first_node.clone().unwrap().borrow().value.to_owned())
    }

    fn find_next_value(&self, value: &str) -> Option<String> {
        let mut node = self.first_node.clone();
        if node.is_none() {
            return None;
        }

        loop {
            if node.is_none() {
                break;
            }

            if node.as_ref().unwrap().borrow().next.is_none() {
                return None;
            }

            if node.as_ref().unwrap().borrow().value.eq(value) {
                return Some(
                    node.unwrap()
                        .borrow()
                        .next
                        .clone()
                        .unwrap()
                        .borrow()
                        .value
                        .to_string(),
                );
            }

            node = node.unwrap().borrow().next.clone();
        }

        None
    }

    pub fn find_and_move_first_place(&mut self, value: &str) -> bool {
        let mut node = self.first_node.clone();
        if node.is_none() {
            return false;
        }

        loop {
            if node.as_ref().unwrap().borrow().next.is_none() {
                break;
            }

            if node.as_ref().unwrap().borrow().value.eq(value) {
                /* Node -> firstNode -> 2 -> lastNode -> None

                2.prev.next = 2.next
                2.prev = None
                2.next = firstNode
                firstNode = 2 */
                let prev = node.as_ref().unwrap().borrow_mut().prev.clone().unwrap().upgrade();
                prev.clone().unwrap().borrow_mut().next = node.as_ref().unwrap().borrow_mut().next.clone();
                prev.unwrap().borrow_mut().prev = None;
                self.first_node = node;
                return true;
            }

            node = node.unwrap().borrow_mut().next.clone();
        }

        false
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
        list.insert_at_the_end("hello");
        list.insert_at_the_end("hello2");
        list.insert_at_the_beginning("hello3");
        list.insert_at_the_end("world");
        assert_eq!(list.last_value(), Some("world".to_string()));
        assert_eq!(list.first_value(), Some("hello3".to_string()));
    }

    #[test]
    fn should_create_a_list_inserting_always_at_the_beginning() {
        let mut list = List::default();
        list.insert_at_the_beginning("hello");
        list.insert_at_the_beginning("hello2");
        list.insert_at_the_beginning("hello3");
        list.insert_at_the_beginning("world");
        assert_eq!(list.first_value(), Some("world".to_string()));
        assert_eq!(list.last_value(), Some("hello".to_string()));
    }

    #[test]
    fn should_mark_and_move_at_first_place_if_exists_in_the_middle() {
        let mut list = List::default();
        list.insert_at_the_beginning("hello");
        list.insert_at_the_beginning("hello2");
        list.insert_at_the_beginning("hello3");
        assert!(list.find_and_move_first_place("hello2"));
        assert_eq!(list.first_value(), Some("hello2".to_string()));
        let next_value = list.find_next_value("hello2");
        assert_eq!(next_value, Some("hello3".to_string()));
        assert_eq!(list.last_value(), Some("hello".to_string()));
    }

    #[test]
    fn should_insert_beginning_and_drop_last_value() {
        let mut list = List::default();
        list.insert_at_the_beginning("hello");
        list.insert_at_the_beginning("hello2");
        list.insert_at_the_beginning("hello3");
        list.insert_at_beginning_and_drop_last_node("hello4");
        assert_eq!(list.first_value(), Some("hello4".to_string()));
        assert_eq!(list.last_value(), Some("hello2".to_string()));
    }

    #[test]
    fn should_insert_beginning_and_skip_dropping_last_value_when_only_one_item() {
        let mut list = List::default();
        list.insert_at_beginning_and_drop_last_node("hello4");
        assert_eq!(list.first_value(), Some("hello4".to_string()));
        assert_eq!(list.last_value(), Some("hello4".to_string()));
    }
}
