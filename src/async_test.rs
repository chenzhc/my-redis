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

pub async fn test_something() {
    thread::sleep(Duration::from_millis(5000));
    info!("Hello from Tokio!");
}

pub struct F1Racer {
    name: String,
    completed_laps: u8,
    laps: u8,
    best_lap_time: u8,
    lap_times: Vec<u8>,
}

impl F1Racer {
    fn new() -> F1Racer {
        return F1Racer {
            name: "max Verstappen".to_string(), 
            completed_laps: 0, 
            laps: 5, 
            best_lap_time: 255, 
            lap_times: vec![87u8, 64, 126,95, 76],
        }
    }

    fn do_lap(&mut self) {
        info!("{} is doing a new lap...{}", self.name, self.best_lap_time);
        let lap_time = self.lap_times.pop();
        if lap_time.is_some() && lap_time.unwrap() < self.best_lap_time {
            self.best_lap_time = lap_time.unwrap();
        }

        self.completed_laps += 1;
    }
}

impl Future for F1Racer {
    type Output = u8;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        info!("Thread assigned is ID: {:?}", std::thread::current().id());
        if self.completed_laps < self.laps {
            self.get_mut().do_lap();
            cx.waker().wake_by_ref();
            return std::task::Poll::Pending;
        }

        info!("{} has completed all laps!", self.name);
        info!("Best lap time for {} was {}", self.name, self.best_lap_time);
        return std::task::Poll::Ready(self.best_lap_time);
    }
}


#[cfg(test)]
mod tests {
    use std::{rc::Rc, time::Duration};
    use futures::{join, pin_mut, select, FutureExt};
    use log::info;
    use tokio::time;
    use super::*;

    #[tokio::test(flavor="multi_thread")]
    async fn test_f1racer_test() {
        crate::init();

        let racer01 = F1Racer::new();
        let mut racer02 = F1Racer::new();

        racer02.name = "Sergio Perez".to_string();
        racer02.lap_times.pop();
        racer02.lap_times.push(57);

        
        let handle01 = tokio::task::spawn(racer01);
        let handle02 = tokio::task::spawn(racer02);
        
        loop {
            if handle01.is_finished() && handle02.is_finished() {
                info!("All racer have finished!");
                break;
            }
            thread::sleep(Duration::from_millis(300));
        }

    }

    #[tokio::test(flavor="multi_thread")]
    async fn it_something_test() {
        crate::init();
        info!("something test");
        test_something().await;

    }

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
