mod protocol_parser;

use crate::protocol_parser::*;
use bytes::BytesMut;
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::sync::Mutex;

type HashId = u64;
type MutexWriteHalfTcpStream = Arc<Mutex<WriteHalf<TcpStream>>>;
type MutexReadHalfTcpStream = Arc<Mutex<ReadHalf<TcpStream>>>;

fn calculate_hash<T: Hash>(t: &T) -> HashId {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[derive(Debug)]
pub struct Client {
    connections: Vec<Connection>,
}

#[derive(Debug)]
struct Connection {
    rd: MutexReadHalfTcpStream,
    wr: MutexWriteHalfTcpStream,
}

impl Client {
    pub async fn connect<A: ToSocketAddrs>(addresses: Vec<A>) -> Result<Client, Box<dyn Error>> {
        let mut connections = vec![];
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
        let (wr, rd) = self.get_write_and_read_conn(&key);

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

        rd.lock().await.read_buf(&mut buf).await?;

        println!("GOT {:?}", String::from_utf8(buf.to_vec()));

        Ok(self)
    }

    pub async fn get(&mut self, key: String) -> Result<String, Box<dyn Error>> {
        let (wr, rd) = self.get_write_and_read_conn(&key);

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

        rd.lock().await.read_buf(&mut buf).await?;

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
        let (wr, rd) = self.get_write_and_read_conn(&key);

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

        rd.lock().await.read_buf(&mut buf).await?;

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

    async fn create_connection<A: ToSocketAddrs>(addr: A) -> Result<Connection, Box<dyn Error>> {
        let stream = TcpStream::connect(addr).await?;

        let (rd, wr) = io::split(stream);

        Ok(Connection {
            rd: Arc::new(Mutex::new(rd)),
            wr: Arc::new(Mutex::new(wr)),
        })
    }

    fn select_connection(&self, key: &str) -> &Connection {
        let hash = calculate_hash(&key.to_string());
        let index = hash as usize % self.connections.len();
        return &self.connections[index];
    }

    fn get_write_and_read_conn(
        &self,
        key: &str,
    ) -> (MutexWriteHalfTcpStream, MutexReadHalfTcpStream) {
        let connection = self.select_connection(key);
        let wr = connection.wr.clone();
        let rd = connection.rd.clone();

        return (wr, rd);
    }
}
