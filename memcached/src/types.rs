use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::item::Item;

pub type Store = Arc<Mutex<HashMap<String, Item>>>;

pub const WRITE_COMMANDS: [&str; 5] = ["set", "replace", "add", "append", "prepend"];
pub const READ_COMMANDS: [&str; 1] = ["get"];

pub const MAX_ALLOWED_ITEMS: usize = 5;