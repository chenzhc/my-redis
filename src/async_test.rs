#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
#![allow(dead_code,unused_variables)]
use dotenv::dotenv;
use log::info;
use num_derive::FromPrimitive;
use core::fmt;
use std::{fmt::{Debug, Display}, future::Future, ops::{Add, Deref, DerefMut}, slice::from_raw_parts, str::from_utf8_unchecked, thread, time::Duration};


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

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn show_addr(ip: IpAddr) {
    info!("{:?}", ip);
}

trait IpAddr2 {
    fn display(&self);
}

pub struct V4(String);
impl IpAddr2 for V4 {
    fn display(&self) {
        info!("ipv4: {:?}", self.0);
    }
}

pub struct V6(String);
impl IpAddr2 for V6 {
    fn display(&self) {
        info!("ipv6: {:?}", self.0);
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Person {
        return Person { name, age };
    }
}

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }
}


pub fn longest2(_x: &str, _y: &str) -> String {
    String::from("really long string")

}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl Point {
    fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

struct Interface<'b, 'a: 'b> {
    manager: &'b mut Manager<'a>
}

impl<'b, 'a: 'b> Interface<'b,'a> {
    pub fn noop(self) {
        info!("interface consumed");
    }
}

struct Manager<'a> {
    text: &'a str,
}

struct List<'a> {
    manager: Manager<'a>,
}

impl<'a> List<'a> {
    pub fn get_interface<'b>(&'b mut self) -> Interface<'b, 'a> 
        where 'a: 'b     
    {
        Interface { manager: &mut self.manager }
    }
}

fn use_list(list: &List) {
    info!("{}", list.manager.text);
}

fn print_author(author: &'static str) {
    info!("{}", author);
}

fn print<T>(message: &T) 
    where T: Display + 'static
{
    info!("{}", message);
}

fn get_memory_location() -> (usize, usize) {
    let string = "Hello world!";
    let pointer = string.as_ptr() as usize;
    let length = string.len();

    return (pointer, length);
}

fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
    unsafe  {
        from_utf8_unchecked(from_raw_parts(pointer as *const u8, length))
    }
}

fn print_it<T>(input: &T) 
    where T: Debug + 'static
{
    info!("'static value passed in is: {:?}", input);
}

fn static_bound<T>(t: &T) 
    where T: Display + 'static
{
    info!("{}", t);
}

fn muuuu(intensity: u32) -> u32 {
    info!("muuuuuuu....");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn workout(intensity: u32, random_number: u32) {
    let action = || {
        info!("muuuuuuu....");
        thread::sleep(Duration::from_secs(2));
        intensity
    };
    if intensity < 25 {
        info!("今天活力满满, 先做 {} 个俯卧撑!", action());
        info!("旁边有妹子在看, {}", action());
    } else if random_number == 3 {
        info!("休息一下");
    } else {
        info!("{}", action());
    }
}

fn fn_one<F>(func: F) 
    where F: FnOnce(usize) -> bool + Copy,
{
    info!("{}", func(3));
    info!("{}", func(4));
}

fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
    f("hello")
}

fn exec2<'a, F: Fn(String) -> ()>(f: F) {
    f("world".to_string())
}

fn exec1<F: FnOnce()>(f: F) {
    f()
}

fn exec22<F: FnMut()>(mut f: F) {
    f()
}

fn exec3<F: Fn()>(f: F) {
    f()
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            count: 0
        }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn sum_for(x: &[f64]) -> f64 {
    let mut result: f64 = 0.0;
    for i in 0..x.len() {
        result += x[i];
    }
    result
}

fn sum_iter(x: &[f64]) -> f64 {
    x.iter().sum::<f64>()
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

struct Meters(u32);
impl fmt::Display for Meters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "目标地点距离你{}米", self.0)
    }
}

impl Add for Meters {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self(self.0 +  other.0)
    }
}

