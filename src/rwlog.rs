use core::result::Result;
use core2::io::Error;
use core2::io::{Read, Write};
#[cfg(feature = "std")]
use log::LogLevel::*;

pub struct ReadWriteLog<RW> {
    inner: RW, 
}

impl<RW: Read + Write> ReadWriteLog<RW> {
    pub fn new(rw: RW) -> ReadWriteLog<RW> {
        ReadWriteLog {
            inner: rw,
        }
    }
}

impl<R: Read> Read for ReadWriteLog<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        let r = self.inner.read(buf)?;

        /*
        if log_enabled!(Debug) {
            debug!("In:");
            for x in hexdump_iter(&buf[..r]) {
                debug!("{}", x);
            }
        }
        */

        Ok(r)
    }
}

impl<RW: Write + Read> Write for ReadWriteLog<RW> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        /*
        if log_enabled!(Debug) {
            debug!("Out:");
            for x in hexdump_iter(buf) {
                debug!("{}", x);
            }
        }
        */

        self.inner.write(buf)
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.inner.flush()
    }
}

