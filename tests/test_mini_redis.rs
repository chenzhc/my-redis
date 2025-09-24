#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
#![allow(dead_code)]
#![allow(unused_variables)]

use log::info;
use mini_redis::{client, Result};
use my_redis::{cmd_test::Command, init};
use serde_json::map::Keys;
use tokio::{net::TcpListener, sync::oneshot};

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

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
                Get { key,resp } => {
                    let res = client.get(&key).await;
                    let _ = resp.send(res);
                },
                Set { key, val, resp } => {
                    let res = client.set(&key, val).await;
                    let _ = resp.send(res);
                }
            }
        }
    });

    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = my_redis::cmd_test::Command::Get {
            key: "hello".to_string(),
            resp: resp_tx,
        };
        tx.send(cmd).await.unwrap();
        let res = resp_rx.await;
        info!("Got = {:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = my_redis::cmd_test::Command::Set { 
            key: "foo".to_string(), 
            val: "bar".into(),
            resp: resp_tx,
        };

        tx2.send(cmd).await.unwrap();

        let res = resp_rx.await;
        info!("Got = {:?}", res);
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
    
}

