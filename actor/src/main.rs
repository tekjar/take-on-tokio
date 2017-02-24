extern crate tokio_core;
#[macro_use]
extern crate quick_error;
extern crate futures;

use std::thread;
use std::time::Duration;

use futures::Sink;
use futures::Future;

pub mod error;
pub mod codec;
pub mod actor;

use actor::Actor;

fn main() {
    let mut a = Actor::start().unwrap();

    for i in 0..100 {
        let message = format!("{}. hello", i);
        a = a.send(message).wait().unwrap();
        thread::sleep(Duration::new(1, 0));
    }

    thread::sleep(Duration::new(100, 0));
    println!("Hello, world!");
}
