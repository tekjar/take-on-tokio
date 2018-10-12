use bytes::{BufMut, BytesMut};
use tokio::codec::{Encoder, Decoder};
use std::{cmp, io, str, usize};

/// A simple `Codec` implementation that splits up data into lines.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LineCodec;

impl Decoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<String>, io::Error> {
        // At least 5 bytes are required for a frame: 4 byte head + one byte
        // '\n'
        if buf.len() < 5 {
            return Ok(None);
        }

        // Check to see if the frame contains a new line
        if let Some(n) = buf.iter().position(|b| *b == b'\n') {
            // remove the serialized frame from the buffer.
            let line = buf.split_to(n);

            // Also remove the '\n'
            buf.split_to(1);

            // Turn this data into a UTF string and return it in a Frame.
            return match str::from_utf8(&line.as_ref()) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "invalid string")),
            }
        }

        Ok(None)
    }
}

impl Encoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn encode(&mut self, msg: String, buf: &mut BytesMut) -> io::Result<()> {
        buf.put_slice(msg.as_bytes());
        buf.put_u8(b'\n');

        Ok(())
    }
}