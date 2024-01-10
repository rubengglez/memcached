use std::time::Duration;

use memcached_client;
use tokio::time::sleep;

#[tokio::test]
async fn it_should_reach_server() {
    let mut client = memcached_client::Client::connect("127.0.0.1:1024")
        .await
        .unwrap();

    let _ = client.set("test", "hola", 100).await;

    sleep(Duration::from_millis(10000)).await;
}
