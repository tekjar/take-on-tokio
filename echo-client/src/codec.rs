extern crate futures;
extern crate tokio_core;

use tokio_core::io::{Codec, EasyBuf};
use std::{io, str};

pub struct LineCodec;

impl Codec for LineCodec {
    type In = String;
    type Out = String;

    fn decode(&mut self, buf: &mut EasyBuf) -> Result<Option<String>, io::Error> {
        // Check to see if the frame contains a new line
        if let Some(n) = buf.as_ref().iter().position(|b| *b == b'\n') {
            // remove the serialized frame from the buffer.
            let line = buf.drain_to(n);

            // Also remove the '\n'
            buf.drain_to(1);

            // Turn this data into a UTF string and return it in a Frame.
            return match str::from_utf8(&line.as_ref()) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "invalid string")),
            };
        }

        Ok(None)
    }

    fn encode(&mut self, msg: String, buf: &mut Vec<u8>) -> io::Result<()> {
        buf.extend(msg.as_bytes());
        buf.push(b'\n');

        Ok(())
    }
}
