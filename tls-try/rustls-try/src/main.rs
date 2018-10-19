extern crate bytes;
extern crate futures;
extern crate tokio; // 0.1.8 // 0.1.23
extern crate tokio_codec;
extern crate tokio_rustls;
extern crate webpki;

pub mod codec;

use std::net::SocketAddr;
use std::io::{BufReader, Cursor};
use std::sync::Arc;

use futures::{future, stream};
use futures::{Future, Sink, Stream};
use tokio::codec::Decoder;
use tokio::net::TcpStream;
use tokio::runtime::current_thread;
use tokio_rustls::{rustls::internal::pemfile, rustls::ClientConfig, TlsConnector};

use codec::LineCodec;

fn lookup_ipv4(host: &str, port: u16) -> SocketAddr {
    use std::net::ToSocketAddrs;

    let addrs = (host, port).to_socket_addrs().unwrap();
    for addr in addrs {
        if let SocketAddr::V4(_) = addr {
            return addr;
        }
    }

    unreachable!("Cannot lookup address");
}

fn main() {
    let host = "localhost";
    let port = 12345;
    let addr = lookup_ipv4(host, port);
    let stream = TcpStream::connect(&addr);
    let mut config = ClientConfig::new();

    let ca = include_bytes!("../ca-chain.cert.pem").to_vec();
    let mut ca = BufReader::new(Cursor::new(ca));

    config.root_store.add_pem_file(&mut ca).unwrap();

    let clientcert = include_bytes!("../bike1.cert.pem").to_vec();
    let clientkey = include_bytes!("../bike1.key.pem").to_vec();
    let mut client_cert = BufReader::new(Cursor::new(clientcert));
    let mut client_keys = BufReader::new(Cursor::new(clientkey));
    let certs = pemfile::certs(&mut client_cert).unwrap();
    let keys =  pemfile::rsa_private_keys(&mut client_keys).unwrap();

    config.set_single_client_cert(certs, keys[0].clone());
    let config = TlsConnector::from(Arc::new(config));

    let connect = stream
        .and_then(move |stream| {
            let domain = webpki::DNSNameRef::try_from_ascii_str(host).unwrap();
            let c = config.connect(domain, stream);            
            c
        })
        .and_then(|stream| {
            let (nw_sink, nw_stream) = LineCodec.framed(stream).split();
            future::ok((nw_sink, nw_stream))
        })
        .and_then(|(sink, stream)| {
            stream.map(|incoming| {
                 println!("{:?}", incoming);
                 incoming
             }).forward(sink)
        });

    current_thread::block_on_all(connect).unwrap();
}
