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
    rd: Arc<Mutex<ReadHalf<TcpStream>>>,
    wr: Arc<Mutex<WriteHalf<TcpStream>>>,
}

impl Client {
    pub async fn connect<A: ToSocketAddrs>(addr: A) -> Result<Client, Box<dyn Error>> {
        let stream = TcpStream::connect(addr).await?;

        let (rd, wr) = io::split(stream);

        Ok(Client {
            rd: Arc::new(Mutex::new(rd)),
            wr: Arc::new(Mutex::new(wr)),
        })
    }

    pub async fn set(
        &mut self,
        key: String,
        value: String,
        exptime: usize,
    ) -> Result<&Client, Box<dyn Error>> {
        let wr = self.wr.clone();

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

        self.rd.lock().await.read_buf(&mut buf).await?;

        println!("GOT {:?}", String::from_utf8(buf.to_vec()));

        Ok(self)
    }

    pub async fn get(&mut self, key: String) -> Result<String, Box<dyn Error>> {
        let wr = self.wr.clone();

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

        self.rd.lock().await.read_buf(&mut buf).await?;

        let data = String::from_utf8(buf.to_vec()).unwrap();

        println!("GOT {:?}", data);

        parse_response(data)
    }
}
