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
use tokio::net::{TcpListener, TcpStream};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::tcp::WriteHalf,
};

use commands::CommandDto;
use types::Store;

use crate::{commands::Commands, config::MyConfig, protocol_parser::CommandParserInputDataBuilder};

pub struct Server {}

impl Default for Server {
    fn default() -> Self {
        Server::new()
    }
}

impl Server {
    pub fn new() -> Server {
        Server {}
    }

    pub async fn run(&self) {
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
            let (socket, _) = listener.accept().await.unwrap();
            tracing::info!("new connection established");
            let store = store.clone();
            let builder = input_builder.clone();

            tokio::spawn(async move {
                handle_connection(socket, store, builder).await;
            });
        }
    }
}

async fn response<'a>(writer: &mut WriteHalf<'a>, response: &str) {
    writer.write_all(response.as_bytes()).await.unwrap();
    writer.flush().await.unwrap();
}

async fn handle_connection(
    mut stream: TcpStream,
    store: Store,
    builder: Arc<CommandParserInputDataBuilder>,
) {
    let mut commands = Commands::new(store);
    let (mut rd, mut wr) = stream.split();
    loop {
        let mut buf = BytesMut::with_capacity(1024);
        rd.read_buf(&mut buf).await.unwrap();
        let command = String::from_utf8(buf.to_vec()).unwrap();

        let input_data = builder.build(command);
        if input_data.is_err() {
            // TODO: deal with different errors and return different messages
            let test = input_data.err();
            tracing::warn!(target: "Wrong command", warning = "Wrong command", "~~~ {:?}",  test);
            response(&mut wr, "wrong command").await;
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
                response(&mut wr, &result).await;
            }
        } else if input_data.command == "get" {
            let result = commands.get(input_data.key.as_str());
            tracing::info!("get result: {:?}", result);
            response(&mut wr, &result).await;
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
                response(&mut wr, &result).await;
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
                response(&mut wr, &result).await;
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
                response(&mut wr, &result).await;
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
                response(&mut wr, &result).await;
            }
        }
    }
}
