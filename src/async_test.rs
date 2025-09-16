#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
#![allow(dead_code,unused_variables)]
use dotenv::dotenv;
use log::info;
use core::fmt;
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

pub fn takes_ownership(some_thing: String) {
    info!("{:p}, {}", &some_thing, some_thing);
}

pub fn makes_copy(some_thing: i32) {
    info!("{}", some_thing);
}

pub fn calculate_length(s: &String) -> usize {
    return s.len();
}

pub fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

pub trait Pilot {
    fn fly(&self);
} 

pub trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        info!("This is your caption speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        info!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        info!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Sport")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use std::{backtrace, rc::Rc, time::Duration};
    use futures::{join, pin_mut, select, FutureExt};
    use log::info;
    use tokio::time;
    use super::*;

    #[test]
    fn it_human_test() {
        crate::init();
        let person = Human;
        Pilot::fly(&person);
        Wizard::fly(&person);
        person.fly();

        info!("A baby dog is called a {}", Dog::baby_name());
        info!("A baby dog is called a {}", <Dog as Animal>::baby_name());
        info!("{}", "=".repeat(10));

        let p1 = Point { x: 10, y: 20 };
        info!("{}", p1.to_string());

    }

    #[test]
    fn it_test03() {
        crate::init();
        let s = String::from("hello");
        info!("{:p}", &s);
        takes_ownership(s);

        let x = 5;
        makes_copy(x);
        info!("{}", x);

        let x = 5;
        let y = &x;
        info!("{}", x);
        info!("{}", *y);

        let s1 = String::from("hello");
        let len = calculate_length(&s1);
        info!("The length of '{}' is {}.", s1, len);
        
        let mut s = String::from("hello");
        change(&mut s);
        info!("{}", s);

        let string_append = String::from("hello ");
        let string_rust = String::from("rust");
        let result = string_append + &string_rust;
        let mut result = result + "!";
        result += "!!!";
        info!("{}", result);

        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        let s = s1 + "-" + &s2 + "-" + &s3;
        info!("{}", s);

        let s1 = "hello";
        let s2 = String::from("rust");
        let s = format!("{} {}!", s1, s2);
        info!("{}", s);

        for c in "中国人".chars() {
            info!("{}", c);
        }

    }

    #[test]
    fn it_test02() {
        crate::init();
        let x = (-42.0_f32).sqrt();
        if x.is_nan() {
            info!("未定义的数字行为");
        }

        for i in 1..5 {
            info!("{}", i);
        }

        let x = '中';
        info!("点用内存大小: {}", size_of_val(&x));

        let s1 = String::from("hello");
        let s2 = s1.clone();
        info!("s1 = {}, s2 = {}",s1, s2 );
        info!("s1 = {}", size_of_val(&s1));
        info!("{}", s1.capacity());
        info!("{}", s1.len());

    }

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
