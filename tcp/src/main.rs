#[macro_use]
extern crate tokio_core;
#[macro_use]
extern crate futures;
extern crate tokio_proto;
extern crate tokio_service;

use futures::Future;
use tokio_core::reactor::Core;
use tokio_proto::TcpClient;
use tokio_service::Service;

pub mod ende;
use ende::LineProto;

pub fn main() {
    // Create the event loop that will drive this server
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let addr = "127.0.0.1:12345".parse().unwrap();

    let f1 = TcpClient::new(LineProto)
        .connect(&addr, &handle.clone())
        .and_then(|client| {
            let req = "hello".into();
            println!("req: {:?}", req);
            client.call(req)
        })
        .map(|res| println!("res: {:?}", res));

    core.run(f1).unwrap();

}
