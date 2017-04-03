extern crate tokio_core;
#[macro_use]
extern crate quick_error;
extern crate futures;
extern crate tokio_io;
extern crate bytes;

use std::thread;
use std::time::Duration;

use futures::Sink;
use futures::Future;

pub mod error;
pub mod codec;
pub mod connection;

use connection::Connection;

fn main() {
    let mut a = Connection::start("127.0.0.1:5555".to_string()).unwrap();
    
    thread::sleep(Duration::new(3, 0));
    for i in 0..100 {
        let message = format!("{}. hello", i);
        a = a.send(message).wait().unwrap();
        thread::sleep(Duration::new(1, 0));
    }
    
    println!("Hello, world!");
}
