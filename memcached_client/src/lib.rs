mod protocol_parser;

use crate::protocol_parser::*;
use bytes::BytesMut;
use std::error::Error;
use std::sync::Arc;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct Client {
    connections: Vec<Connection>,
}

#[derive(Debug)]
struct Connection {
    rd: Arc<Mutex<ReadHalf<TcpStream>>>,
    wr: Arc<Mutex<WriteHalf<TcpStream>>>,
}

impl Client {
    async fn create_connection<A: ToSocketAddrs>(addr: A) -> Result<Connection, Box<dyn Error>> {
        let stream = TcpStream::connect(addr).await?;

        let (rd, wr) = io::split(stream);

        Ok(Connection {
            rd: Arc::new(Mutex::new(rd)),
            wr: Arc::new(Mutex::new(wr)),
        })
    }

    pub async fn connect<A: ToSocketAddrs>(addresses: Vec<A>) -> Result<Client, Box<dyn Error>> {
        let mut connections: Vec<Connection> = vec![];
        for address in addresses {
            let conn = Client::create_connection(address).await?;
            connections.push(conn);
        }

        Ok(Client { connections })
    }

    pub async fn set(
        &mut self,
        key: String,
        value: String,
        exptime: isize,
    ) -> Result<&Client, Box<dyn Error>> {
        let wr = self.connections.first().unwrap().wr.clone();

        tokio::spawn(async move {
            wr.lock()
                .await
                .write_all(
                    format!("set {} 0 {} {}--{}--", key, exptime, value.len(), value).as_bytes(),
                )
                .await?;

            // Sometimes, the rust type inferencer needs
            // a little help
            Ok::<_, io::Error>(())
        });

        let mut buf = BytesMut::with_capacity(1024);

        self.connections.first().unwrap().rd.lock().await.read_buf(&mut buf).await?;

        println!("GOT {:?}", String::from_utf8(buf.to_vec()));

        Ok(self)
    }

    pub async fn get(&mut self, key: String) -> Result<String, Box<dyn Error>> {
        let wr = self.connections.first().unwrap().wr.clone();

        tokio::spawn(async move {
            wr.lock()
                .await
                .write_all(format!("get {}", key).as_bytes())
                .await?;

            // Sometimes, the rust type inferencer needs
            // a little help
            Ok::<_, io::Error>(())
        });

        let mut buf = BytesMut::with_capacity(1024);

        self.connections.first().unwrap().rd.lock().await.read_buf(&mut buf).await?;

        let data = String::from_utf8(buf.to_vec()).unwrap();

        println!("GOT {:?}", data);

        parse_response(data)
    }

    // Add documentation about the bool response
    pub async fn add(
        &mut self,
        key: String,
        value: String,
        exptime: isize,
    ) -> Result<bool, Box<dyn Error>> {
        let wr = self.connections.first().unwrap().wr.clone();

        tokio::spawn(async move {
            wr.lock()
                .await
                .write_all(
                    format!("add {} 0 {} {}--{}--", key, exptime, value.len(), value).as_bytes(),
                )
                .await?;

            // Sometimes, the rust type inferencer needs
            // a little help
            Ok::<_, io::Error>(())
        });

        let mut buf = BytesMut::with_capacity(1024);

        self.connections.first().unwrap().rd.lock().await.read_buf(&mut buf).await?;

        let response = String::from_utf8(buf.to_vec());

        println!("GOT {:?}", response);

        match response {
            Ok(resp) => {
                if resp.trim() == "NOT_STORED" {
                    Ok(false)
                } else {
                    Ok(true)
                }
            }
            Err(err) => Err(err.into()),
        }
    }
}
