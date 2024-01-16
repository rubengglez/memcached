use memcached::Server;

#[tokio::main]
async fn main() {
    let server = Server::default();
    server.run().await;
}
