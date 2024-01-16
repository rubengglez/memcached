use crate::{item::Item, types::Store};

pub struct Commands {
    store: Store,
}

type ResultCommand = String;

pub struct CommandDto {
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

    pub fn set(&mut self, data: CommandDto) -> ResultCommand {
        self.store.lock().unwrap().insert(
            data.key,
            Item::new(
                data.flags,
                data.exptime,
                data.value_size_in_bytes,
                data.value,
            ),
        );

        String::from("STORED\r\n")
    }

    pub fn get(&mut self, key: &str) -> ResultCommand {
        return match self.store.lock().unwrap().get(key) {
            None => String::from("END\r\n"),
            Some(item) => {
                if item.expired() {
                    return String::from("END\r\n");
                }
                let mut message = format!("VALUE {} {} {}\r\n", key, item.flags, item.value_length);
                message += &item.value;
                message += "\r\nEND\r\n";

                message
            }
        };
    }

    pub fn add(&mut self, data: CommandDto) -> ResultCommand {
        let mut unlocked_store = self.store.lock().unwrap();

        match unlocked_store.get(&data.key) {
            None => {
                unlocked_store.insert(
                    data.key,
                    Item::new(
                        data.flags,
                        data.exptime,
                        data.value_size_in_bytes,
                        data.value,
                    ),
                );

                String::from("STORED\r\n")
            }
            Some(item) => {
                if item.expired() {
                    unlocked_store.insert(
                        data.key,
                        Item::new(
                            data.flags,
                            data.exptime,
                            data.value_size_in_bytes,
                            data.value,
                        ),
                    );

                    String::from("STORED\r\n")
                } else {
                    String::from("NOT_STORED\r\n")
                }
            }
        }
    }

    pub fn replace(&mut self, data: CommandDto) -> ResultCommand {
        let mut unlocked_store = self.store.lock().unwrap();

        match unlocked_store.get(&data.key) {
            None => String::from("NOT_STORED\r\n"),
            Some(_) => {
                unlocked_store.insert(
                    data.key,
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
    }

    pub fn append(&mut self, data: CommandDto) -> ResultCommand {
        let mut unlocked_store = self.store.lock().unwrap();

        match unlocked_store.get(&data.key) {
            None => String::from("NOT_STORED\r\n"),
            Some(_) => {
                unlocked_store.entry(data.key).and_modify(|val| {
                    val.value = val.value.to_owned() + data.value.trim_end();
                    val.value_length = val.value.len();
                });

                String::from("STORED\r\n")
            }
        }
    }

    pub fn prepend(&mut self, data: CommandDto) -> ResultCommand {
        let mut unlocked_store = self.store.lock().unwrap();

        match unlocked_store.get(&data.key) {
            None => String::from("NOT_STORED\r\n"),
            Some(_) => {
                unlocked_store.entry(data.key).and_modify(|val| {
                    val.value = data.value.trim_end().to_owned() + &val.value;
                    val.value_length = val.value.len();
                });

                String::from("STORED\r\n")
            }
        }
    }
}
