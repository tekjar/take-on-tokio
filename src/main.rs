extern crate tokio_core;
extern crate tokio_tls;
extern crate futures;
extern crate openssl;

use std::net::ToSocketAddrs;

use futures::Future;
use tokio_core::reactor::Core;
use tokio_core::net::TcpStream;
use tokio_tls::ClientContext;
use tokio_tls::backend::secure_transport::ClientContextExt;
use openssl::x509::X509FileType;
use openssl::ssl::SslContext;

pub fn main() {
    let mut core = Core::new().unwrap();
    let addr = "veh-test-mqtt-broker.atherengineering.in:8883".to_socket_addrs().unwrap().next().unwrap();

    let socket = TcpStream::connect(&addr, &core.handle());

    let tls_handshake = socket.and_then(|socket| {
        // let cx = ClientContext::new().unwrap();
        // cx.anchor_certificates();
        // cx.handshake("veh-test-mqtt-broker.atherengineering.in:8883", socket)
    });

    let request = tls_handshake.and_then(|socket| {
        println!("Connection Successful");
    });

    let response = request.and_then(|(socket, _)| {
        tokio_core::io::read_to_end(socket, Vec::new())
    });

    let (_, data) = core.run(response).unwrap();
    println!("{}", String::from_utf8_lossy(&data));
}
