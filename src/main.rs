use chrono::prelude::*;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, BufWriter, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    ops::{Add, Sub},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::config::MyConfig;

mod config;
mod errors;

struct Item {
    flags: u16,
    exptime: i64,
    value: String,
    value_length: usize,
}

impl Item {
    fn new(flags: u16, exptime: isize, value_length: usize, value: String) -> Self {
        let mut will_expire_on = Utc::now();

        if exptime < 0 {
            will_expire_on = will_expire_on.sub(Duration::new(1, 0));
        } else {
            will_expire_on = will_expire_on.add(Duration::new(exptime as u64, 0));
        }

        println!("que es now: {}", Utc::now().timestamp());
        println!("que es will_expire_on: {}", will_expire_on);

        Item {
            flags,
            exptime: will_expire_on.timestamp(),
            value_length,
            value,
        }
    }

    fn expired(&self) -> bool {
        let now = Utc::now().timestamp();

        println!("que es now: {}", now);
        println!("que es exptime: {}", self.exptime);

        self.exptime < now
    }
}

type Store = Arc<Mutex<HashMap<String, Item>>>;

fn main() {
    let config = match MyConfig::parse(std::env::args(), None) {
        Ok(c) => c,
        Err(err) => panic!("Invalid arguments {:?}", err),
    };

    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], config.port))).unwrap();

    println!("Listening on port {}", config.port);

    let store = Arc::new(Mutex::new(HashMap::new()));

    for stream_wrapper in listener.incoming() {
        let stream = stream_wrapper.unwrap();

        let mut store: Arc<Mutex<HashMap<String, Item>>> = Arc::clone(&store);
        thread::spawn(move || {
            handle_connection(stream, &mut store);
        });
    }
}

fn response_wrong_number_of_arguments(writer: &mut BufWriter<&TcpStream>, command: &str) {
    response(
        writer,
        &format!("wrong number of arguments for {command}\r\n"),
    );
}

fn response(writer: &mut BufWriter<&TcpStream>, response: &str) {
    writer.write_all(response.as_bytes()).unwrap();
    writer.flush().unwrap();
}

fn handle_connection(stream: TcpStream, store: &mut Store) {
    loop {
        let mut read_buffer = BufReader::new(&stream);
        let mut write_buffer = BufWriter::new(&stream);

        let mut command = String::new();
        // TODO: blocking call. Limit number of bytes taken
        read_buffer.read_line(&mut command).unwrap();
        let mut iterator = command.split_whitespace();
        let size = iterator.clone().count();
        iterator.next();
        let key = iterator.next().unwrap();
        if command.starts_with("set") {
            if size != 5 && size != 6 {
                response_wrong_number_of_arguments(&mut write_buffer, "set");
                continue;
            }

            let flags: u16 = iterator.next().unwrap().parse().unwrap();
            let exptime: isize = iterator.next().unwrap().parse().unwrap();
            let value_size_in_bytes: usize = iterator.next().unwrap().parse().unwrap();
            let no_reply = iterator.next();

            // TODO: validate size of payload with the value_size_in_bytes
            let mut value = String::new();
            read_buffer.read_line(&mut value).unwrap();

            store.lock().unwrap().insert(
                key.to_string(),
                Item::new(flags, exptime, value_size_in_bytes, value),
            );

            if no_reply == None {
                response(&mut write_buffer, "STORED\r\n");
            }
        } else {
            if size != 2 {
                response_wrong_number_of_arguments(&mut write_buffer, "get");
                continue;
            }

            match store.lock().unwrap().get(key) {
                None => {
                    response(&mut write_buffer, "END\r\n");
                }
                Some(item) => {
                    if item.expired() {
                        response(&mut write_buffer, "END\r\n");
                        continue;
                    }
                    let mut message =
                        format!("VALUE {} {} {}\r\n", key, item.flags, item.value_length);
                    message += &item.value;
                    message += "END\r\n";

                    response(&mut write_buffer, &message);
                }
            }
        }
    }
}