fn calculate_distance(d1: Meters, d2: Meters) -> Meters {
    d1 + d2 
}

#[derive(FromPrimitive)]
enum MyEnum {
    A = 1,
    B,
    C,
}

impl TryFrom<i32> for MyEnum {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == MyEnum::A as i32 => Ok(MyEnum::A),
            x if x == MyEnum::B as i32 => Ok(MyEnum::B),
            x if x == MyEnum::C as i32 => Ok(MyEnum::C),
            _ => Err(()),
        }
    }
}

fn foo(x: &str) -> String {
    let a = "Hello, ".to_string() + x;
    a
}

trait Draw {
    fn draw(&self);
}

struct Button {
    id: u32,
}
impl Draw for Button {
    fn draw(&self) {
        info!("这是屏幕上第{}号按钮", self.id);
    }
}

struct Select {
    id: u32,
}
impl Draw for Select {
    fn draw(&self) {
        info!("这个选择框贼难用{}", self.id);
    }
}

fn gen_static_str() -> &'static str {
    let mut s = String::new();
    s.push_str("hello, world");

    Box::leak(s.into_boxed_str())
}

#[derive(Debug)]
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// 解引用
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn display(s: &str) {
    info!("{}", s);
}

struct MyBox2<T> {
    v: T,
}
impl<T> MyBox2<T> {
    fn new(x: T) -> MyBox2<T> {
        MyBox2 { v: x}
    }
}

impl<T> Deref for MyBox2<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

impl<T> DerefMut for MyBox2<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.v
    }
}

fn display2(s: &mut String) {
    s.push_str("world");
    info!("{}", s);
}

struct HasDrop1;
struct HasDrop2;
impl Drop for HasDrop1 {
    fn drop(&mut self) {
        info!("Dropping HasDrop1!");
    }
}
impl Drop for HasDrop2 {
    fn drop(&mut self) {
        info!("Dropping HasDrop2!");
    }
}
struct HasTwoDrops {
    one: HasDrop1,
    two: HasDrop2,
}
impl Drop for HasTwoDrops {
    fn drop(&mut self) {
        info!("Dropping HasTwoDrops!");
    }
}

struct Foo;
impl Drop for Foo {
    fn drop(&mut self) {
        info!("Dropping Foo!");
    }
}

#[cfg(test)]
mod tests {
    use std::{backtrace, collections::HashMap, fs::File, rc::Rc, time::Duration};
    use futures::{join, pin_mut, select, FutureExt};
    use log::info;
    use rand::Rng;
    use tokio::time;
    use super::*;

    #[test]
    fn it_rc_test() {
        crate::init();
        
        let a = Rc::new(String::from("test ref counting"));
        info!("count after creating a = {}", Rc::strong_count(&a));
        let b = Rc::clone(&a);
        info!("count after creating b = {}", Rc::strong_count(&b));
        {
            let c = Rc::clone(&a);
            info!("count after creating c = {}", Rc::strong_count(&c));
        }
        info!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }

    #[test]
    fn it_my_box_test() {
        crate::init();
        let x = MyBox::new(5);
        info!("{:?}", x);
        info!("{}", *x);

        let s = String::from("hello world!");
        display(&s);

        let s = MyBox::new(String::from("Hello world"));
        display(&s);
        let s1: &str = &s;
        let s2: String = s.to_string();
        info!("{}", s1);
        info!("{}", s2);

        let  mut s = MyBox2::new(String::from("hello, "));
        display2(&mut s);

        let _x = HasTwoDrops {
            one: HasDrop1,
            two: HasDrop2,
        };
        let foo = Foo;
        drop(foo);        
        info!("Running!");

        let a = Rc::new(String::from("hello,world"));
        let b = Rc::clone(&a);
        info!("{}", Rc::strong_count(&a));
        info!("{}", Rc::strong_count(&b));




    }

