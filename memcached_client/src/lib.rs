use std::{error::Error};
use tokio::net::{TcpStream};
use tokio::io::AsyncWriteExt;

#[derive(Debug)]
pub struct Client {
    endpoint: String,
    // endpoint: SocketAddr,
}

impl Client {
    pub async fn connect() -> Result<Client, Box<dyn Error>> {
        // Connect to a peer
       let mut stream = TcpStream::connect("0.0.0.0:1234").await?;

        // Write some data.
        stream.write_all(b"hello world!").await?;

        Ok(Client {
            endpoint: "a".to_owned(),
        })
    }

    pub fn lolo() -> usize {
        return 5;
    }
}
