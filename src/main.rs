use bytes::BytesMut;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use commands::CommandDto;
use tracing_subscriber;
use types::Store;

use crate::{commands::Commands, config::MyConfig};

mod commands;
mod config;
mod errors;
mod item;
mod types;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .init();

    let config = match MyConfig::parse(std::env::args(), None) {
        Ok(c) => c,
        Err(err) => panic!("Invalid arguments {:?}", err),
    };

    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], config.port)))
        .await
        .unwrap();

    tracing::info!("Listening on port {}", config.port);

    let store = Arc::new(Mutex::new(HashMap::new()));

    loop {
        // The second item contains the IP and port of the new connection.
        let (mut socket, _) = listener.accept().await.unwrap();
        let store = store.clone();

        tokio::spawn(async move {
            handle_connection(&mut socket, store).await;
        });
    }
}

async fn response_wrong_number_of_arguments(writer: &mut TcpStream, command: &str) {
    response(
        writer,
        &format!("wrong number of arguments for {command}\r\n"),
    )
    .await;
}

async fn response(writer: &mut TcpStream, response: &str) {
    writer.write_all(response.as_bytes()).await.unwrap();
    writer.flush().await.unwrap();
}

async fn handle_connection(stream: &mut TcpStream, store: Store) {
    let mut commands = Commands::new(store);
    loop {
        let mut buf = BytesMut::with_capacity(1024);
        stream.read_buf(&mut buf).await.unwrap();
        let command = String::from_utf8(buf.to_vec()).unwrap();

        let mut iterator = command.split_whitespace();
        let size = iterator.clone().count();
        iterator.next();
        let key = iterator.next().unwrap();
        if command.starts_with("set") {
            if size != 5 && size != 6 {
                tracing::info!("Wrong number of arguments for {}", "set");
                response_wrong_number_of_arguments(stream, "set").await;
                continue;
            }

            let flags: u16 = iterator.next().unwrap().parse().unwrap();
            let exptime: isize = iterator.next().unwrap().parse().unwrap();
            let value_size_in_bytes: usize = iterator.next().unwrap().parse().unwrap();
            let no_reply = iterator.next();

            buf.clear();
            stream.read_buf(&mut buf).await.unwrap();
            // TODO: validate size of payload with the value_size_in_bytes
            let value = String::from_utf8(buf.to_vec()).unwrap();

            let result = commands.set(CommandDto {
                key: key.to_string(),
                value,
                flags,
                exptime,
                value_size_in_bytes,
            });
            tracing::info!("set result: {:?}", result);

            if no_reply == None {
                response(stream, &result).await;
            }
        } else if command.starts_with("get") {
            if size != 2 {
                response_wrong_number_of_arguments(stream, "get").await;
                continue;
            }

            let result = commands.get(key);
            tracing::info!("get result: {:?}", result);
            response(stream, &result).await;
        } else if command.starts_with("add") {
            if size != 5 && size != 6 {
                tracing::info!("Wrong number of arguments for {}", "add");
                response_wrong_number_of_arguments(stream, "add").await;
                continue;
            }

            let flags: u16 = iterator.next().unwrap().parse().unwrap();
            let exptime: isize = iterator.next().unwrap().parse().unwrap();
            let value_size_in_bytes: usize = iterator.next().unwrap().parse().unwrap();
            let no_reply = iterator.next();

            buf.clear();
            stream.read_buf(&mut buf).await.unwrap();
            // TODO: validate size of payload with the value_size_in_bytes
            let value = String::from_utf8(buf.to_vec()).unwrap();

            let result = commands.add(CommandDto {
                key: key.to_string(),
                value,
                flags,
                exptime,
                value_size_in_bytes,
            });
            tracing::info!("add result: {:?}", result);
            if no_reply == None {
                response(stream, &result).await;
            }
        } else if command.starts_with("replace") {
            if size != 5 && size != 6 {
                tracing::info!("Wrong number of arguments for {}", "replace");
                response_wrong_number_of_arguments(stream, "replace").await;
                continue;
            }

            let flags: u16 = iterator.next().unwrap().parse().unwrap();
            let exptime: isize = iterator.next().unwrap().parse().unwrap();
            let value_size_in_bytes: usize = iterator.next().unwrap().parse().unwrap();
            let no_reply = iterator.next();

            buf.clear();
            stream.read_buf(&mut buf).await.unwrap();
            // TODO: validate size of payload with the value_size_in_bytes
            let value = String::from_utf8(buf.to_vec()).unwrap();

            let result = commands.replace(CommandDto {
                key: key.to_string(),
                value,
                flags,
                exptime,
                value_size_in_bytes,
            });
            tracing::info!("replace result: {:?}", result);
            if no_reply == None {
                response(stream, &result).await;
            }
        } else if command.starts_with("append") {
            if size != 5 && size != 6 {
                tracing::info!("Wrong number of arguments for {}", "append");
                response_wrong_number_of_arguments(stream, "append").await;
                continue;
            }

            let flags: u16 = iterator.next().unwrap().parse().unwrap();
            let exptime: isize = iterator.next().unwrap().parse().unwrap();
            let value_size_in_bytes: usize = iterator.next().unwrap().parse().unwrap();
            let no_reply = iterator.next();

            buf.clear();
            stream.read_buf(&mut buf).await.unwrap();
            // TODO: validate size of payload with the value_size_in_bytes
            let value = String::from_utf8(buf.to_vec()).unwrap();

            let result = commands.append(CommandDto {
                key: key.to_string(),
                value,
                flags,
                exptime,
                value_size_in_bytes,
            });
            tracing::info!("append result: {:?}", result);
            if no_reply == None {
                response(stream, &result).await;
            }
        } else if command.starts_with("prepend") {
            if size != 5 && size != 6 {
                tracing::info!("Wrong number of arguments for {}", "prepend");
                response_wrong_number_of_arguments(stream, "prepend").await;
                continue;
            }

            let flags: u16 = iterator.next().unwrap().parse().unwrap();
            let exptime: isize = iterator.next().unwrap().parse().unwrap();
            let value_size_in_bytes: usize = iterator.next().unwrap().parse().unwrap();
            let no_reply = iterator.next();

            buf.clear();
            stream.read_buf(&mut buf).await.unwrap();
            // TODO: validate size of payload with the value_size_in_bytes
            let value = String::from_utf8(buf.to_vec()).unwrap();

            let result = commands.prepend(CommandDto {
                key: key.to_string(),
                value,
                flags,
                exptime,
                value_size_in_bytes,
            });
            tracing::info!("prepend result: {:?}", result);
            if no_reply == None {
                response(stream, &result).await;
            }
        } else {
            tracing::warn!("Wrong command: {}", command);
            response(stream, "wrong command").await;
            continue;
        }
    }
}
