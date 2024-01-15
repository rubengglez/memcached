use memcached::Server;

#[tokio::main]
async fn main() {
    let mut server = Server::new();
    server.run(1024).await;
}
