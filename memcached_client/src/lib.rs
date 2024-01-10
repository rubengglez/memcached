use std::error::Error;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpStream, ToSocketAddrs};

#[derive(Debug)]
pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub async fn connect<A: ToSocketAddrs>(addr: A) -> Result<Client, Box<dyn Error>> {
        // Connect to a peer
        let stream = TcpStream::connect(addr).await?;

        Ok(Client { stream })
    }

    pub async fn set(
        &mut self,
        key: &str,
        value: &str,
        exptime: usize,
    ) -> Result<&Client, Box<dyn Error>> {
        self.stream
            .write_all(format!("set {} 0 {} {}--{}--", key, exptime, value.len(), value).as_bytes())
            .await?;


        Ok(self)
    }
}
