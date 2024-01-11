use memcached_client;

#[tokio::test]
async fn it_should_reach_server() {
    let mut client = memcached_client::Client::connect("127.0.0.1:1024")
        .await
        .unwrap();

    let _ = client
        .set("test".to_string(), "hola".to_string(), 100)
        .await;

    let data = client.get("test".to_string()).await.unwrap();
    assert_eq!(data, "hola");
}
