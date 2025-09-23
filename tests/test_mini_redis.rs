#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
#![allow(dead_code)]
#![allow(unused_variables)]

use log::info;
use mini_redis::{client, Command, Result};
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

#[tokio::test] 
async fn it_reids_client_test02() {
    init();
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);
    let tx2 = tx.clone();

    let manager = tokio::spawn(async move {
        let mut client = client::connect("localhost:6378").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            use my_redis::cmd_test::Command::*;

            match cmd {
                Get { key } => {
                    let _ = client.get(&key).await;
                },
                Set { key, val } => {
                    let _ = client.set(&key, val).await;
                }
            }
        }
    });

    let t1 = tokio::spawn(async move {
        let cmd = my_redis::cmd_test::Command::Get {
            key: "hello".to_string(),
        };

        tx.send(cmd).await.unwrap();
    });

    let t2 = tokio::spawn(async move {
        let cmd = my_redis::cmd_test::Command::Set { 
            key: "foo".to_string(), 
            val: "bar".into(),
        };

        tx2.send(cmd).await.unwrap();
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
    
}