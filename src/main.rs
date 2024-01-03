use std::{
    collections::HashMap,
    io::{BufRead, BufReader, BufWriter, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use commands::CommandDto;
use types::Store;

use crate::{commands::Commands, config::MyConfig, item::Item};

mod commands;
mod config;
mod errors;
mod item;
mod types;

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
    let mut commands = Commands::new(store.clone());
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

            let result = commands.set(CommandDto {
                key: key.to_string(),
                value,
                flags,
                exptime,
                value_size_in_bytes,
            });

            if no_reply == None {
                response(&mut write_buffer, &result);
            }
        } else if command.starts_with("get") {
            if size != 2 {
                response_wrong_number_of_arguments(&mut write_buffer, "get");
                continue;
            }

            let result = commands.get(key);
            response(&mut write_buffer, &result);
        } else if command.starts_with("add") {
            if size != 5 && size != 6 {
                response_wrong_number_of_arguments(&mut write_buffer, "add");
                continue;
            }

            let flags: u16 = iterator.next().unwrap().parse().unwrap();
            let exptime: isize = iterator.next().unwrap().parse().unwrap();
            let value_size_in_bytes: usize = iterator.next().unwrap().parse().unwrap();
            let no_reply = iterator.next();

            // TODO: validate size of payload with the value_size_in_bytes
            let mut value = String::new();
            read_buffer.read_line(&mut value).unwrap();

            let result = commands.add(CommandDto {
                key: key.to_string(),
                value,
                flags,
                exptime,
                value_size_in_bytes,
            });
            if no_reply == None {
                response(&mut write_buffer, &result);
            }
        } else if command.starts_with("replace") {
            if size != 5 && size != 6 {
                response_wrong_number_of_arguments(&mut write_buffer, "replace");
                continue;
            }

            let flags: u16 = iterator.next().unwrap().parse().unwrap();
            let exptime: isize = iterator.next().unwrap().parse().unwrap();
            let value_size_in_bytes: usize = iterator.next().unwrap().parse().unwrap();
            let no_reply = iterator.next();

            // TODO: validate size of payload with the value_size_in_bytes
            let mut value = String::new();
            read_buffer.read_line(&mut value).unwrap();

            let result = commands.replace(CommandDto {
                key: key.to_string(),
                value,
                flags,
                exptime,
                value_size_in_bytes,
            });
            if no_reply == None {
                response(&mut write_buffer, &result);
            }
        } else if command.starts_with("append") {
            if size != 5 && size != 6 {
                response_wrong_number_of_arguments(&mut write_buffer, "append");
                continue;
            }

            let flags: u16 = iterator.next().unwrap().parse().unwrap();
            let exptime: isize = iterator.next().unwrap().parse().unwrap();
            let value_size_in_bytes: usize = iterator.next().unwrap().parse().unwrap();
            let no_reply = iterator.next();

            // TODO: validate size of payload with the value_size_in_bytes
            let mut value = String::new();
            read_buffer.read_line(&mut value).unwrap();

            let result = commands.append(CommandDto {
                key: key.to_string(),
                value,
                flags,
                exptime,
                value_size_in_bytes,
            });
            if no_reply == None {
                response(&mut write_buffer, &result);
            }
        } else if command.starts_with("prepend") {
            if size != 5 && size != 6 {
                response_wrong_number_of_arguments(&mut write_buffer, "prepend");
                continue;
            }

            let flags: u16 = iterator.next().unwrap().parse().unwrap();
            let exptime: isize = iterator.next().unwrap().parse().unwrap();
            let value_size_in_bytes: usize = iterator.next().unwrap().parse().unwrap();
            let no_reply = iterator.next();

            // TODO: validate size of payload with the value_size_in_bytes
            let mut value = String::new();
            read_buffer.read_line(&mut value).unwrap();

            let result = commands.prepend(CommandDto {
                key: key.to_string(),
                value,
                flags,
                exptime,
                value_size_in_bytes,
            });
            if no_reply == None {
                response(&mut write_buffer, &result);
            }
        } else {
            response(&mut write_buffer, "wrong command");
            continue;
        }
    }
}
