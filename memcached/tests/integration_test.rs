use memcached_client;
use std::{borrow::Borrow, error::Error};

#[test]
fn it_adds_two() {
    assert_eq!(5, memcached_client::Client::lolo());
}

#[tokio::test]
async fn it_should_reach_server() {
    let client = memcached_client::Client::connect().await;
    assert!(client.is_ok());
}
