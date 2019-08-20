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

pub mod connection;
mod codec;
mod error;
use connection::{Connection, test_future};

fn main() {
    // let mut conn = Connection::start("127.0.0.1:5555".to_owned()).unwrap();
    // thread::sleep_ms(2000);
    // conn.clone().send("skdf".to_string()).wait();
    // thread::sleep_ms(4000);
    // conn.clone().send("ssdfd".to_string()).wait();
    // thread::sleep_ms(5000);
    // conn.clone().send("ssd2547".to_string()).wait();
    // // println!("Hello, world!");
    // loop {}
    test_future();
}
