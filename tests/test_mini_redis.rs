#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::time::{Duration, Instant};

use bytes::buf;
use futures::io;
use log::info;
use mini_redis::{client, Result};
use my_redis::{cmd_test::{handle_client, Command, Delay}, init};
use serde_json::map::Keys;
use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener, sync::oneshot};

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


#[tokio::test]
async fn it_async_read_file_test() -> anyhow::Result<()> {
    init();
    let mut f = File::open("foo.txt").await?;
    let mut buffer = [0; 10];

    let n = f.read(&mut buffer[..]).await?;

    info!("The bytes: {:?}", &buffer[..n]);
    Ok(())
}

#[tokio::test]
async fn it_async_file_test02() -> anyhow::Result<()>{
    init();
    let mut f = File::open("foo.txt").await?;
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer).await?;

    Ok(())
}

#[tokio::test]
async fn it_async_write_file_test01() -> anyhow::Result<()> {
    init();
    let mut file = File::create("foo.txt").await?;

    let n = file.write(b"some bytes").await?;

    info!("Wrote the first {} bytes of 'some bytes'.", n);
    Ok(())
}

#[tokio::test]
async fn it_async_write_file_test02() -> anyhow::Result<()> {
    init();
    let mut file = File::create("foo.txt").await?;

    file.write_all(b"some text bytes").await?;

    Ok(())
}

#[tokio::test]
async fn it_async_cp_file_test01() -> anyhow::Result<()> {
    init();
    let mut reader: &[u8] = b"hello";
    let mut file = File::create("foo.txt").await?;

    tokio::io::copy(&mut reader, &mut file).await?;

    Ok(())
}


#[tokio::test]
async fn it_tcp_copy_server_test() -> anyhow::Result<()> {
    init();
    let mut data = [0u8; 12];
    let listener = TcpListener::bind("0.0.0.0:6142").await?;
    info!("Server listening on 0.0.0.0:6142");

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let _ = handle_client(socket).await;
        });
    }
}


#[tokio::test]
async fn it_delay_test01() {
    init();
    let when = Instant::now() + Duration::from_millis(10);
    let future = Delay {
        when
    };

    let out = future.await;

    info!("{}", out);
}