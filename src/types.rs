use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::item::Item;

pub type Store = Arc<Mutex<HashMap<String, Item>>>;
