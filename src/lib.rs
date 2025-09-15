#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
#![allow(dead_code)]
#![allow(unused_variables)]
use dotenv::dotenv;
use log::info;

pub mod hello_redis;

pub mod hello_server;

pub mod async_test;

pub mod cmd_test;

pub mod csv_test;

// init log config
pub fn init() {
    dotenv().ok();
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    // info!("INFO");
    // let _ = env_logger::builder()
    //     .target(env_logger::Target::Stdout)
    //     .filter_level(log::LevelFilter::Trace)
    //     .is_test(true)
    //     .try_init();
}


pub async fn say_world() {
    info!("world");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_test_db01() {
        crate::init();
        info!("This is an info message from the test.");

        let op = say_world();

        info!("hello");

        op.await;

    }
}