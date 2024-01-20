use memcached_client::{self, Client};

async fn clean_data(client: &mut Client) {
    client
        .set("test".to_string(), "hola".to_string(), -1)
        .await
        .unwrap();
}

#[tokio::test]
async fn it_should_set_and_retrieve_the_value() {
    let mut client = memcached_client::Client::connect(vec!["127.0.0.1:1024"])
        .await
        .unwrap();

    let _ = client
        .set("test".to_string(), "hola".to_string(), 100)
        .await;

    let data = client.get("test".to_string()).await.unwrap();
    assert_eq!(data, "hola");

    clean_data(&mut client).await;
}

#[tokio::test]
async fn it_should_add_and_retrieve_the_value() {
    let mut client = memcached_client::Client::connect(vec!["127.0.0.1:1024"])
        .await
        .unwrap();

    let stored = client
        .add("test".to_string(), "hola".to_string(), 100)
        .await;

    assert!(stored.unwrap());

    let data = client.get("test".to_string()).await.unwrap();
    assert_eq!(data, "hola");

    clean_data(&mut client).await;
}
