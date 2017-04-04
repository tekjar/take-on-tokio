extern crate tokio_core;
#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate futures;
extern crate tokio_io;
extern crate bytes;
extern crate tokio_timer;

use std::thread;
use std::time::Duration;

use futures::{Future, Sink, Stream, Poll, StartSend, Async};

pub mod error;
pub mod codec;
pub mod connection;

use connection::Connection;
use tokio_core::reactor::Core;

fn main() {
    let mut mq_command = Connection::start("127.0.0.1:5555".to_string()).unwrap();

    for i in 0..100 {
        let message = format!("{}. hello", i);
        mq_command = mq_command.send(message).wait().unwrap();
        thread::sleep(Duration::new(1, 0));
    }


    
    println!("Hello, world!");
}
