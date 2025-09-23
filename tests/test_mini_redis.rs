#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
#![allow(dead_code)]
#![allow(unused_variables)]

use log::info;
use mini_redis::{client, Result};
use my_redis::init;
use tokio::net::TcpListener;

#[tokio::test]
async fn it_redis_client_test() -> Result<()> {
    init();
    let mut client = client::connect("localhost:6378").await?;

    client.set("hello", "world".into()).await?;

    let result = client.get("hello").await?;

    info!("从服务器端获取到结果 = {:?}", result);
    Ok(())
}