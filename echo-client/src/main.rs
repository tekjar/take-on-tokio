extern crate tokio_core;
#[macro_use]
extern crate quick_error;
extern crate futures;

use std::thread;
use futures::Sink;
use futures::Future;
use futures::*;

pub mod error;
pub mod codec;
pub mod actor;

use actor::Actor;

fn main() {
    let mut a = Actor::start("127.0.0.1:5555".to_string()).unwrap();
    thread::sleep_ms(3000);
    for i in 0..100 {
        let message = format!("{}. hello", i);
        a = a.send(message).wait().unwrap();
        thread::sleep_ms(1000);
    }
    thread::sleep_ms(10000);
    println!("Hello, world!");
}
