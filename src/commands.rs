use crate::{item::Item, types::Store};

pub struct Commands {
    store: Store,
}

type ResultCommand = String;

pub struct SetData {
    pub(crate) key: String,
    pub(crate) value: String,
    pub(crate) flags: u16,
    pub(crate) exptime: isize,
    pub(crate) value_size_in_bytes: usize,
}

impl Commands {
    pub fn new(store: Store) -> Commands {
        Commands { store }
    }

    pub fn set(&mut self, data: SetData) -> ResultCommand {
        self.store.lock().unwrap().insert(
            data.key.to_string(),
            Item::new(
                data.flags,
                data.exptime,
                data.value_size_in_bytes,
                data.value,
            ),
        );

        String::from("STORED\r\n")
    }
}
