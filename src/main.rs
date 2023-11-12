use std::net::{SocketAddr, TcpListener};

use crate::config::Config;

mod config;


fn main() {
    let config = match Config::parse(std::env::args()) {
        Ok(c) => c,
        Err(_) => panic!("Invalid arguments"),
    };

    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], config.port))).unwrap();

    for stream in listener.incoming() {
        let _ = stream.unwrap();

        println!("Connection established!");
    }
}
