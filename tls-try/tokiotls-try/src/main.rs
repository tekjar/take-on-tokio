extern crate bytes;
extern crate futures;
extern crate tokio; // 0.1.8 // 0.1.23
extern crate tokio_codec;
extern crate webpki;
extern crate tokio_tls;
extern crate native_tls;

pub mod codec;

use std::net::SocketAddr;
use std::fs;
use std::io::BufReader;
use std::net::ToSocketAddrs;
use std::sync::Arc;

use futures::{future, stream};
use futures::{Future, Sink, Stream};
use tokio::codec::Decoder;
use tokio::net::TcpStream;
use tokio::runtime::current_thread;

use codec::LineCodec;
use std::fs::File;
use std::io::Read;
use native_tls::Identity;
use native_tls::TlsConnectorBuilder;
use tokio_tls::TlsConnector;
use native_tls::Certificate;
use tokio::io;
use native_tls::TlsStream;
use tokio::io::AsyncRead;
use tokio::io::AsyncWrite;
use futures::Async;
use std::io::Error;

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
    let iotcore = "roots.pem";
    let capath = "ca-chain.cert.pem";
    let host = "localhost";
    let port = 12345;
    let addr = lookup_ipv4(host, port);

    let ca = Certificate::from_pem(include_bytes!("../ca-chain.cert.pem")).unwrap();
    let identity = Identity::from_pkcs12(include_bytes!("../identity.pfx"), "test").unwrap();
    let tls_connector = TlsConnector::builder()
                                            .identity(identity)
                                            .add_root_certificate(ca)
                                            .build()
                                            .unwrap();



    let stream = TcpStream::connect(&addr);

    let connect = stream
        .and_then(move |stream| {
            tls_connector.connect(host, stream).map_err(|e| {
                io::Error::new(io::ErrorKind::Other, e)
            })
        })
        .and_then(|stream| {
            let (nw_sink, nw_stream) = LineCodec.framed(stream).split();
            future::ok((nw_sink, nw_stream))
        });

    let (nw_sink, nw_stream) = current_thread::block_on_all(connect).unwrap();

    let forwarder = nw_stream.map(|incoming| {
        println!("{:?}", incoming);
        incoming
    }).forward(nw_sink);

    current_thread::block_on_all(forwarder).unwrap();
}


enum Network(Tls)

impl AsyncRead for TlsStream<TcpStream>{}
impl AsyncWrite for TlsStream<TcpStream>{
    fn shutdown(&mut self) -> Result<Async<()>, Error> {
        unimplemented!()
    }
}