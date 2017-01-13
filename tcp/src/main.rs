#[macro_use]
extern crate tokio_core;
extern crate tokio_tls;
#[macro_use]
extern crate futures;

use std::io::BufReader;
use std::thread;
use std::time::Duration;

use futures::Future;
use tokio_core::reactor::Core;
use tokio_core::net::TcpStream;
use tokio_core::io::{self, Io};

pub fn main() {
    let mut l = Core::new().unwrap();
    let handle = l.handle();

    let local = "127.0.0.1:12345".parse().unwrap();

    let f1 = TcpStream::connect(&local, &handle).and_then(move |socket| {
        let (reader, mut writer) = socket.split();
        let reader = BufReader::new(reader);
        loop {
            println!("sending ...");
            tokio_core::io::write_all(&mut writer, b"Hello!\n");
            thread::sleep(Duration::new(1, 0));
        }
        Ok(())
    });
    l.run(f1).unwrap();

}
