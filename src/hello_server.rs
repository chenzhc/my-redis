#![cfg_attr( debug_assertions,  allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
#![allow(dead_code)]
#![allow(unused_variables)]
use dotenv::dotenv;
use log::info;
use mini_redis::{Connection, Frame};
use tokio::net::{TcpSocket, TcpStream};

pub async fn process(socket: TcpStream) {
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        info!("Got: {:?}", frame);

        let response = Frame::Error("unimplemenented".to_string());
        connection.write_frame(&response).await.unwrap();
        
    }
}

#[cfg(test)]
mod tests {

    use std::{rc::Rc};
    use tokio::{net::TcpListener, task::yield_now};
    use crate::hello_redis;
    use super::*;

    #[tokio::test]
    async fn it_spawn_yield_now_test() {
        crate::init();

        tokio::spawn(async {
            {
                let rc = Rc::new("hello");
                info!("{}", rc);
            }
            yield_now().await;
        });
        
    }

    #[tokio::test]
    async fn it_spawn_test01() {
        crate::init();

        let handle = tokio::spawn(async {
            "return value"
        });

        let out = handle.await.unwrap();
        info!("Got: {}", out);

    }

    #[tokio::test]
    async fn it_test_db01() {
        crate::init();
        info!("This is an info message from the test.");

        let listener = TcpListener::bind("127.0.0.1:6378").await.unwrap();

        loop {
            let (socket, _) = listener.accept().await.unwrap();
            tokio::spawn(async move {
                process(socket).await;
            });
        }

    }
}