    #[test]
    fn it_try_from_test() {
        crate::init();
        let x = MyEnum::C as i32;

        match x.try_into() {
            Ok(MyEnum::A) => info!("a"),
            Ok(MyEnum::B) => info!("b"),
            Ok(MyEnum::C) => info!("c"),
            Err(_) => info!("unknow number"),
        }

        let b = foo("world");
        info!("{}", b);

        let a = Box::new(3);
        info!("a = {:p}", a.as_ref());

        let arr = Box::new([0; 1000]);
        let arr1 = arr;
        info!("{}",arr1.len());

        let elemes: Vec<Box<dyn Draw>> = vec![
            Box::new(Button { id: 1}),
            Box::new(Select{id: 2})
        ];

        for e in elemes {
            e.draw();
        }

        let arr = vec![Box::new(1), Box::new(2)];
        let (first, second) = (&arr[0], &arr[1]);
        let sum = **first + **second;
        info!("sum = {}", sum);
        info!("first = {:?}", **first);

        let s = gen_static_str();
        info!("{}", s);

        let x = 5;
        let y = &x;

        info!("{}", *y);

        let x = Box::new(1);
        info!("{}", *x);


    }


    #[test]
    fn it_next_test() {
        crate::init();
        let mut counter = Counter::new();
        info!("{:?}", counter.next());
        info!("{:?}", counter.next());
        info!("{:?}", counter.next());
        info!("{:?}", counter.next());
        info!("{:?}", counter.next());
        info!("{:?}", counter.next());

        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a,b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        info!("{}", sum);

        let v = vec![1u64, 2,3,4,5,6];
        let val = v.iter()
            .enumerate()
            .filter(|&(idx, _)| idx % 2 == 0)
            .map(|(_, val)| val)
            .fold(0u64, |sum, acm| sum + acm);
        info!("{}", val);

        let a = i8::MAX;
        info!("{}", a);

        let a = 3.1 as i8;
        let b = 100_i8 as i32;
        let c = 'a' as u8;
        info!("{}, {}, {}", a, b, c);

        let mut values = [1,2];
        let p1 = values.as_mut_ptr();
        info!("{:p}", p1);
        let first_address = p1 as usize;
        info!("{}", first_address);
        let second_address = first_address + 4;
        info!("{}", second_address);
        let p2 = second_address as *mut i32;
        unsafe  {
            *p2 += 1;
        }

        info!("{}", values[1]);


        let a: u8 = 10;
        let b: u16 = 1500;
        // let b_: u8 = b.try_into().unwrap();

        let w = Wrapper(vec![
            String::from("hello"),
            String::from("world")
        ]);
        info!("w = {}", w);

        let d = calculate_distance(Meters(10), Meters(20));
        info!("{}", d);


        type Meters = u32;
        let x: u32 = 5;
        let y: Meters = 5;
        info!("x + y = {}", x + y);

        let x = 2;
        match num_traits::FromPrimitive::from_i32(x) {
            Some(MyEnum::A) => {
                info!("Got A");
            },
            Some(MyEnum::B) => info!("Got B"),
            Some(MyEnum::C) => info!("Got C"),
            None => info!("Couldn't convert {}", x),
        }

    }

