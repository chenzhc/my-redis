#![cfg_attr(  debug_assertions,  allow(dead_code, unused_imports, unused_variables, unused_mut))]
#![allow(dead_code)]
#![allow(unused_variables)]
use std::io::Read;

use log::info;
use mini_redis::{client, Result};


#[tokio::main]
async fn main() -> Result<()> {
    my_redis::init();
    info!("Hello, world!");

    let mut client = client::connect("127.0.0.1:6379").await?;

    client.set("Hello", "word".into()).await?;

    let result = client.get("Hello").await?;

    if result.is_some() {
        let bytes = result.unwrap();
        let rs_str = String::from_utf8(bytes.to_vec()).ok().unwrap();
        info!("got value from the server, result = {}", rs_str);
    }

    Ok(())
}
