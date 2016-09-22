#[macro_use]
extern crate tokio_core;
extern crate tokio_tls;
#[macro_use]
extern crate futures;

use std::net::ToSocketAddrs;
use futures::Future;
use tokio_core::reactor::Core;
use tokio_core::net::TcpStream;

pub mod networkstream;
pub mod error;

use networkstream::NetworkStream;

pub fn main() {
    let mut core = Core::new().unwrap();
    let addr = "localhost:12345".to_socket_addrs().unwrap().next().unwrap();

    let socket = TcpStream::connect(&addr, &core.handle());

    let future = socket.and_then(|stream| {
        NetworkStream::new(stream)
    });

    let data = core.run(future).expect("End EventLoop");
    // println!("{:?}", String::from_utf8_lossy(&data));
}
