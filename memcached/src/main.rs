mod commands;
mod config;
mod errors;
mod item;
mod protocol_parser;
mod types;

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

use crate::{commands::Commands, config::MyConfig, protocol_parser::CommandParserInputDataBuilder};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .init();

    let config = match MyConfig::parse(std::env::args(), None) {
        Ok(c) => c,
        Err(err) => panic!("Invalid arguments {:?}", err),
    };
    let input_builder = Arc::new(CommandParserInputDataBuilder::new(config.protocol));
    let store = Arc::new(Mutex::new(HashMap::new()));

    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], config.port)))
        .await
        .unwrap();

    tracing::info!("Listening on port {}", config.port);

    loop {
        // The second item contains the IP and port of the new connection.
        let (mut socket, _) = listener.accept().await.unwrap();
        tracing::info!("new connection established");
        let store = store.clone();
        let builder = input_builder.clone();

        tokio::spawn(async move {
            handle_connection(&mut socket, store, builder).await;
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

async fn handle_connection(
    stream: &mut TcpStream,
    store: Store,
    builder: Arc<CommandParserInputDataBuilder>,
) {
    let mut commands = Commands::new(store);
    loop {
        let mut buf = BytesMut::with_capacity(1024);
        // TODO: split stream into reader and writer
        stream.read_buf(&mut buf).await.unwrap();
        let command = String::from_utf8(buf.to_vec()).unwrap();

        let input_data = builder.build(command);
        if input_data.is_err() {
            // TODO: deal with different errors and return different messages
            tracing::warn!("Wrong command");
            response(stream, "wrong command").await;
            continue;
        }
        let input_data = input_data.unwrap();
        if input_data.command == "set" {
            let result = commands.set(CommandDto {
                key: input_data.key,
                value: input_data.value.unwrap(),
                flags: input_data.flags.unwrap(),
                exptime: input_data.exptime.unwrap(),
                value_size_in_bytes: input_data.value_size_bytes.unwrap(),
            });
            tracing::info!("set result: {:?}", result);

            if input_data.no_reply == Some(false) {
                response(stream, &result).await;
            }
        } else if input_data.command == "get" {
            let result = commands.get(input_data.key.as_str());
            tracing::info!("get result: {:?}", result);
            response(stream, &result).await;
        } else if input_data.command == "add" {
            let result = commands.add(CommandDto {
                key: input_data.key,
                value: input_data.value.unwrap(),
                flags: input_data.flags.unwrap(),
                exptime: input_data.exptime.unwrap(),
                value_size_in_bytes: input_data.value_size_bytes.unwrap(),
            });
            tracing::info!("add result: {:?}", result);
            if input_data.no_reply == Some(false) {
                response(stream, &result).await;
            }
        } else if input_data.command == "replace" {
            let result = commands.replace(CommandDto {
                key: input_data.key,
                value: input_data.value.unwrap(),
                flags: input_data.flags.unwrap(),
                exptime: input_data.exptime.unwrap(),
                value_size_in_bytes: input_data.value_size_bytes.unwrap(),
            });
            tracing::info!("replace result: {:?}", result);
            if input_data.no_reply == Some(false) {
                response(stream, &result).await;
            }
        } else if input_data.command == "append" {
            let result = commands.append(CommandDto {
                key: input_data.key,
                value: input_data.value.unwrap(),
                flags: input_data.flags.unwrap(),
                exptime: input_data.exptime.unwrap(),
                value_size_in_bytes: input_data.value_size_bytes.unwrap(),
            });
            tracing::info!("append result: {:?}", result);
            if input_data.no_reply == Some(false) {
                response(stream, &result).await;
            }
        } else if input_data.command == "prepend" {
            let result = commands.prepend(CommandDto {
                key: input_data.key,
                value: input_data.value.unwrap(),
                flags: input_data.flags.unwrap(),
                exptime: input_data.exptime.unwrap(),
                value_size_in_bytes: input_data.value_size_bytes.unwrap(),
            });
            tracing::info!("prepend result: {:?}", result);
            if input_data.no_reply == Some(false) {
                response(stream, &result).await;
            }
        }
    }
}
