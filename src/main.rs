#![cfg_attr(  debug_assertions,  allow(dead_code, unused_imports, unused_variables, unused_mut))]
#![allow(dead_code)]
#![allow(unused_variables)]
use std::io::Read;
use log::info;
use mini_redis::Error;
use std::result::Result;


#[tokio::main]
async fn main() -> Result<(), Error> {
    my_redis::init();
    info!("Hello, world!");

    Ok(())
}
