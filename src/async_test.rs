#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
#![allow(dead_code,unused_variables)]
use dotenv::dotenv;
use log::info;
use std::{future::Future, thread, time::Duration};


pub async fn get_number() -> u8 {
    info!("Running feture1");
    return 8;
}

pub async fn get_number2() -> u8 {
    info!("Running feture2");
    thread::sleep(Duration::from_millis(50));
    return 10;
}

pub async fn get_number3() -> u8 {
    info!("Running feture3");
    thread::sleep(Duration::from_millis(100));
    return 12;
}


#[cfg(test)]
mod tests {
    use std::{rc::Rc, time::Duration};

    use futures::{join, pin_mut, select, FutureExt};
    use log::info;
    use tokio::time;

    use crate::hello_redis;

    use super::*;

    #[tokio::test]
    async fn it_test_local_01() {
        crate::init();
        info!("test");
        let  num1 = get_number().fuse();
        let  num2 = get_number2().fuse();
        let  num3 = get_number3().fuse();

        pin_mut!(num1, num2, num3);

        let result = smol::block_on( async {
            // join!(num1, num2, num3)
            loop {
                select! {
                    x = num1 => info!("num1 is completed! {}", x),
                    x = num2 => info!("num2 is completed! {}", x),
                    x = num3 => info!("num3 is completed! {}", x),
                    complete => { 
                        info!("All futures have finished polling. Breaking out of loop");
                        break;
                    }
                }
            }
        });
        // let (rs1, rs2, rs3) = result;
        // info!("Get number: {}, {}, {}", rs1, rs2, rs3);
        

    }

}
