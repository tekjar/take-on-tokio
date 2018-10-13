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
use native_tls::{Identity, Certificate};
use tokio_tls::{TlsConnector, TlsStream};
use tokio::io::{self, AsyncRead, AsyncWrite};
use futures::Async;
use std::io::{Read, Write, Error};

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

    let ca = include_bytes!("../roots.pem");
    let ca = Certificate::from_pem(ca).unwrap();
    let identity = Identity::from_pkcs12(include_bytes!("../identity.pfx"), "test").unwrap();
    let tls_connector = native_tls::TlsConnector::builder()
                                            .identity(identity)
                                            .add_root_certificate(ca)
                                            .build()
                                            .unwrap();



    let stream = TcpStream::connect(&addr);

    let connect = stream
        .and_then(move |stream| {
            let tls_connector: TlsConnector = tls_connector.into();
            tls_connector.connect(host, stream).map_err(|e| {
                io::Error::new(io::ErrorKind::Other, e)
            })
        })
        .and_then(|stream| {
            let stream = NetworkStream::new_tls_stream(stream);
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


pub enum NetworkStream {
    Tcp(TcpStream),
    Tls(TlsStream<TcpStream>)
}

impl NetworkStream {
    pub fn new_tls_stream(stream: TlsStream<TcpStream>) -> NetworkStream {
        NetworkStream::Tls(stream)
    }
}

impl Read for NetworkStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match *self {
            NetworkStream::Tcp(ref mut s) => s.read(buf),
            NetworkStream::Tls(ref mut s) => s.read(buf),
        }
    }
}

impl Write for NetworkStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match *self {
            NetworkStream::Tcp(ref mut s) => s.write(buf),
            NetworkStream::Tls(ref mut s) => s.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match *self {
            NetworkStream::Tcp(ref mut s) => s.flush(),
            NetworkStream::Tls(ref mut s) => s.flush(),
        }
    }
}

impl AsyncRead for NetworkStream {}
impl AsyncWrite for NetworkStream {
    fn shutdown(&mut self) -> Result<Async<()>, Error> {
        unimplemented!()
    }
}