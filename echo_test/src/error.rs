use std::io;
use std::net::AddrParseError;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: io::Error) {
            from()
            description("io error")
            display("I/O error: {}", err)
            cause(err)
        }
        Sender {
            description("Sender error")
        }
        Line {
            description("Await PingResp error")
        }
        Other(descr: &'static str) {
            description(descr)
            display("Error {}", descr)
        }       
        Discard {
            from(&'static str)
        }
        AddressResolution(err: AddrParseError) {
            from()
        }
        ConnectionReset
    }
}
