extern crate bytes;
extern crate futures;
extern crate tokio; // 0.1.8 // 0.1.23
extern crate tokio_codec;
extern crate tokio_rustls;
extern crate webpki;

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
    let iotcore = "roots.pem";
    let capath = "/home/raviteja/Desktop/design2/rumqtt/utils/certgenerator/out/intermediate/certs/ca-chain.cert.pem";
    let host = "localhost";
    let port = 12345;
    let addr = lookup_ipv4(host, port);
    let stream = TcpStream::connect(&addr);
    let mut config = ClientConfig::new();
    config.enable_sni = false;

    let certfile = fs::File::open(&capath).expect("Cannot open CA file");
    let mut ca = BufReader::new(certfile);

    println!("@@ {:?}", ca);
    config.root_store.add_pem_file(&mut ca).unwrap();

    // let clientcert = "/home/raviteja/Desktop/design2/rumqtt/utils/certgenerator/out/client/certs/bike1.cert.pem";
    // let clientkey = "/home/raviteja/Desktop/design2/rumqtt/utils/certgenerator/out/client/private/bike1.key.pem";
    // let mut client_cert = BufReader::new(fs::File::open(clientcert).unwrap());
    // let mut client_keys = BufReader::new(fs::File::open(clientkey).unwrap());
    // let certs = pemfile::certs(&mut client_cert).unwrap();
    // let keys =  pemfile::rsa_private_keys(&mut client_keys).unwrap();

    // config.set_single_client_cert(certs, keys[0].clone());
    let config = TlsConnector::from(Arc::new(config));

    let connect = stream
        .and_then(move |stream| {
            println!("1");
            let domain = webpki::DNSNameRef::try_from_ascii_str(host).unwrap();
            println!("2");

            let c = config.connect(domain, stream);
            
            println!("3");
            c
        })
        .and_then(|stream| {
            println!("4");
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