    #[test]
    fn it_sum_test() {
        crate::init();

        let sum = |x: i32, y: i32| -> i32 {
            x + y
        };

        info!("{}", sum(1,2));
        let x = vec![1,2,3];
        fn_one(|z| { z == x.len()});

        let v = vec![1,2,3];
        let handle = thread::spawn(move || {
            info!("Here's a vector: {:?}", v);
        });
        handle.join().unwrap();

        let mut s =  String::new();
        let update_string = |str| s.push_str(str);

        exec(update_string);
        info!("{:?}", s);

        let s = "hello, ".to_string();
        let update_string = |str| info!("{},{}", s, str);
        exec2(update_string);
        info!("{:?}", s);

        let s = String::new();
        let update_string = || info!("{}", s);

        exec1(update_string);
        exec22(update_string);
        exec3(update_string);

        let arr = [1,2,3];
        for v in arr {
            info!("{}", v);
        }

        for i in 1..10 {
            info!("{}", i);
        }

        let arr = [1,2,3];
        for v in arr.into_iter() {
            info!("{}", v);
        }

        info!("{}", "=".repeat(20));
        let values = vec![1,2,3];
        for v in values.into_iter().into_iter().into_iter() {
            info!("{}", v);
        }

        let values = vec![1,2,3];
        for v in values.into_iter() {
            info!("{}", v);
        }

        let values = vec![1,2,3];
        let _values_iter = values.iter();

        info!("{:?}", values);

        let mut values = vec![1,2,3];
        let mut values_iter_mut = values.iter_mut();
        if let Some(v) = values_iter_mut.next() {
            *v = 0;
        }

        info!("{:?}", values);

        let names: [&'static str; 2] = ["sunface", "sunfei"];
        let ages = [18,19];
        let folks: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();
        info!("{:?}", folks);

    }

    #[test]
    fn it_static_input_test() {
        crate::init();
        let i = 5;
        print_it(&i);

        let r1;
        let r2;
        {
            static STATIC_EXAMPLE: i32 = 42;
            r1 = &STATIC_EXAMPLE;
            let x = "&'static str";
            r2 = x;
        }
        info!("&'static i32: {}", r1);
        info!("&'static str: {}", r2);

        let x = 1;
        let sum = |y| x+y;

        info!("{}", sum(2));

        let intensity = 10;
        let random_number = 7;
        workout(intensity, random_number);
      
    }

    #[test]
    fn it_null_test01() {
        crate::init();

        let mut s = String::from("hello");
        let r1 = &s;
        let r2 = &s;
        info!("{} and {}", r1, r2);
        let r3 = &mut s;
        info!("{}", r3);


        let mut  p = Point {x: 0, y:0};
        let r = &mut p;
        let rr: &Point = &*r;
        info!("{:?}", rr);

        r.move_to(10, 10);
        info!("{:?}", r);

        let mut list = List {
            manager: Manager { text: "hello" }
        };

        list.get_interface().noop();

        info!("Interface should be dropped here and the borrow released");

        use_list(&list);

        let mark_twain: &'static str = "Samuel Clemens";
        print_author(mark_twain);
        print(&mark_twain);

        let (pointer, length) = get_memory_location();
        let message = get_str_at_location(pointer, length);
        info!("The {} bytes at 0x{:X} stored: {}",
            length, pointer, message
        );

    }

    #[test]
    fn it_longest_test() {
        crate::init();
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        info!("The longest string is {}", result);

        let s = longest2("not", "important");
        info!("{}", s);

        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };

        info!("{:?}", i);

        let f = File::open("hello.txt");

        let f = match f {
            Ok(file) => file,
            Err(error) => {
                panic!("Problem opening the file: {:?}", error);
            },
        };
        
    }

