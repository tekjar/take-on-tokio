extern crate tokio_core;
#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate futures;
extern crate tokio_io;
extern crate tokio_timer;
extern crate bytes;

use std::thread;
use std::error::Error;
use std::time::Duration;

use futures::{Future, Sink};

pub mod error;
pub mod codec;
pub mod connection;

use connection::Connection;

fn main() {
    let mut a = Connection::start("127.0.0.1:5555".to_string()).unwrap();

    for i in 0..100 {
        let message = format!("{}. hello", i);
        a = match a.send(message).wait() {
            Ok(a) => a,
            Err(e) => {
                println!("tx error: {:?}", e.description());
                panic!()
            }
        };
        thread::sleep(Duration::new(1, 0));
    }

    println!("Hello, world!");
}
