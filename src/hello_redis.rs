#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
#![allow(dead_code,unused_variables)]
use dotenv::dotenv;
use mini_redis::{client, Result};
use log::info;

pub async fn client_set_hello() -> Result<()> {
    
    let mut client = client::connect("127.0.0.1:6378").await?;

    client.set("Hello", "word".into()).await?;

    let result = client.get("Hello").await?;
    info!("{:?}", result);

    if result.is_some() {
        let bytes = result.unwrap();
        let rs_str = String::from_utf8(bytes.to_vec()).ok().unwrap();
        info!("got value from the server, result = {}", rs_str);
    } else {
        info!("{:?}", result);
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use std::{rc::Rc, time::Duration};

    use tokio::time;

    use crate::hello_redis;

    use super::*;

    #[tokio::test]
    async fn it_test_local_01() {
        crate::init();

        let nonsend_data = Rc::new("world");
        let local = tokio::task::LocalSet::new();

        let nosend_data2 = nonsend_data.clone();
        local.spawn_local(async move {
            info!("hello {}", nosend_data2);
        });

        local.spawn_local(async move {
            time::sleep(Duration::from_millis(100)).await;
            info!("goodbay {}", nonsend_data);
        });

        local.await;


    }

    #[tokio::test]
    async fn it_test_db01() {
        crate::init();
        info!("This is an info message from the test.");

        let result = client_set_hello().await;
        info!("{:?}", result);
        
    }
}