    #[test]
    fn it_test_hash_map() {
        crate::init();
        let text = "hello world wonderful world";

        let mut map = HashMap::new();
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(10);
            *count += 1;
        }
        info!("{:?}", map);

    }

    #[test]
    fn it_person_test() {
        crate::init();
        let mut people = vec![
            Person::new("Zoe".to_string(), 25),
            Person::new("Al".to_string(), 60),
            Person::new("Al".to_string(), 30),
            Person::new("John".to_string(), 1),
            Person::new("John".to_string(), 25),
        ];
        // people.sort_unstable_by(|a, b| b.age.cmp(&a.age));
        people.sort_unstable();
        info!("{:?}", people);

        let mut my_gems = HashMap::new();
        my_gems.insert("红宝石", 1);
        my_gems.insert("蓝宝石", 2);
        my_gems.insert("河边检的误以为是宝石的破石头", 18);
        info!("{:?}", my_gems);

        let teams_list = vec![
            ("中国队".to_string(), 100),
            ("美国队".to_string(), 10),
            ("日本队".to_string(), 50),
        ];

        let mut teams_map = HashMap::new();
        for team in &teams_list {
            teams_map.insert(&team.0, team.1);
        }
        info!("{:?}", teams_map);

        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");
        let score: Option<&i32> = scores.get(&team_name);
        info!("{:?}", score);
        let score: i32 = scores.get(&team_name).copied().unwrap_or(0);
        info!("{}", score);

        for (key, value) in &scores {
            info!("{} : {}", key, value );
        }

        let mut scores = HashMap::new();
        scores.insert("Blue", 10);

        let old = scores.insert("Blue", 20);
        info!("old: {:?}", old);

        let new = scores.get("Blue");
        info!("new: {:?}",new);

        let v = scores.entry("Yellow").or_insert(5);
        info!("v: {}", *v);

        let v = scores.entry("Yellow").or_insert(50);
        info!("v: {:?}", *v);


    }

    #[test]
    fn it_show_ip_test() {
        crate::init();
        let v = vec![
            IpAddr::V4("127.0.0.1".to_string()),
            IpAddr::V6("::1".to_string()),
        ];

        for ip in v {
            show_addr(ip);
        }

        let v: Vec<Box<dyn IpAddr2>> = vec![
            Box::new(V4("127.0.0.1".to_string())),
            Box::new(V6("::1".to_string())),
        ];

        for ip in v {
            ip.display();
        }

        let v = vec![0; 3];
        info!("{:?}", v);
        let v_from = Vec::from([0,0,0]);
        info!("{:?}", v_from);

        let mut v = Vec::with_capacity(10);
        v.extend([1,2,3]);
        info!("Vector 长度是: {}, 容量是: {}", v.len(), v.capacity());

        v.reserve(100);
        info!("Vector (reserve) 长度是: {}, 容量是: {}",v.len(), v.capacity());

        v.shrink_to_fit();
        info!("Vector (shrink_to_fit) 长度是: {}, 容量是: {}", v.len(), v.capacity());

        let mut v = vec![1,2];
        info!("{}", v.is_empty());

        v.insert(2, 3);
        info!("{:?}", v);
        info!("{}", v.remove(1));
        info!("{}", v.pop().unwrap());
        info!("{:?}", v.pop());
        info!("{:?}", v.pop());
        v.clear();
        info!("{:?}", v);

        let mut v1 = [11,22].to_vec();
        v.append(&mut v1);
        info!("{:?}", v);
        v.truncate(1);
        info!("{:?}", v);
        v.retain(|x| *x > 10);
        info!("{:?}", v);

        let mut v = vec![11,22,33,44,55];
        let mut m: Vec<_> = v.drain(1..=3).collect();
        info!("{:?}", m);

        let v2 = m.split_off(1);
        info!("{:?}", v2);

        let v = vec![11,22,33,44,55];
        let slice = &v[1..=3];
        info!("{:?}", slice);

        let mut vec = vec![1,5,10,2,15];
        vec.sort_unstable();
        info!("{:?}", vec);

        let mut vec = vec![1.0, 5.6, 10.3, 2.0, 15f32];
        vec.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        info!("{:?}", vec);


    }

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

        let mut v  = Vec::new();
        v.push(1);
        info!("{:?}", v);

        let v = vec![1,2,3];
        info!("{:?}", v);

        let third = &v[2];
        info!("第三个元素是 {}", third);

        match v.get(2) {
            Some(third) => info!("第三个元素是: {}", third),
            None => info!("根本没有!"),
        }

        let does_not_exist = v.get(100);
        if does_not_exist.is_none() {
            info!("not exist: 100");
        }

        let v = vec![1,2,3];
        for i in &v {
            info!("{}", i);
        }

        let mut v = vec![1,2,3];
        for i in &mut v {
            *i += 10;
        }

        for i in &v {
            info!("{}", i);
        }

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
